use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, ensure, Result};
use boltz_client::{ElementsAddress, ToHex as _};
use electrum_client::GetBalanceRes;
use log::{debug, error, warn};
use lwk_wollet::bitcoin::Witness;
use lwk_wollet::elements::{secp256k1_zkp, AddressParams, Txid};
use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;
use lwk_wollet::hashes::hex::{DisplayHex, FromHex};
use lwk_wollet::hashes::{sha256, Hash as _};
use lwk_wollet::WalletTx;
use tokio::sync::Mutex;

use super::model::*;
use crate::prelude::{Direction, Swap};
use crate::swapper::Swapper;
use crate::wallet::OnchainWallet;
use crate::{
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    recover::model::{BtcScript, HistoryTxId, LBtcScript},
    utils,
};

pub(crate) struct Recoverer {
    master_blinding_key: MasterBlindingKey,
    swapper: Arc<dyn Swapper>,
    onchain_wallet: Arc<dyn OnchainWallet>,
    liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
    bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
}

impl Recoverer {
    pub(crate) fn new(
        master_blinding_key: Vec<u8>,
        swapper: Arc<dyn Swapper>,
        onchain_wallet: Arc<dyn OnchainWallet>,
        liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
        bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    ) -> Result<Self> {
        Ok(Self {
            master_blinding_key: MasterBlindingKey::from_hex(
                &master_blinding_key.to_lower_hex_string(),
            )?,
            swapper,
            onchain_wallet,
            liquid_chain_service,
            bitcoin_chain_service,
        })
    }

    fn recover_cooperative_preimages(
        &self,
        recovered_send_data: &mut HashMap<String, &mut RecoveredOnchainDataSend>,
    ) -> HashMap<String, Txid> {
        let mut failed = HashMap::new();
        for (swap_id, recovered_data) in recovered_send_data {
            let Some(claim_tx_id) = &recovered_data.claim_tx_id else {
                continue;
            };

            match self.swapper.get_submarine_preimage(swap_id) {
                Ok(preimage) => recovered_data.preimage = Some(preimage),
                Err(err) => {
                    warn!("Could not recover Send swap {swap_id} preimage cooperatively: {err:?}");
                    failed.insert(swap_id.clone(), claim_tx_id.txid);
                }
            }
        }
        failed
    }

    async fn recover_non_cooperative_preimages(
        &self,
        recovered_send_data: &mut HashMap<String, &mut RecoveredOnchainDataSend>,
        failed_cooperative: HashMap<String, Txid>,
    ) -> Result<()> {
        let claim_tx_ids: Vec<Txid> = failed_cooperative.values().cloned().collect();
        let claim_txs = self
            .liquid_chain_service
            .lock()
            .await
            .get_transactions(claim_tx_ids.as_slice())
            .await
            .map_err(|e| anyhow!("Failed to fetch claim txs from recovery: {e}"))?;

        let claim_tx_ids_len = claim_tx_ids.len();
        let claim_txs_len = claim_txs.len();
        ensure!(
            claim_tx_ids_len == claim_txs_len,
            anyhow!("Got {claim_txs_len} send claim transactions, expected {claim_tx_ids_len}")
        );

        let claim_txs_by_swap_id: HashMap<String, lwk_wollet::elements::Transaction> =
            failed_cooperative.into_keys().zip(claim_txs).collect();

        for (swap_id, claim_tx) in claim_txs_by_swap_id {
            let Some(recovered_data) = recovered_send_data.get_mut(&swap_id) else {
                continue;
            };

            match Self::get_send_swap_preimage_from_claim_tx(&swap_id, &claim_tx) {
                Ok(preimage) => recovered_data.preimage = Some(preimage),
                Err(e) => {
                    error!(
                        "Couldn't get non-cooperative swap preimage from claim tx {} for swap {swap_id}: {e}",
                        claim_tx.txid()
                    );
                    // Keep only claim tx for which there is a recovered or synced preimage
                    recovered_data.claim_tx_id = None;
                }
            }
        }

        Ok(())
    }

