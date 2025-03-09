use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, ensure, Result};
use electrum_client::GetBalanceRes;
use log::{info, warn};
use lwk_wollet::elements::Txid;
use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;
use lwk_wollet::hashes::hex::{DisplayHex, FromHex};
use lwk_wollet::WalletTx;

use super::handlers::{
    ChainReceiveSwapHandler, ChainSendSwapHandler, ReceiveSwapHandler, SendSwapHandler,
};
use super::model::*;

use crate::model::Direction;
use crate::prelude::Swap;
use crate::sdk::NETWORK_PROPAGATION_GRACE_PERIOD;
use crate::swapper::Swapper;
use crate::wallet::OnchainWallet;
use crate::{
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    recover::model::{BtcScript, HistoryTxId, LBtcScript},
    utils,
};

const LIQUID_TIP_LEEWAY: u32 = 3;

pub(crate) struct Recoverer {
    master_blinding_key: MasterBlindingKey,
    swapper: Arc<dyn Swapper>,
    onchain_wallet: Arc<dyn OnchainWallet>,
    liquid_chain_service: Arc<dyn LiquidChainService>,
    bitcoin_chain_service: Arc<dyn BitcoinChainService>,
}

impl Recoverer {
    pub(crate) fn new(
        master_blinding_key: Vec<u8>,
        swapper: Arc<dyn Swapper>,
        onchain_wallet: Arc<dyn OnchainWallet>,
        liquid_chain_service: Arc<dyn LiquidChainService>,
        bitcoin_chain_service: Arc<dyn BitcoinChainService>,
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

    /// For each swap, recovers data from chain services.
    ///
    /// The returned data include txs and the partial swap state. See [PartialSwapState::derive_partial_state].
    ///
    /// The caller is expected to merge this data with any other data available, then persist the
    /// reconstructed swap.
    ///
    /// ### Arguments
    ///
    /// - `swaps`: The swaps for which we want to recover onchain data.
    ///
    /// Returns the raw onchain tx map used for recovery.
    pub(crate) async fn recover_from_onchain(
        &self,
        swaps: &mut [Swap],
    ) -> Result<HashMap<Txid, WalletTx>> {
        let wallet_tip = self.onchain_wallet.tip().await;
        let liquid_tip = self.liquid_chain_service.tip().await?;
        if wallet_tip.abs_diff(liquid_tip) > LIQUID_TIP_LEEWAY {
            log::debug!("Wallet and liquid chain service tips are too far apart, starting manual wallet sync");
            self.onchain_wallet.full_scan().await?;
        }

        let recovery_started_at = utils::now();

        // Fetch raw transaction map and convert to our internal format
        let raw_tx_map = self.onchain_wallet.transactions_by_tx_id().await?;

        // Fetch chain tips for expiration checks
        let bitcoin_tip = self.bitcoin_chain_service.tip()?;
        let liquid_tip = self.liquid_chain_service.tip().await?;

        // Convert swaps to SwapsList and fetch history data
        let swaps_list = swaps.to_vec().try_into()?;
        let recovery_context = self
            .create_recovery_context(
                &swaps_list,
                TxMap::from_raw_tx_map(raw_tx_map.clone()),
                liquid_tip,
                bitcoin_tip.height as u32,
                self.master_blinding_key,
            )
            .await?;

        // Apply recovered data to the swaps
        for swap in swaps.iter_mut() {
            let swap_id = &swap.id();
            let is_local_within_grace_period = swap.is_local()
                && recovery_started_at.saturating_sub(swap.last_updated_at())
                    < NETWORK_PROPAGATION_GRACE_PERIOD.as_secs() as u32;
            let res = match swap {
                Swap::Send(s) => {
                    SendSwapHandler::recover_swap(
                        s,
                        &recovery_context,
                        is_local_within_grace_period,
                    )
                    .await
                }

                Swap::Receive(s) => {
                    ReceiveSwapHandler::recover_swap(
                        s,
                        &recovery_context,
                        is_local_within_grace_period,
                    )
                    .await
                }
                Swap::Chain(s) => match s.direction {
                    Direction::Outgoing => {
                        ChainSendSwapHandler::recover_swap(
                            s,
                            &recovery_context,
                            is_local_within_grace_period,
                        )
                        .await
                    }
                    Direction::Incoming => {
                        ChainReceiveSwapHandler::recover_swap(
                            s,
                            &recovery_context,
                            is_local_within_grace_period,
                        )
                        .await
                    }
                },
            };
            if let Err(err) = res {
                warn!("Error recovering data for swap {swap_id}: {err}");
            }
        }

        Ok(raw_tx_map)
    }

    /// For a given [SwapList], this fetches the script histories from the chain services
    async fn create_recovery_context(
        &self,
        swaps_list: &SwapsList,
        tx_map: TxMap,
        liquid_tip_height: u32,
        bitcoin_tip_height: u32,
        master_blinding_key: MasterBlindingKey,
    ) -> Result<RecoveryContext> {
        // Fetch history data for each lbtc swap script
        let lbtc_script_to_history_map = self
            .create_lbtc_history_map(swaps_list.get_swap_lbtc_scripts())
            .await?;

        // Fetch history data for each btc swap script
        let (btc_script_to_history_map, btc_script_to_txs_map, btc_script_to_balance_map) = self
            .create_btc_history_maps(swaps_list.get_swap_btc_scripts())
            .await?;

        Ok(RecoveryContext {
            lbtc_script_to_history_map,
            btc_script_to_history_map,
            btc_script_to_txs_map,
            btc_script_to_balance_map,
            tx_map,
            liquid_tip_height,
            bitcoin_tip_height,
            master_blinding_key,
            liquid_chain_service: self.liquid_chain_service.clone(),
            swapper: self.swapper.clone(),
        })
    }

    async fn create_lbtc_history_map(
        &self,
        swap_lbtc_scripts: Vec<LBtcScript>,
    ) -> Result<HashMap<LBtcScript, Vec<HistoryTxId>>> {
        let t0 = std::time::Instant::now();
        let lbtc_script_histories = self
            .liquid_chain_service
            .get_scripts_history(&swap_lbtc_scripts.to_vec())
            .await?;
        info!(
            "Recoverer executed liquid get_scripts_history for {} scripts in {} milliseconds",
            swap_lbtc_scripts.len(),
            t0.elapsed().as_millis()
        );

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

        Ok(lbtc_script_to_history_map)
    }

    async fn create_btc_history_maps(
        &self,
        swap_btc_script_bufs: Vec<BtcScript>,
    ) -> Result<(
        HashMap<BtcScript, Vec<HistoryTxId>>,
        HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>>,
        HashMap<BtcScript, GetBalanceRes>,
    )> {
        let swap_btc_scripts = swap_btc_script_bufs
            .iter()
            .map(|x| x.as_script())
            .collect::<Vec<&lwk_wollet::bitcoin::Script>>();

        let t0 = std::time::Instant::now();
        let btc_script_histories = self
            .bitcoin_chain_service
            .get_scripts_history(&swap_btc_scripts)?;

        info!(
            "Recoverer executed bitcoin get_scripts_history for {} scripts in {} milliseconds",
            swap_btc_scripts.len(),
            t0.elapsed().as_millis()
        );

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

        let t0 = std::time::Instant::now();
        let btc_script_txs = self
            .bitcoin_chain_service
            .get_transactions(&btx_script_tx_ids)?;
        info!(
            "Recoverer executed bitcoin get_transactions for {} transactions in {} milliseconds",
            btx_script_tx_ids.len(),
            t0.elapsed().as_millis()
        );

        let t0 = std::time::Instant::now();
        let btc_script_balances = self
            .bitcoin_chain_service
            .scripts_get_balance(&swap_btc_scripts)?;
        info!(
            "Recoverer executed bitcoin scripts_get_balance for {} scripts in {} milliseconds",
            swap_btc_scripts.len(),
            t0.elapsed().as_millis()
        );

        let btc_script_to_txs_map: HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>> =
            swap_btc_script_bufs
                .clone()
                .into_iter()
                .zip(btc_script_histories.iter())
                .map(|(script, history)| {
                    let relevant_tx_ids: Vec<Txid> = history.iter().map(|h| h.txid).collect();
                    let relevant_txs: Vec<boltz_client::bitcoin::Transaction> = btc_script_txs
                        .iter()
                        .filter(|&tx| {
                            relevant_tx_ids.contains(&tx.compute_txid().to_raw_hash().into())
                        })
                        .cloned()
                        .collect();

                    (script, relevant_txs)
                })
                .collect();

        let btc_script_to_balance_map: HashMap<BtcScript, GetBalanceRes> = swap_btc_script_bufs
            .into_iter()
            .zip(btc_script_balances)
            .collect();

        Ok((
            btc_script_to_history_map,
            btc_script_to_txs_map,
            btc_script_to_balance_map,
        ))
    }
}
