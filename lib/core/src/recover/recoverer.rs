use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, ensure, Result};
use boltz_client::ElementsAddress;
use log::{debug, error, warn};
use lwk_wollet::elements::{secp256k1_zkp, AddressParams, Txid};
use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;
use lwk_wollet::hashes::hex::{DisplayHex, FromHex};
use tokio::sync::Mutex;

use crate::wallet::OnchainWallet;
use crate::{
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    recover::model::{BtcScript, HistoryTxId, LBtcScript},
};

use super::model::*;

pub(crate) struct Recoverer {
    master_blinding_key: MasterBlindingKey,
    onchain_wallet: Arc<dyn OnchainWallet>,
    liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
    bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
}

impl Recoverer {
    pub(crate) fn new(
        master_blinding_key: Vec<u8>,
        onchain_wallet: Arc<dyn OnchainWallet>,
        liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
        bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    ) -> Result<Self> {
        Ok(Self {
            master_blinding_key: MasterBlindingKey::from_hex(
                &master_blinding_key.to_lower_hex_string(),
            )?,
            onchain_wallet,
            liquid_chain_service,
            bitcoin_chain_service,
        })
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
    /// - `partial_sync`: recovers related scripts like MRH when true, otherwise recovers all scripts.
    pub(crate) async fn recover_from_onchain(
        &self,
        swaps: SwapsList,
        partial_sync: bool,
    ) -> Result<RecoveredOnchainData> {
        self.onchain_wallet.full_scan().await?;
        let tx_map = TxMap::from_raw_tx_map(self.onchain_wallet.transactions_by_tx_id().await?);

        let histories = self.fetch_swaps_histories(&swaps, partial_sync).await?;

        let recovered_send_data = self.recover_send_swap_tx_ids(&tx_map, histories.send)?;
        let recovered_receive_data =
            self.recover_receive_swap_tx_ids(&tx_map, histories.receive)?;
        let recovered_chain_send_data = self.recover_send_chain_swap_tx_ids(
            &tx_map,
            histories.send_chain,
            &swaps.send_chain_swap_immutable_data_by_swap_id,
        )?;
        let recovered_chain_receive_data = self.recover_receive_chain_swap_tx_ids(
            &tx_map,
            histories.receive_chain,
            &swaps.receive_chain_swap_immutable_data_by_swap_id,
        )?;

        Ok(RecoveredOnchainData {
            send: recovered_send_data,
            receive: recovered_receive_data,
            chain_send: recovered_chain_send_data,
            chain_receive: recovered_chain_receive_data,
        })
    }

    /// For a given [SwapList], this fetches the script histories from the chain services
    async fn fetch_swaps_histories(
        &self,
        swaps_list: &SwapsList,
        partial_sync: bool,
    ) -> Result<SwapsHistories> {
        let swap_lbtc_scripts = swaps_list.get_swap_lbtc_scripts(partial_sync);

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

        let swap_btc_scripts = swaps_list.get_swap_btc_scripts(partial_sync);
        let btc_script_histories = self
            .bitcoin_chain_service
            .lock()
            .await
            .get_scripts_history(
                &swap_btc_scripts
                    .iter()
                    .map(|x| x.as_script())
                    .collect::<Vec<&lwk_wollet::bitcoin::Script>>(),
            )?;
        let btx_script_tx_ids: Vec<lwk_wollet::bitcoin::Txid> = btc_script_histories
            .iter()
            .flatten()
            .map(|h| h.txid.to_raw_hash())
            .map(lwk_wollet::bitcoin::Txid::from_raw_hash)
            .collect::<Vec<lwk_wollet::bitcoin::Txid>>();

        let btc_swap_scripts_len = swap_btc_scripts.len();
        let btc_script_histories_len = btc_script_histories.len();
        ensure!(
                btc_swap_scripts_len == btc_script_histories_len,
                anyhow!("Got {btc_script_histories_len} BTC script histories, expected {btc_swap_scripts_len}")
            );
        let btc_script_to_history_map: HashMap<BtcScript, Vec<HistoryTxId>> = swap_btc_scripts
            .clone()
            .into_iter()
            .zip(btc_script_histories.iter())
            .map(|(k, v)| (k, v.iter().map(HistoryTxId::from).collect()))
            .collect();

        let btc_script_txs = self
            .bitcoin_chain_service
            .lock()
            .await
            .get_transactions(&btx_script_tx_ids)?;
        let btc_script_to_txs_map: HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>> =
            swap_btc_scripts
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
    ) -> Result<HashMap<String, RecoveredOnchainDataReceive>> {
        let mut res: HashMap<String, RecoveredOnchainDataReceive> = HashMap::new();
        for (swap_id, history) in receive_histories_by_swap_id {
            debug!("[Recover Receive] Checking swap {swap_id}");

            let mrh_tx_id = history
                .lbtc_mrh_script_history
                .iter()
                .find(|&tx| tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
                .cloned();
            let mrh_amount_sat = mrh_tx_id
                .clone()
                .and_then(|h| tx_map.incoming_tx_map.get(&h.txid))
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

            // The btc_lockup_script_history can contain 3 kinds of txs, of which only 2 are expected:
            // - 1) btc_user_lockup_tx_id (initial BTC funds sent by the sender)
            // - 2A) btc_server_claim_tx_id (the swapper tx that claims the BTC funds, in Success case)
            // - 2B) btc_refund_tx_id (refund tx we initiate, in Failure case)
            // The exact type of the second is found in the next step.
            let (btc_user_lockup_tx_id, btc_second_tx_id) = match history
                .btc_lockup_script_history
                .len()
            {
                // Only lockup tx available
                1 => (Some(history.btc_lockup_script_history[0].clone()), None),

                // Both txs available (lockup + claim, or lockup + refund)
                // Any tx above the first two, we ignore, as that is address re-use which is not supported
                n if n >= 2 => {
                    let first_tx = history.btc_lockup_script_txs[0].clone();
                    let first_tx_id = history.btc_lockup_script_history[0].clone();
                    let second_tx_id = history.btc_lockup_script_history[1].clone();

                    let btc_lockup_script = receive_chain_swap_immutable_data_by_swap_id
                        .get(&swap_id)
                        .map(|imm| imm.lockup_script.clone())
                        .ok_or_else(|| {
                            anyhow!(
                                "BTC lockup script not found for Onchain Receive Swap {swap_id}"
                            )
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
                    warn!("BTC script history with length {n} found while recovering data for Chain Receive Swap {swap_id}");
                    (None, None)
                }
            };

            // The second BTC tx is only a refund in case we didn't claim.
            // If we claimed, then the second BTC tx was an internal BTC server claim tx, which we're not tracking.
            let btc_refund_tx_id = match lbtc_claim_tx_id.is_some() {
                true => None,
                false => btc_second_tx_id,
            };

            res.insert(
                swap_id,
                RecoveredOnchainDataChainReceive {
                    lbtc_server_lockup_tx_id,
                    lbtc_claim_tx_id,
                    lbtc_claim_address,
                    btc_user_lockup_tx_id,
                    btc_refund_tx_id,
                },
            );
        }

        Ok(res)
    }
}