    async fn recover_preimages(
        &self,
        mut recovered_send_data: HashMap<String, &mut RecoveredOnchainDataSend>,
    ) -> Result<()> {
        // Recover the preimages by querying the swapper, only if there is a claim_tx_id
        let failed_cooperative = self.recover_cooperative_preimages(&mut recovered_send_data);

        // For those which failed, recover the preimages by querying onchain (non-cooperative case)
        self.recover_non_cooperative_preimages(&mut recovered_send_data, failed_cooperative)
            .await
    }

    pub(crate) fn get_send_swap_preimage_from_claim_tx(
        swap_id: &str,
        claim_tx: &lwk_wollet::elements::Transaction,
    ) -> Result<String, anyhow::Error> {
        debug!("Send Swap {swap_id} has claim tx {}", claim_tx.txid());

        let input = claim_tx
            .input
            .first()
            .ok_or_else(|| anyhow!("Found no input for claim tx"))?;

        let script_witness_bytes = input.clone().witness.script_witness;
        log::info!("Found Send Swap {swap_id} claim tx witness: {script_witness_bytes:?}");
        let script_witness = Witness::from(script_witness_bytes);

        let preimage_bytes = script_witness
            .nth(1)
            .ok_or_else(|| anyhow!("Claim tx witness has no preimage"))?;
        let preimage = sha256::Hash::from_slice(preimage_bytes)
            .map_err(|e| anyhow!("Claim tx witness has invalid preimage: {e}"))?;
        let preimage_hex = preimage.to_hex();
        debug!("Found Send Swap {swap_id} claim tx preimage: {preimage_hex}");

        Ok(preimage_hex)
    }

