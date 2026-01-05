use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, ensure, Result};
use log::{debug, error, info, warn};
use lwk_wollet::elements::AssetId;

use super::handlers::{
    ChainReceiveSwapHandler, ChainSendSwapHandler, ReceiveSwapHandler, SendSwapHandler,
};
use super::model::*;
use crate::prelude::*;

use elements::Txid;
use lwk_wollet::{
    elements_miniscript::slip77::MasterBlindingKey,
    hashes::hex::{DisplayHex, FromHex},
    WalletTx,
};

use crate::sdk::NETWORK_PROPAGATION_GRACE_PERIOD;
use crate::swapper::Swapper;
use crate::wallet::OnchainWallet;
use crate::{
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    model::{BtcScript, Direction, LBtcScript},
    persist::Persister,
    prelude::Swap,
    utils,
};

const LIQUID_TIP_LEEWAY: u32 = 3;

pub struct Recoverer {
    master_blinding_key: MasterBlindingKey,
    lbtc_asset_id: AssetId,
    swapper: Arc<dyn Swapper>,
    onchain_wallet: Arc<dyn OnchainWallet>,
    liquid_chain_service: Arc<dyn LiquidChainService>,
    bitcoin_chain_service: Arc<dyn BitcoinChainService>,
    persister: std::sync::Arc<Persister>,
}