    /// For each swap, recovers data from chain services.
    ///
    /// The returned data include txs and the partial swap state. See [PartialSwapState::derive_partial_state].
    ///
    /// The caller is expected to merge this data with any other data available, then persist the
    /// reconstructed swap.
    ///
    /// ### Arguments
    ///
    /// - `tx_map`: all known onchain txs of this wallet at this time, essentially our own LWK cache.
    /// - `swaps`: immutable data of the swaps for which we want to recover onchain data.
    pub(crate) async fn recover_from_onchain(&self, swaps: &mut [Swap]) -> Result<()> {
        self.onchain_wallet.full_scan().await?;
        let tx_map = TxMap::from_raw_tx_map(self.onchain_wallet.transactions_by_tx_id().await?);

        let swaps_list = swaps.to_vec().try_into()?;
        let histories = self.fetch_swaps_histories(&swaps_list).await?;

        let mut recovered_send_data = self.recover_send_swap_tx_ids(&tx_map, histories.send)?;
        let recovered_send_data_without_preimage = recovered_send_data
            .iter_mut()
            .filter_map(|(swap_id, recovered_data)| {
                if let Some(Swap::Send(send_swap)) = swaps.iter().find(|s| s.id() == *swap_id) {
                    if send_swap.preimage.is_none() {
                        return Some((swap_id.clone(), recovered_data));
                    }
                }
                None
            })
            .collect::<HashMap<String, &mut RecoveredOnchainDataSend>>();
        self.recover_preimages(recovered_send_data_without_preimage)
            .await?;

        let recovered_receive_data = self.recover_receive_swap_tx_ids(
            &tx_map,
            histories.receive,
            &swaps_list.receive_swap_immutable_data_by_swap_id,
        )?;
        let recovered_chain_send_data = self.recover_send_chain_swap_tx_ids(
            &tx_map,
            histories.send_chain,
            &swaps_list.send_chain_swap_immutable_data_by_swap_id,
        )?;
        let recovered_chain_receive_data = self.recover_receive_chain_swap_tx_ids(
            &tx_map,
            histories.receive_chain,
            &swaps_list.receive_chain_swap_immutable_data_by_swap_id,
        )?;

        let bitcoin_tip = self.bitcoin_chain_service.lock().await.tip()?;
        let liquid_tip = self.liquid_chain_service.lock().await.tip().await?;

        for swap in swaps.iter_mut() {
            let swap_id = &swap.id();
            match swap {
                Swap::Send(send_swap) => {
                    let Some(recovered_data) = recovered_send_data.get_mut(swap_id) else {
                        log::warn!("Could not apply recovered data for Send swap {swap_id}: recovery data not found");
                        continue;
                    };
                    send_swap.lockup_tx_id = recovered_data
                        .lockup_tx_id
                        .clone()
                        .map(|h| h.txid.to_string());
                    send_swap.refund_tx_id = recovered_data
                        .refund_tx_id
                        .clone()
                        .map(|h| h.txid.to_string());
                    match (&send_swap.preimage, &recovered_data.preimage) {
                        // Update the preimage only if we don't have one already (e.g. from
                        // real-time sync)
                        (Some(_), _) | (None, None) => {}

                        // Keep only verified preimages
                        (None, Some(recovered_preimage)) => {
                            match utils::verify_payment_hash(recovered_preimage, &send_swap.invoice)
                            {
                                Ok(_) => send_swap.preimage = Some(recovered_preimage.clone()),
                                Err(e) => {
                                    error!("Failed to verify recovered preimage for swap {swap_id}: {e}");
                                    recovered_data.claim_tx_id = None;
                                }
                            }
                        }
                    }
                    // Set the state only AFTER the preimage and claim_tx_id have been verified
                    let timeout_block_height = send_swap.timeout_block_height as u32;
                    let is_expired = liquid_tip >= timeout_block_height;
                    if let Some(new_state) = recovered_data.derive_partial_state(is_expired) {
                        send_swap.state = new_state;
                    }
                }
                Swap::Receive(receive_swap) => {
                    let Some(recovered_data) = recovered_receive_data.get(swap_id) else {
                        log::warn!("Could not apply recovered data for Receive swap {swap_id}: recovery data not found");
                        continue;
                    };
                    let timeout_block_height = receive_swap.timeout_block_height;
                    let is_expired = liquid_tip >= timeout_block_height;
                    if let Some(new_state) = recovered_data.derive_partial_state(is_expired) {
                        receive_swap.state = new_state;
                    }
                    receive_swap.claim_tx_id = recovered_data
                        .claim_tx_id
                        .clone()
                        .map(|history_tx_id| history_tx_id.txid.to_string());
                    receive_swap.mrh_tx_id = recovered_data
                        .mrh_tx_id
                        .clone()
                        .map(|history_tx_id| history_tx_id.txid.to_string());
                    receive_swap.lockup_tx_id = recovered_data
                        .lockup_tx_id
                        .clone()
                        .map(|history_tx_id| history_tx_id.txid.to_string());
                    if let Some(mrh_amount_sat) = recovered_data.mrh_amount_sat {
                        receive_swap.payer_amount_sat = mrh_amount_sat;
                        receive_swap.receiver_amount_sat = mrh_amount_sat;
                    }
                }
                Swap::Chain(chain_swap) => match chain_swap.direction {
                    Direction::Incoming => {
                        let Some(recovered_data) = recovered_chain_receive_data.get(swap_id) else {
                            log::warn!("Could not apply recovered data for incoming Chain swap {swap_id}: recovery data not found");
                            continue;
                        };
                        if recovered_data.btc_user_lockup_amount_sat > 0 {
                            chain_swap.actual_payer_amount_sat =
                                Some(recovered_data.btc_user_lockup_amount_sat);
                        }
                        let is_expired =
                            bitcoin_tip.height as u32 >= chain_swap.timeout_block_height;
                        let expected_user_lockup_amount_sat = match chain_swap.payer_amount_sat {
                            0 => None,
                            expected => Some(expected),
                        };
                        if let Some(new_state) = recovered_data.derive_partial_state(
                            expected_user_lockup_amount_sat,
                            is_expired,
                            chain_swap.is_waiting_fee_acceptance(),
                        ) {
                            chain_swap.state = new_state;
                        }
                        chain_swap.server_lockup_tx_id = recovered_data
                            .lbtc_server_lockup_tx_id
                            .clone()
                            .map(|h| h.txid.to_string());
                        chain_swap
                            .claim_address
                            .clone_from(&recovered_data.lbtc_claim_address);
                        chain_swap.user_lockup_tx_id = recovered_data
                            .btc_user_lockup_tx_id
                            .clone()
                            .map(|h| h.txid.to_string());
                        chain_swap.claim_tx_id = recovered_data
                            .lbtc_claim_tx_id
                            .clone()
                            .map(|h| h.txid.to_string());
                        chain_swap.refund_tx_id = recovered_data
                            .btc_refund_tx_id
                            .clone()
                            .map(|h| h.txid.to_string());
                    }
                    Direction::Outgoing => {
                        let Some(recovered_data) = recovered_chain_send_data.get(swap_id) else {
                            log::warn!("Could not apply recovered data for outgoing Chain swap {swap_id}: recovery data not found");
                            continue;
                        };
                        let is_expired = liquid_tip >= chain_swap.timeout_block_height;
                        if let Some(new_state) = recovered_data.derive_partial_state(is_expired) {
                            chain_swap.state = new_state;
                        }
                        chain_swap.server_lockup_tx_id = recovered_data
                            .btc_server_lockup_tx_id
                            .clone()
                            .map(|h| h.txid.to_string());
                        chain_swap.user_lockup_tx_id = recovered_data
                            .lbtc_user_lockup_tx_id
                            .clone()
                            .map(|h| h.txid.to_string());
                        chain_swap.claim_tx_id = recovered_data
                            .btc_claim_tx_id
                            .clone()
                            .map(|h| h.txid.to_string());
                        chain_swap.refund_tx_id = recovered_data
                            .lbtc_refund_tx_id
                            .clone()
                            .map(|h| h.txid.to_string());
                    }
                },
            }
        }

        Ok(())
    }

    /// For a given [SwapList], this fetches the script histories from the chain services
    async fn fetch_swaps_histories(&self, swaps_list: &SwapsList) -> Result<SwapsHistories> {
        let swap_lbtc_scripts = swaps_list.get_swap_lbtc_scripts();
        let lbtc_script_histories = self
            .liquid_chain_service
            .lock()
            .await
            .get_scripts_history(&swap_lbtc_scripts.iter().collect::<Vec<&LBtcScript>>())
            .await?;
        let lbtc_swap_scripts_len = swap_lbtc_scripts.len();
        let lbtc_script_histories_len = lbtc_script_histories.len();
        ensure!(
                lbtc_swap_scripts_len == lbtc_script_histories_len,
                anyhow!("Got {lbtc_script_histories_len} L-BTC script histories, expected {lbtc_swap_scripts_len}")
            );
        let lbtc_script_to_history_map: HashMap<LBtcScript, Vec<HistoryTxId>> = swap_lbtc_scripts
            .into_iter()
            .zip(lbtc_script_histories.into_iter())
            .map(|(k, v)| (k, v.into_iter().map(HistoryTxId::from).collect()))
            .collect();

        let bitcoin_chain_service = self.bitcoin_chain_service.lock().await;
        let swap_btc_script_bufs = swaps_list.get_swap_btc_scripts();
        let swap_btc_scripts = swap_btc_script_bufs
            .iter()
            .map(|x| x.as_script())
            .collect::<Vec<&lwk_wollet::bitcoin::Script>>();
        let btc_script_histories = bitcoin_chain_service.get_scripts_history(&swap_btc_scripts)?;
        let btx_script_tx_ids: Vec<lwk_wollet::bitcoin::Txid> = btc_script_histories
            .iter()
            .flatten()
            .map(|h| h.txid.to_raw_hash())
            .map(lwk_wollet::bitcoin::Txid::from_raw_hash)
            .collect::<Vec<lwk_wollet::bitcoin::Txid>>();

        let btc_swap_scripts_len = swap_btc_script_bufs.len();
        let btc_script_histories_len = btc_script_histories.len();
        ensure!(
                btc_swap_scripts_len == btc_script_histories_len,
                anyhow!("Got {btc_script_histories_len} BTC script histories, expected {btc_swap_scripts_len}")
            );
        let btc_script_to_history_map: HashMap<BtcScript, Vec<HistoryTxId>> = swap_btc_script_bufs
            .clone()
            .into_iter()
            .zip(btc_script_histories.iter())
            .map(|(k, v)| (k, v.iter().map(HistoryTxId::from).collect()))
            .collect();

        let btc_script_txs = bitcoin_chain_service.get_transactions(&btx_script_tx_ids)?;
        let btc_script_balances = bitcoin_chain_service.scripts_get_balance(&swap_btc_scripts)?;
        let btc_script_to_txs_map: HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>> =
            swap_btc_script_bufs
                .clone()
                .into_iter()
                .zip(btc_script_histories.iter())
                .map(|(script, history)| {
                    let relevant_tx_ids: Vec<Txid> = history.iter().map(|h| h.txid).collect();
                    let relevant_txs: Vec<boltz_client::bitcoin::Transaction> = btc_script_txs
                        .iter()
                        .filter(|&tx| relevant_tx_ids.contains(&tx.txid().to_raw_hash().into()))
                        .cloned()
                        .collect();

                    (script, relevant_txs)
                })
                .collect();
        let btc_script_to_balance_map: HashMap<BtcScript, GetBalanceRes> = swap_btc_script_bufs
            .into_iter()
            .zip(btc_script_balances)
            .collect();

        Ok(SwapsHistories {
            send: swaps_list.send_histories_by_swap_id(&lbtc_script_to_history_map),
            receive: swaps_list.receive_histories_by_swap_id(&lbtc_script_to_history_map),
            send_chain: swaps_list.send_chain_histories_by_swap_id(
                &lbtc_script_to_history_map,
                &btc_script_to_history_map,
                &btc_script_to_txs_map,
            ),
            receive_chain: swaps_list.receive_chain_histories_by_swap_id(
                &lbtc_script_to_history_map,
                &btc_script_to_history_map,
                &btc_script_to_txs_map,
                &btc_script_to_balance_map,
            ),
        })
    }