impl Recoverer {
    pub(crate) fn new(
        master_blinding_key: Vec<u8>,
        lbtc_asset_id: AssetId,
        swapper: Arc<dyn Swapper>,
        onchain_wallet: Arc<dyn OnchainWallet>,
        liquid_chain_service: Arc<dyn LiquidChainService>,
        bitcoin_chain_service: Arc<dyn BitcoinChainService>,
        persister: std::sync::Arc<Persister>,
    ) -> Result<Self> {
        Ok(Self {
            master_blinding_key: MasterBlindingKey::from_hex(
                &master_blinding_key.to_lower_hex_string(),
            )?,
            lbtc_asset_id,
            swapper,
            onchain_wallet,
            liquid_chain_service,
            bitcoin_chain_service,
            persister,
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
    /// - `maybe_chain_tips`: Optional chain tips. If not provided, the latest tips will be fetched.
    ///
    /// Returns the raw onchain tx map used for recovery.
    pub(crate) async fn recover_from_onchain(
        &self,
        swaps: &mut [Swap],
        maybe_chain_tips: Option<ChainTips>,
    ) -> Result<HashMap<Txid, WalletTx>> {
        debug!("Recoverer::recover_from_onchain: start");

        let chain_tips = match maybe_chain_tips {
            None => ChainTips {
                liquid_tip: self.liquid_chain_service.tip().await?,
                bitcoin_tip: {
                    if swaps.iter().any(|s| matches!(s, Swap::Chain(_))) {
                        self.bitcoin_chain_service.tip().await.ok()
                    } else {
                        info!("No chain swaps to recover, skipping bitcoin tip fetch");
                        None
                    }
                },
            },
            Some(tips) => tips,
        };
        debug!(
            "Recoverer::recover_from_onchain: got chain tips Liquid {} Bitcoin {:?}",
            chain_tips.liquid_tip, chain_tips.bitcoin_tip
        );

        self.sync_wallet_if_needed(chain_tips.liquid_tip).await?;

        let recovery_started_at = utils::now();

        // Create wallet transactions map
        let raw_tx_map = self.onchain_wallet.transactions_by_tx_id().await?;
        debug!(
            "Recoverer::recover_from_onchain: got {} raw txs from LWK wallet",
            raw_tx_map.len()
        );

        // Convert swaps to SwapsList and fetch history data
        let swaps_list = swaps.to_vec().try_into()?;
        let (swap_recovery_context, chain_swap_recovery_context) = self
            .create_recovery_contexts(
                &swaps_list,
                TxMap::from_raw_tx_map(raw_tx_map.clone()),
                chain_tips.liquid_tip,
                chain_tips.bitcoin_tip,
                self.master_blinding_key,
            )
            .await?;

        // Apply recovered data to the swaps
        for swap in swaps.iter_mut() {
            let swap_id = &swap.id();
            let is_within_grace_period = recovery_started_at.saturating_sub(swap.last_updated_at())
                < NETWORK_PROPAGATION_GRACE_PERIOD.as_secs() as u32;
            let res = match swap {
                Swap::Send(s) => {
                    SendSwapHandler::recover_swap(s, &swap_recovery_context, is_within_grace_period)
                        .await
                }

                Swap::Receive(s) => {
                    ReceiveSwapHandler::recover_swap(
                        s,
                        &swap_recovery_context,
                        is_within_grace_period,
                    )
                    .await
                }
                Swap::Chain(s) => {
                    let Some(recovery_context) = &chain_swap_recovery_context else {
                        error!("Skipping recovery for chain swap {swap_id} because chain swaps recovery context is not available");
                        continue;
                    };
                    match s.direction {
                        Direction::Outgoing => {
                            ChainSendSwapHandler::recover_swap(
                                s,
                                recovery_context,
                                is_within_grace_period,
                            )
                            .await
                        }
                        Direction::Incoming => {
                            ChainReceiveSwapHandler::recover_swap(
                                s,
                                recovery_context,
                                is_within_grace_period,
                            )
                            .await
                        }
                    }
                }
            };
            if let Err(err) = res {
                warn!("Error recovering data for swap {swap_id}: {err}");
            }

            debug!("Recoverer::recover_from_onchain: successfully recovered swap {swap_id}");
        }

        debug!("Recoverer::recover_from_onchain completed");
        Ok(raw_tx_map)
    }

    async fn sync_wallet_if_needed(&self, liquid_tip: u32) -> Result<()> {
        let wallet_tip = self.onchain_wallet.tip().await;
        let tip_difference = wallet_tip.abs_diff(liquid_tip);
        let tips_too_far_apart = tip_difference > LIQUID_TIP_LEEWAY;

        let last_used_derivation_index = self
            .persister
            .get_last_derivation_index()?
            .unwrap_or_default();
        let last_scanned_derivation_index = self
            .persister
            .get_last_scanned_derivation_index()?
            .unwrap_or_default();
        let has_unscanned_derivation_indices =
            last_used_derivation_index > last_scanned_derivation_index;

        if tips_too_far_apart || has_unscanned_derivation_indices {
            if tips_too_far_apart {
                debug!(
                    "Starting manual wallet sync due to tips difference {tip_difference} exceeding leeway {LIQUID_TIP_LEEWAY} (wallet: {wallet_tip}, liquid: {liquid_tip})"
                );
            }
            if has_unscanned_derivation_indices {
                debug!(
                    "Starting manual wallet sync due to unscanned derivation indices {last_scanned_derivation_index} to {last_used_derivation_index}"
                );
            }
            self.onchain_wallet.full_scan().await?;
        }
        Ok(())
    }

    /// For a given [SwapList], this fetches the script histories from the chain services
    async fn create_recovery_contexts(
        &self,
        swaps_list: &SwapsList,
        tx_map: TxMap,
        liquid_tip_height: u32,
        bitcoin_tip_height: Option<u32>,
        master_blinding_key: MasterBlindingKey,
    ) -> Result<(
        ReceiveOrSendSwapRecoveryContext,
        Option<ChainSwapRecoveryContext>,
    )> {
        debug!("Recoverer::create_recovery_contexts: start");
        // Fetch history data for each lbtc swap script
        let lbtc_script_to_history_map = self
            .fetch_lbtc_history_map(swaps_list.get_swap_lbtc_scripts())
            .await?;

        let chain_swap_recovery_context = if let Some(bitcoin_tip_height) = bitcoin_tip_height {
            // Fetch history data for each btc swap script
            let (btc_script_to_history_map, btc_script_to_txs_map, btc_script_to_balance_map) =
                self.fetch_btc_script_maps(swaps_list.get_swap_btc_scripts())
                    .await?;

            Some(ChainSwapRecoveryContext {
                lbtc_script_to_history_map: lbtc_script_to_history_map.clone(),
                btc_script_to_history_map,
                btc_script_to_txs_map,
                btc_script_to_balance_map,
                tx_map: tx_map.clone(),
                liquid_tip_height,
                bitcoin_tip_height,
                master_blinding_key,
            })
        } else {
            if swaps_list
                .swaps_by_id
                .values()
                .any(|s| matches!(s, Swap::Chain(_)))
            {
                warn!("Not creating chain swaps recovery context because bitcoin tip is not available");
            }
            None
        };

        debug!("Recoverer::create_recovery_contexts: end");
        Ok((
            ReceiveOrSendSwapRecoveryContext {
                lbtc_script_to_history_map,
                tx_map,
                liquid_tip_height,
                liquid_chain_service: self.liquid_chain_service.clone(),
                swapper: self.swapper.clone(),
                lbtc_asset_id: self.lbtc_asset_id,
            },
            chain_swap_recovery_context,
        ))
    }

    async fn fetch_lbtc_history_map(
        &self,
        swap_lbtc_scripts: Vec<LBtcScript>,
    ) -> Result<HashMap<LBtcScript, Vec<LBtcHistory>>> {
        debug!("Recoverer::fetch_lbtc_history_map: start");

        let t0 = web_time::Instant::now();
        let lbtc_script_histories = self
            .liquid_chain_service
            .get_scripts_history_with_retry(&swap_lbtc_scripts, 3)
            .await?;
        info!(
            "Recoverer::fetch_lbtc_history_map: executed liquid get_scripts_history for {} scripts in {} milliseconds",
            swap_lbtc_scripts.len(),
            t0.elapsed().as_millis()
        );

        let lbtc_swap_scripts_len = swap_lbtc_scripts.len();
        let lbtc_script_histories_len = lbtc_script_histories.len();
        ensure!(
                lbtc_swap_scripts_len == lbtc_script_histories_len,
                anyhow!("Got {lbtc_script_histories_len} L-BTC script histories, expected {lbtc_swap_scripts_len}")
            );
        let lbtc_script_to_history_map: HashMap<LBtcScript, Vec<LBtcHistory>> = swap_lbtc_scripts
            .into_iter()
            .zip(lbtc_script_histories.into_iter())
            .collect();

        debug!("Recoverer::fetch_lbtc_history_map: end");
        Ok(lbtc_script_to_history_map)
    }

    async fn fetch_btc_script_maps(
        &self,
        swap_btc_script_bufs: Vec<BtcScript>,
    ) -> Result<(
        HashMap<BtcScript, Vec<BtcHistory>>,
        HashMap<BtcScript, Vec<bitcoin::Transaction>>,
        HashMap<BtcScript, BtcScriptBalance>,
    )> {
        debug!("Recoverer::fetch_btc_history_map: start");
        let swap_btc_scripts = swap_btc_script_bufs
            .iter()
            .map(|x| x.as_script())
            .collect::<Vec<&lwk_wollet::bitcoin::Script>>();

        let t0 = web_time::Instant::now();
        let btc_script_histories = self
            .bitcoin_chain_service
            .get_scripts_history_with_retry(&swap_btc_scripts, 3)
            .await?;

        info!(
            "Recoverer::fetch_btc_history_map: executed bitcoin get_scripts_history for {} scripts in {} milliseconds",
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
        let btc_script_to_history_map: HashMap<BtcScript, Vec<BtcHistory>> = swap_btc_script_bufs
            .clone()
            .into_iter()
            .zip(btc_script_histories.clone())
            .collect();

        let t0 = web_time::Instant::now();
        let btc_script_txs = self
            .bitcoin_chain_service
            .get_transactions_with_retry(&btx_script_tx_ids, 3)
            .await?;
        info!(
            "Recoverer executed bitcoin get_transactions for {} transactions in {} milliseconds",
            btx_script_tx_ids.len(),
            t0.elapsed().as_millis()
        );

        let t0 = web_time::Instant::now();
        let btc_script_balances = self
            .bitcoin_chain_service
            .scripts_get_balance(&swap_btc_scripts)
            .await?;
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
                    let relevant_tx_ids: Vec<bitcoin::Txid> =
                        history.iter().map(|h| h.txid).collect();
                    let relevant_txs: Vec<bitcoin::Transaction> = btc_script_txs
                        .iter()
                        .filter(|&tx| {
                            relevant_tx_ids.contains(&tx.compute_txid().to_raw_hash().into())
                        })
                        .cloned()
                        .collect();

                    (script, relevant_txs)
                })
                .collect();

        let btc_script_to_balance_map: HashMap<BtcScript, BtcScriptBalance> = swap_btc_script_bufs
            .into_iter()
            .zip(btc_script_balances)
            .collect();

        debug!("Recoverer::fetch_btc_history_map: end");
        Ok((
            btc_script_to_history_map,
            btc_script_to_txs_map,
            btc_script_to_balance_map,
        ))
    }
}