    /// Reconstruct Send Swap tx IDs from the onchain data and the immutable data
    fn recover_send_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        send_histories_by_swap_id: HashMap<String, SendSwapHistory>,
    ) -> Result<HashMap<String, RecoveredOnchainDataSend>> {
        let mut res: HashMap<String, RecoveredOnchainDataSend> = HashMap::new();
        for (swap_id, history) in send_histories_by_swap_id {
            debug!("[Recover Send] Checking swap {swap_id}");

            // If a history tx is one of our outgoing txs, it's a lockup tx
            let lockup_tx_id = history
                .iter()
                .find(|&tx| tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid))
                .cloned();
            if lockup_tx_id.is_none() {
                error!("No lockup tx found when recovering data for Send Swap {swap_id}");
            }

            // If a history tx is one of our incoming txs, it's a refund tx
            let refund_tx_id = history
                .iter()
                .find(|&tx| tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
                .cloned();

            // A history tx that is neither a known incoming or outgoing tx is a claim tx
            let claim_tx_id = history
                .iter()
                .filter(|&tx| !tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
                .find(|&tx| !tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid))
                .cloned();

            res.insert(
                swap_id,
                RecoveredOnchainDataSend {
                    lockup_tx_id,
                    claim_tx_id,
                    refund_tx_id,
                    preimage: None,
                },
            );
        }

        Ok(res)
    }

    /// Reconstruct Receive Swap tx IDs from the onchain data and the immutable data
    fn recover_receive_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        receive_histories_by_swap_id: HashMap<String, ReceiveSwapHistory>,
        receive_swap_immutable_data_by_swap_id: &HashMap<String, ReceiveSwapImmutableData>,
    ) -> Result<HashMap<String, RecoveredOnchainDataReceive>> {
        let mut res: HashMap<String, RecoveredOnchainDataReceive> = HashMap::new();
        for (swap_id, history) in receive_histories_by_swap_id {
            debug!("[Recover Receive] Checking swap {swap_id}");

            // The MRH script history txs filtered by the swap timestamp
            let swap_timestamp = receive_swap_immutable_data_by_swap_id
                .get(&swap_id)
                .map(|imm: &ReceiveSwapImmutableData| imm.swap_timestamp)
                .ok_or_else(|| anyhow!("Swap timestamp not found for Receive Swap {swap_id}"))?;
            let mrh_txs: HashMap<Txid, WalletTx> = history
                .lbtc_mrh_script_history
                .iter()
                .filter_map(|h| tx_map.incoming_tx_map.get(&h.txid))
                .filter(|tx| tx.timestamp.map(|t| t > swap_timestamp).unwrap_or(true))
                .map(|tx| (tx.txid, tx.clone()))
                .collect();
            let mrh_tx_id = history
                .lbtc_mrh_script_history
                .iter()
                .find(|&tx| mrh_txs.contains_key::<Txid>(&tx.txid))
                .cloned();
            let mrh_amount_sat = mrh_tx_id
                .clone()
                .and_then(|h| mrh_txs.get(&h.txid))
                .map(|tx| tx.balance.values().sum::<i64>().unsigned_abs());

            let (lockup_tx_id, claim_tx_id) = match history.lbtc_claim_script_history.len() {
                // Only lockup tx available
                1 => (Some(history.lbtc_claim_script_history[0].clone()), None),

                2 => {
                    let first = history.lbtc_claim_script_history[0].clone();
                    let second = history.lbtc_claim_script_history[1].clone();

                    if tx_map.incoming_tx_map.contains_key::<Txid>(&first.txid) {
                        // If the first tx is a known incoming tx, it's the claim tx and the second is the lockup
                        (Some(second), Some(first))
                    } else if tx_map.incoming_tx_map.contains_key::<Txid>(&second.txid) {
                        // If the second tx is a known incoming tx, it's the claim tx and the first is the lockup
                        (Some(first), Some(second))
                    } else {
                        // If none of the 2 txs is the claim tx, then the txs are lockup and swapper refund
                        // If so, we expect them to be confirmed at different heights
                        let first_conf_height = first.height;
                        let second_conf_height = second.height;
                        match (first.confirmed(), second.confirmed()) {
                            // If they're both confirmed, the one with the lowest confirmation height is the lockup
                            (true, true) => match first_conf_height < second_conf_height {
                                true => (Some(first), None),
                                false => (Some(second), None),
                            },

                            // If only one tx is confirmed, then that is the lockup
                            (true, false) => (Some(first), None),
                            (false, true) => (Some(second), None),

                            // If neither is confirmed, this is an edge-case
                            (false, false) => {
                                warn!("Found unconfirmed lockup and refund txs while recovering data for Receive Swap {swap_id}");
                                (None, None)
                            }
                        }
                    }
                }
                n => {
                    warn!("Script history with length {n} found while recovering data for Receive Swap {swap_id}");
                    (None, None)
                }
            };

            // Take only the lockup_tx_id and claim_tx_id if either are set,
            // otherwise take the mrh_tx_id and mrh_amount_sat
            let recovered_onchain_data = match (lockup_tx_id.as_ref(), claim_tx_id.as_ref()) {
                (Some(_), None) | (Some(_), Some(_)) => RecoveredOnchainDataReceive {
                    lockup_tx_id,
                    claim_tx_id,
                    mrh_tx_id: None,
                    mrh_amount_sat: None,
                },
                _ => RecoveredOnchainDataReceive {
                    lockup_tx_id: None,
                    claim_tx_id: None,
                    mrh_tx_id,
                    mrh_amount_sat,
                },
            };

            res.insert(swap_id, recovered_onchain_data);
        }

        Ok(res)
    }

    /// Reconstruct Chain Send Swap tx IDs from the onchain data and the immutable data
    fn recover_send_chain_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        chain_send_histories_by_swap_id: HashMap<String, SendChainSwapHistory>,
        send_chain_swap_immutable_data_by_swap_id: &HashMap<String, SendChainSwapImmutableData>,
    ) -> Result<HashMap<String, RecoveredOnchainDataChainSend>> {
        let mut res: HashMap<String, RecoveredOnchainDataChainSend> = HashMap::new();
        for (swap_id, history) in chain_send_histories_by_swap_id {
            debug!("[Recover Chain Send] Checking swap {swap_id}");

            // If a history tx is one of our outgoing txs, it's a lockup tx
            let lbtc_user_lockup_tx_id = history
                .lbtc_lockup_script_history
                .iter()
                .find(|&tx| tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid))
                .cloned();
            if lbtc_user_lockup_tx_id.is_none() {
                error!("No lockup tx found when recovering data for Chain Send Swap {swap_id}");
            }

            // If a history tx is one of our incoming txs, it's a refund tx
            let lbtc_refund_tx_id = history
                .lbtc_lockup_script_history
                .iter()
                .find(|&tx| tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
                .cloned();

            let (btc_server_lockup_tx_id, btc_claim_tx_id) = match history
                .btc_claim_script_history
                .len()
            {
                // Only lockup tx available
                1 => (Some(history.btc_claim_script_history[0].clone()), None),

                2 => {
                    let first_tx = history.btc_claim_script_txs[0].clone();
                    let first_tx_id = history.btc_claim_script_history[0].clone();
                    let second_tx_id = history.btc_claim_script_history[1].clone();

                    let btc_lockup_script = send_chain_swap_immutable_data_by_swap_id
                        .get(&swap_id)
                        .map(|imm| imm.claim_script.clone())
                        .ok_or_else(|| {
                            anyhow!("BTC claim script not found for Onchain Send Swap {swap_id}")
                        })?;

                    // We check the full tx, to determine if this is the BTC lockup tx
                    let is_first_tx_lockup_tx = first_tx
                        .output
                        .iter()
                        .any(|out| matches!(&out.script_pubkey, x if x == &btc_lockup_script));

                    match is_first_tx_lockup_tx {
                        true => (Some(first_tx_id), Some(second_tx_id)),
                        false => (Some(second_tx_id), Some(first_tx_id)),
                    }
                }
                n => {
                    warn!("BTC script history with length {n} found while recovering data for Chain Send Swap {swap_id}");
                    (None, None)
                }
            };

            res.insert(
                swap_id,
                RecoveredOnchainDataChainSend {
                    lbtc_user_lockup_tx_id,
                    lbtc_refund_tx_id,
                    btc_server_lockup_tx_id,
                    btc_claim_tx_id,
                },
            );
        }

        Ok(res)
    }

    /// Reconstruct Chain Receive Swap tx IDs from the onchain data and the immutable data data
    fn recover_receive_chain_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        chain_receive_histories_by_swap_id: HashMap<String, ReceiveChainSwapHistory>,
        receive_chain_swap_immutable_data_by_swap_id: &HashMap<
            String,
            ReceiveChainSwapImmutableData,
        >,
    ) -> Result<HashMap<String, RecoveredOnchainDataChainReceive>> {
        let secp = secp256k1_zkp::Secp256k1::new();

        let mut res: HashMap<String, RecoveredOnchainDataChainReceive> = HashMap::new();
        for (swap_id, history) in chain_receive_histories_by_swap_id {
            debug!("[Recover Chain Receive] Checking swap {swap_id}");

            let (lbtc_server_lockup_tx_id, lbtc_claim_tx_id, lbtc_claim_address) = match history
                .lbtc_claim_script_history
                .len()
            {
                // Only lockup tx available
                1 => (
                    Some(history.lbtc_claim_script_history[0].clone()),
                    None,
                    None,
                ),

                2 => {
                    let first = &history.lbtc_claim_script_history[0];
                    let second = &history.lbtc_claim_script_history[1];

                    // If a history tx is a known incoming tx, it's the claim tx
                    let (lockup_tx_id, claim_tx_id) =
                        match tx_map.incoming_tx_map.contains_key::<Txid>(&first.txid) {
                            true => (second, first),
                            false => (first, second),
                        };

                    // Get the claim address from the claim tx output
                    let claim_address = tx_map
                        .incoming_tx_map
                        .get(&claim_tx_id.txid)
                        .and_then(|tx| {
                            tx.outputs
                                .iter()
                                .find(|output| output.is_some())
                                .and_then(|output| output.clone().map(|o| o.script_pubkey))
                        })
                        .and_then(|script| {
                            ElementsAddress::from_script(
                                &script,
                                Some(self.master_blinding_key.blinding_key(&secp, &script)),
                                &AddressParams::LIQUID,
                            )
                            .map(|addr| addr.to_string())
                        });

                    (
                        Some(lockup_tx_id.clone()),
                        Some(claim_tx_id.clone()),
                        claim_address,
                    )
                }
                n => {
                    warn!("L-BTC script history with length {n} found while recovering data for Chain Receive Swap {swap_id}");
                    (None, None, None)
                }
            };

            // Get the current confirmed amount available for the lockup script
            let btc_user_lockup_address_balance_sat = history
                .btc_lockup_script_balance
                .map(|balance| balance.confirmed)
                .unwrap_or_default();

            // The btc_lockup_script_history can contain 3 kinds of txs, of which only 2 are expected:
            // - 1) btc_user_lockup_tx_id (initial BTC funds sent by the sender)
            // - 2A) btc_server_claim_tx_id (the swapper tx that claims the BTC funds, in success case)
            // - 2B) btc_refund_tx_id (refund tx we initiate, in failure case or with lockup address reuse)
            // The exact type of the second is found in the next step.
            let btc_lockup_script = receive_chain_swap_immutable_data_by_swap_id
                .get(&swap_id)
                .map(|imm| imm.lockup_script.clone())
                .ok_or_else(|| {
                    anyhow!("BTC lockup script not found for Onchain Receive Swap {swap_id}")
                })?;
            let (btc_lockup_incoming_txs, btc_lockup_outgoing_txs): (Vec<_>, Vec<_>) =
                history.btc_lockup_script_txs.iter().partition(|tx| {
                    tx.output
                        .iter()
                        .any(|out| matches!(&out.script_pubkey, x if x == &btc_lockup_script))
                });
            // Get the user lockup tx from the first incoming txs
            let btc_user_lockup_tx_id = btc_lockup_incoming_txs
                .first()
                .and_then(|tx| {
                    history
                        .btc_lockup_script_history
                        .iter()
                        .find(|h| h.txid.as_raw_hash() == tx.txid().as_raw_hash())
                })
                .cloned();
            let btc_user_lockup_amount_sat = btc_lockup_incoming_txs
                .first()
                .and_then(|tx| {
                    tx.output
                        .iter()
                        .find(|out| out.script_pubkey == btc_lockup_script)
                        .map(|out| out.value)
                })
                .unwrap_or_default()
                .to_sat();
            let btc_outgoing_tx_ids: Vec<HistoryTxId> = btc_lockup_outgoing_txs
                .iter()
                .filter_map(|tx| {
                    history
                        .btc_lockup_script_history
                        .iter()
                        .find(|h| h.txid.as_raw_hash() == tx.txid().as_raw_hash())
                })
                .cloned()
                .collect();
            // Get the last unconfirmed tx from the outgoing txs, else take the last outgoing tx
            let btc_last_outgoing_tx_id = btc_outgoing_tx_ids
                .iter()
                .rev()
                .find(|h| h.height == 0)
                .or(btc_outgoing_tx_ids.last())
                .cloned();

            // The first outgoing BTC tx is only a refund in case we didn't claim.
            // If we claimed, then the first tx was the swapper BTC claim tx.
            // If there are more than 1 outgoing txs then this is a refund from lockup address re-use,
            // so take the last unconfirmed tx else take the last confirmed tx.
            let btc_refund_tx_id = match lbtc_claim_tx_id.is_some() {
                true => match btc_lockup_outgoing_txs.len() > 1 {
                    true => btc_last_outgoing_tx_id,
                    false => None,
                },
                false => btc_last_outgoing_tx_id,
            };

            res.insert(
                swap_id,
                RecoveredOnchainDataChainReceive {
                    lbtc_server_lockup_tx_id,
                    lbtc_claim_tx_id,
                    lbtc_claim_address,
                    btc_user_lockup_tx_id,
                    btc_user_lockup_address_balance_sat,
                    btc_user_lockup_amount_sat,
                    btc_refund_tx_id,
                },
            );
        }

        Ok(res)
    }
}
