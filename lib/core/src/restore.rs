//! This module provides functionality for restoring the swap tx IDs from onchain data

use std::collections::HashMap;

use anyhow::Result;
use log::{error, info};
use lwk_wollet::elements::Txid;
use lwk_wollet::WalletTx;

use crate::restore::immutable::{ReceiveChainSwapCompositeData, ReceiveSwapCompositeData, SendChainSwapCompositeData, SendSwapCompositeData, SwapsList};
use crate::sdk::LiquidSdk;

/// A map of all our known LWK onchain txs, indexed by tx ID
pub(crate) struct TxMap {
    outgoing_tx_map: HashMap<Txid, WalletTx>,
    incoming_tx_map: HashMap<Txid, WalletTx>,
}
impl TxMap {
    pub(crate) fn from_raw_tx_map(raw_tx_map: HashMap<Txid, WalletTx>) -> Self {
        let (outgoing_tx_map, incoming_tx_map): (HashMap<Txid, WalletTx>, HashMap<Txid, WalletTx>) =
            raw_tx_map
                .into_iter()
                .partition(|(_, tx)| tx.balance.values().sum::<i64>() < 0);

        Self {
            outgoing_tx_map,
            incoming_tx_map,
        }
    }
}

pub(crate) struct RecoveredOnchainDataSend {
    lockup_tx_ids: HashMap<String, Txid>,
    refund_tx_ids: HashMap<String, Txid>,
}

pub(crate) struct RecoveredOnchainDataReceive {
    lockup_claim_tx_ids: HashMap<String, (Txid, Txid)>,
}

pub(crate) struct RecoveredOnchainDataChainSend {
    lbtc_user_lockup_tx_ids: HashMap<String, Txid>,
    lbtc_refund_tx_ids: HashMap<String, Txid>,
    btc_server_lockup_tx_ids: HashMap<String, Txid>,
    btc_claim_tx_ids: HashMap<String, Txid>,
}

pub(crate) struct RecoveredOnchainDataChainReceive {
    /// Server lockup tx ID, claim tx ID.
    ///
    /// We store them in a pair because when they are present, we always expect both to be present.
    lbtc_server_lockup_claim_tx_ids: HashMap<String, (Txid, Txid)>,

    btc_user_lockup_tx_ids: HashMap<String, Txid>,
    btc_refund_tx_ids: HashMap<String, Txid>,
}

pub(crate) struct RecoveredOnchainData {
    send: RecoveredOnchainDataSend,
    receive: RecoveredOnchainDataReceive,
    chain_send:    RecoveredOnchainDataChainSend,
    chain_receive: RecoveredOnchainDataChainReceive,
}

impl LiquidSdk {
    pub(crate) async fn recover_from_onchain(&self, tx_map: TxMap) -> Result<()> {
        let immutable_db = self.get_swaps_list().await?;

        let _recovered = self.get_onchain_data(tx_map, immutable_db).await?;

        // TODO Persist updated swaps
        // TODO     Send updates with found txids
        // TODO     How to set tx IDs without having to also set state?
        // self.send_swap_state_handler
        //     .update_swap_info(&swap_id, Failed, None, None, Some(&refund_tx_id_str))
        //     .await?;

        Ok(())
    }
    pub(crate) async fn get_onchain_data(
        &self,
        tx_map: TxMap,
        immutable_db: SwapsList,
    ) -> Result<RecoveredOnchainData> {
        let swap_list_with_histories = self.get_swaps_list_with_histories(immutable_db).await?;

        let recovered_send_data = self
            .recover_send_swap_tx_ids(&tx_map, swap_list_with_histories.send_swaps_composite_data_by_swap_id)
            .await?;
        let recovered_receive_data = self
            .recover_receive_swap_tx_ids(&tx_map, swap_list_with_histories.receive_swaps_composite_data_by_swap_id)
            .await?;
        let recovered_chain_send_data = self
            .recover_send_chain_swap_tx_ids(&tx_map, swap_list_with_histories.send_chain_swaps_composite_data_by_swap_id)
            .await?;
        let recovered_chain_receive_data = self
            .recover_receive_chain_swap_tx_ids(&tx_map, swap_list_with_histories.receive_chain_swaps_composite_data_by_swap_id)
            .await?;

        Ok(RecoveredOnchainData {
            send: recovered_send_data,
            receive: recovered_receive_data,
            chain_send: recovered_chain_send_data,
            chain_receive: recovered_chain_receive_data,
        })
    }

    /// Reconstruct Send Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_send_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        send_composite_data_by_swap_id: HashMap<String, SendSwapCompositeData>,
    ) -> Result<RecoveredOnchainDataSend> {
        let mut lockup_tx_map: HashMap<String, Txid> = HashMap::new();
        let mut refund_tx_map: HashMap<String, Txid> = HashMap::new();

        for (swap_id, comp) in send_composite_data_by_swap_id {
            // If a history tx is one of our outgoing txs, it's a lockup tx
            let maybe_lockup_tx_id = comp.history
                .iter()
                .find(|tx_id| tx_map.outgoing_tx_map.contains_key::<Txid>(tx_id));
            match maybe_lockup_tx_id {
                None => {
                    error!("No lockup tx found when recovering data for Send Swap {swap_id}")
                }
                Some(lockup_tx_id) => {
                    lockup_tx_map.insert(swap_id.clone(), *lockup_tx_id);
                }
            }

            // If a history tx is one of our incoming txs, it's a refund tx
            let maybe_refund_tx_id = comp.history
                .iter()
                .find(|tx_id| tx_map.incoming_tx_map.contains_key::<Txid>(tx_id));
            if let Some(refund_tx_id) = maybe_refund_tx_id {
                refund_tx_map.insert(swap_id.clone(), *refund_tx_id);
            }
        }

        info!("[Recover Send] Found {} lockup txs from onchain data", lockup_tx_map.len());
        info!("[Recover Send] Found {} refund txs from onchain data", refund_tx_map.len());

        Ok(RecoveredOnchainDataSend {
            lockup_tx_ids: lockup_tx_map,
            refund_tx_ids: refund_tx_map,
        })
    }

    /// Reconstruct Receive Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_receive_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        receive_composite_data_by_swap_id: HashMap<String, ReceiveSwapCompositeData>,
    ) -> Result<RecoveredOnchainDataReceive> {
        let mut lockup_claim_tx_ids_map: HashMap<String, (Txid, Txid)> = HashMap::new();

        for (swap_id, comp) in receive_composite_data_by_swap_id {
            match comp.history.len() {
                2 => {
                    let first = &comp.history[0];
                    let second = &comp.history[1];

                    // If a history tx is a known incoming txs, it's the claim tx
                    let (lockup_tx_id, claim_tx_id) =
                        match tx_map.incoming_tx_map.contains_key::<Txid>(first) {
                            true => (second, first),
                            false => (first, second),
                        };
                    lockup_claim_tx_ids_map.insert(swap_id.clone(), (*lockup_tx_id, *claim_tx_id));
                }
                n => {
                    error!("Script history with unexpected length {n} found while recovering data for Receive Swap {swap_id}")
                }
            }
        }

        info!("[Recover Receive] Found {} lockup and claim tx pairs from onchain data",lockup_claim_tx_ids_map.len());

        Ok(RecoveredOnchainDataReceive {
            lockup_claim_tx_ids: lockup_claim_tx_ids_map,
        })
    }

    /// Reconstruct Chain Send Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_send_chain_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        chain_send_composite_data_by_swap_id: HashMap<String, SendChainSwapCompositeData>,
    ) -> Result<RecoveredOnchainDataChainSend> {
        let mut lbtc_user_lockup_tx_map: HashMap<String, Txid> = HashMap::new();
        let mut lbtc_refund_tx_map: HashMap<String, Txid> = HashMap::new();
        let mut btc_server_lockup_tx_map: HashMap<String, Txid> = HashMap::new();
        let mut btc_claim_tx_map: HashMap<String, Txid> = HashMap::new();

        for (swap_id, comp) in chain_send_composite_data_by_swap_id {
            info!("[Recover Chain Send] Checking swap {swap_id}");

            // If a history tx is one of our outgoing txs, it's a lockup tx
            let maybe_lockup_tx_id = comp.lbtc_lockup_script_history
                .iter()
                .find(|tx_id| tx_map.outgoing_tx_map.contains_key::<Txid>(tx_id));
            match maybe_lockup_tx_id {
                None => {
                    error!("No lockup tx found when recovering data for Chain Send Swap {swap_id}")
                }
                Some(lockup_tx_id) => {
                    lbtc_user_lockup_tx_map.insert(swap_id.clone(), *lockup_tx_id);
                }
            }

            // If a history tx is one of our incoming txs, it's a refund tx
            let maybe_refund_tx_id = comp.lbtc_lockup_script_history
                .iter()
                .find(|tx_id| tx_map.incoming_tx_map.contains_key::<Txid>(tx_id));
            if let Some(refund_tx_id) = maybe_refund_tx_id {
                lbtc_refund_tx_map.insert(swap_id.clone(), *refund_tx_id);
            }

            match comp.btc_claim_script_history.len() {
                2 => {
                    // TODO How to tell the claim tx apart from the lockup tx? Is the order in which they're received from Electrum reliable?
                    btc_server_lockup_tx_map.insert(swap_id.clone(), comp.btc_claim_script_history[0]);
                    btc_claim_tx_map.insert(swap_id.clone(), comp.btc_claim_script_history[1]);
                }
                n => {
                    error!("BTC script history with unexpected length {n} found while recovering data for Chain Send Swap {swap_id}")
                }
            }
        }

        info!("[Recover Chain Send] Found user {} L-BTC lockup txs from onchain data", lbtc_user_lockup_tx_map.len());
        info!("[Recover Chain Send] Found {} L-BTC refund txs from onchain data", lbtc_refund_tx_map.len());
        info!("[Recover Chain Send] Found server {} BTC lockup txs from onchain data", btc_server_lockup_tx_map.len());
        info!("[Recover Chain Send] Found {} BTC claim txs from onchain data", btc_claim_tx_map.len());

        Ok(RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_ids: lbtc_user_lockup_tx_map,
            lbtc_refund_tx_ids: lbtc_refund_tx_map,
            btc_server_lockup_tx_ids: btc_server_lockup_tx_map,
            btc_claim_tx_ids: btc_claim_tx_map,
        })
    }

    /// Reconstruct Chain Receive Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_receive_chain_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        chain_receive_composite_data_by_swap_id: HashMap<String, ReceiveChainSwapCompositeData>,
    ) -> Result<RecoveredOnchainDataChainReceive> {
        let mut lbtc_server_lockup_claim_tx_ids: HashMap<String, (Txid, Txid)> = HashMap::new();
        let mut btc_user_lockup_tx_ids: HashMap<String, Txid> = HashMap::new();
        let mut btc_refund_tx_ids: HashMap<String, Txid> = HashMap::new();

        for (swap_id, comp) in chain_receive_composite_data_by_swap_id {
            info!("[Recover Chain Receive] Checking swap {swap_id}");

            match comp.lbtc_claim_script_history.len() {
                2 => {
                    let first = &comp.lbtc_claim_script_history[0];
                    let second = &comp.lbtc_claim_script_history[1];

                    // If a history tx is a known incoming txs, it's the claim tx
                    let (lockup_tx_id, claim_tx_id) =
                        match tx_map.incoming_tx_map.contains_key::<Txid>(first) {
                            true => (second, first),
                            false => (first, second),
                        };
                    lbtc_server_lockup_claim_tx_ids
                        .insert(swap_id.clone(), (*lockup_tx_id, *claim_tx_id));
                }
                n => {
                    error!("L-BTC script history with unexpected length {n} found while recovering data for Chain Receive Swap {swap_id}")
                }
            }

            match comp.btc_lockup_script_history.len() {
                // TODO How to treat case when history length > 2? (address re-use)
                x if x >= 2 => {
                    // TODO How to tell the user lockup tx apart from the refund tx? Is the order in which they're received from Electrum reliable?
                    // TODO How to tell BTC refund apart from BTC server claim tx?
                    btc_user_lockup_tx_ids.insert(swap_id.clone(), comp.btc_lockup_script_history[0]);
                    btc_refund_tx_ids.insert(swap_id.clone(), comp.btc_lockup_script_history[1]);
                }
                n => {
                    error!("BTC script history with unexpected length {n} found while recovering data for Chain Receive Swap {swap_id}")
                }
            }
        }

        info!("[Recover Chain Receive] Found {} L-BTC server lockup and claim tx pairs from onchain data", lbtc_server_lockup_claim_tx_ids.len());
        info!("[Recover Chain Receive] Found {} BTC user lockup tx IDs from onchain data", btc_user_lockup_tx_ids.len());
        info!("[Recover Chain Receive] Found {} BTC refund tx IDs from onchain data", btc_refund_tx_ids.len());

        Ok(RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_claim_tx_ids,
            btc_user_lockup_tx_ids,
            btc_refund_tx_ids
        })
    }
}

/// Methods to simulate the immutable DB data available from real-time sync
// TODO Remove once real-time sync is integrated
pub(crate) mod immutable {
    use std::collections::HashMap;

    use anyhow::{anyhow, ensure, Result};
    use boltz_client::boltz::CreateReverseResponse;
    use boltz_client::{BtcSwapScript, LBtcSwapScript};
    use log::{error, info};
    use lwk_wollet::elements::Txid;

    use crate::prelude::*;
    use crate::sdk::LiquidSdk;

    type BtcScript = lwk_wollet::bitcoin::ScriptBuf;
    type LBtcScript = lwk_wollet::elements::Script;

    #[derive(Clone)]
    struct SendSwapImmutableData {
        pub(crate) swap_id: String,
        swap_script: LBtcSwapScript,
        pub(crate) script: LBtcScript
    }

    pub(crate) struct SendSwapCompositeData {
        immutable_data: SendSwapImmutableData,
        pub(crate) history: Vec<Txid>
    }

    #[derive(Clone)]
    struct ReceiveSwapImmutableData {
        pub(crate) swap_id: String,
        create_resp: CreateReverseResponse,
        pub(crate) swap_script: LBtcSwapScript,
        pub(crate) script: LBtcScript
    }

    pub(crate) struct ReceiveSwapCompositeData {
        immutable_data: ReceiveSwapImmutableData,
        pub(crate) history: Vec<Txid>
    }

    #[derive(Clone)]
    struct SendChainSwapImmutableData {
        swap_id: String,
        lockup_swap_script: LBtcSwapScript,
        lockup_script: LBtcScript,
        claim_swap_script: BtcSwapScript,
        claim_script: BtcScript
    }

    pub(crate) struct SendChainSwapCompositeData {
        immutable_data: SendChainSwapImmutableData,
        pub(crate) lbtc_lockup_script_history: Vec<Txid>,
        pub(crate) btc_claim_script_history: Vec<Txid>
    }

    #[derive(Clone)]
    struct ReceiveChainSwapImmutableData {
        swap_id: String,
        lockup_swap_script: BtcSwapScript,
        lockup_script: BtcScript,
        claim_swap_script: LBtcSwapScript,
        claim_script: LBtcScript
    }

    pub(crate) struct ReceiveChainSwapCompositeData {
        immutable_data: ReceiveChainSwapImmutableData,
        pub(crate) lbtc_claim_script_history: Vec<Txid>,
        pub(crate) btc_lockup_script_history: Vec<Txid>
    }

    /// Swap data received from the immutable DB
    pub(crate) struct SwapsList {
        send_swap_immutable_db_by_swap_id: HashMap<String, SendSwapImmutableData>,
        receive_swap_immutable_db_by_swap_id_: HashMap<String, ReceiveSwapImmutableData>,
        send_chain_swap_immutable_db_by_swap_id: HashMap<String, SendChainSwapImmutableData>,
        receive_chain_swap_immutable_db_by_swap_id: HashMap<String, ReceiveChainSwapImmutableData>
    }

    impl SwapsList {
        fn init(
            send_swaps: Vec<SendSwap>,
            receive_swaps: Vec<ReceiveSwap>,
            send_chain_swaps: Vec<ChainSwap>,
            receive_chain_swaps: Vec<ChainSwap>,
        ) -> Result<Self> {
            let send_swap_immutable_db_by_swap_id: HashMap<String, SendSwapImmutableData> = send_swaps
                .iter()
                .filter_map(|swap| match swap.get_swap_script() {
                    Ok(swap_script) => match &swap_script.funding_addrs {
                        Some(address) => {
                            Some((swap.id.clone(), SendSwapImmutableData {
                                swap_id: swap.id.clone(),
                                swap_script: swap_script.clone(),
                                script: address.script_pubkey()
                            }))
                        }
                        None => {
                            error!("No funding address found for Send Swap {}", swap.id);
                            None
                        }
                    },
                    Err(e) => {
                        error!("Failed to get swap script for Send Swap {}: {e}", swap.id);
                        None
                    }
                })
                .collect();
            let send_swap_immutable_db_size = send_swap_immutable_db_by_swap_id.len();
            info!("Send Swap immutable DB: {send_swap_immutable_db_size} rows");

            let receive_swap_immutable_db_by_swap_id_: HashMap<String, ReceiveSwapImmutableData> =
                receive_swaps
                    .iter()
                    .filter_map(|swap| {
                        let create_response_res = swap.get_boltz_create_response();
                        let swap_script_res = swap.get_swap_script();
                        let swap_id = &swap.id;

                        match (&create_response_res, &swap_script_res) {
                            (Ok(create_resp), Ok(swap_script)) => {
                                match &swap_script.funding_addrs {
                                    Some(address) => {
                                        Some((swap.id.clone(), ReceiveSwapImmutableData {
                                            swap_id: swap.id.clone(),
                                            create_resp: create_resp.clone(),
                                            swap_script: swap_script.clone(),
                                            script: address.script_pubkey(),
                                        }))
                                    },
                                    None => {
                                        error!("No funding address found for Receive Swap {}", swap.id);
                                        None
                                    }
                                }
                            }
                            (Err(e), _) => {
                                error!("Failed to deserialize Create Response for Receive Swap {swap_id}: {e}");
                                None
                            }
                            (_, Err(e)) => {
                                error!("Failed to get swap script for Receive Swap {swap_id}: {e}");
                                None
                            }
                        }
                    })
                    .collect();
            let receive_swap_immutable_db_size = receive_swap_immutable_db_by_swap_id_.len();
            info!("Receive Swap immutable DB: {receive_swap_immutable_db_size} rows");

            let send_chain_swap_immutable_db_by_swap_id: HashMap<String, SendChainSwapImmutableData> =
                send_chain_swaps.iter().filter_map(|swap| {
                    let maybe_lockup_script = swap.get_lockup_swap_script()
                        .map(|s| s.as_liquid_script().ok()).ok().flatten();
                    let maybe_claim_script = swap.get_claim_swap_script()
                        .map(|s| s.as_bitcoin_script().ok()).ok().flatten();

                    match (maybe_lockup_script, maybe_claim_script) {
                        (Some(lockup_swap_script), Some(claim_script)) => {
                            let maybe_lockup_script_pk = lockup_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());
                            let maybe_claim_script_pk = claim_script.clone().funding_addrs.map(|addr| addr.script_pubkey());

                            match (maybe_lockup_script_pk, maybe_claim_script_pk) {
                                (Some(lockup_script), Some(claim_script_pk)) => {
                                    Some((swap.id.clone(), SendChainSwapImmutableData {
                                        swap_id: swap.id.clone(),
                                        lockup_swap_script,
                                        lockup_script,
                                        claim_swap_script: claim_script,
                                        claim_script: claim_script_pk,
                                    }))
                                }
                                // TODO Add errors and logging
                                _ => None
                            }
                        }
                        // TODO Add errors and logging
                        _ => None
                    }
                })
                .collect();
            let send_chain_swap_immutable_db_size = send_chain_swap_immutable_db_by_swap_id.len();
            info!("Send Chain Swap immutable DB: {send_chain_swap_immutable_db_size} rows");

            let receive_chain_swap_immutable_db_by_swap_id: HashMap<String, ReceiveChainSwapImmutableData> =
                receive_chain_swaps.iter().filter_map(|swap| {
                    let maybe_lockup_swap_script = swap.get_lockup_swap_script()
                        .map(|s| s.as_bitcoin_script().ok()).ok().flatten();
                    let maybe_claim_swap_script = swap.get_claim_swap_script()
                        .map(|s| s.as_liquid_script().ok()).ok().flatten();

                    match  (maybe_lockup_swap_script, maybe_claim_swap_script) {
                        (Some(lockup_swap_script), Some(claim_swap_script)) => {
                            let maybe_lockup_script = lockup_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());
                            let maybe_claim_script = claim_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());

                            match (maybe_lockup_script, maybe_claim_script) {
                                (Some(lockup_script), Some(claim_script)) => {
                                    Some((swap.id.clone(), ReceiveChainSwapImmutableData {
                                        swap_id: swap.id.clone(),
                                        lockup_swap_script,
                                        lockup_script,
                                        claim_swap_script,
                                        claim_script,
                                    }))
                                }
                                // TODO Add errors and logging
                                _ => None
                            }
                        }
                        // TODO Add errors and logging
                        _ => None
                    }
                })
                .collect();
            let receive_chain_swap_immutable_db_size = receive_chain_swap_immutable_db_by_swap_id.len();
            info!("Receive Chain Swap immutable DB: {receive_chain_swap_immutable_db_size} rows");

            Ok(SwapsList {
                send_swap_immutable_db_by_swap_id,
                receive_swap_immutable_db_by_swap_id_,
                send_chain_swap_immutable_db_by_swap_id,
                receive_chain_swap_immutable_db_by_swap_id,
            })
        }

        fn send_swaps_by_script(&self) -> HashMap<LBtcScript, SendSwapImmutableData> {
            self.send_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| (imm.script.clone(), imm))
                .collect()
        }

        fn send_composite_by_swap_id(&self, lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<Txid>>) -> HashMap<String, SendSwapCompositeData> {
            let send_swaps_by_script = self.send_swaps_by_script();

            let mut data: HashMap<String, SendSwapCompositeData> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script, lbtc_script_history)| {
                    if let Some(imm) = send_swaps_by_script.get(lbtc_script) {
                        data.insert(imm.swap_id.clone(), SendSwapCompositeData {
                            immutable_data: imm.clone(),
                            history: lbtc_script_history.clone(),
                        });
                    }
                });
            data
        }

        fn receive_swaps_by_script(
            &self,
        ) -> HashMap<LBtcScript, ReceiveSwapImmutableData> {
            self.receive_swap_immutable_db_by_swap_id_
                .clone()
                .into_values().map(|imm| (imm.script.clone(), imm))
                .collect()
        }

        fn receive_composite_by_swap_id(&self, lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<Txid>>) -> HashMap<String, ReceiveSwapCompositeData> {
            let receive_swaps_by_script = self.receive_swaps_by_script();

            let mut data: HashMap<String, ReceiveSwapCompositeData> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script, lbtc_script_history)| {
                    if let Some(imm) = receive_swaps_by_script.get(lbtc_script) {
                        data.insert(imm.swap_id.clone(), ReceiveSwapCompositeData {
                            immutable_data: imm.clone(),
                            history: lbtc_script_history.clone(),
                        });
                    }
                });
            data
        }

        fn send_chain_swaps_by_lbtc_lockup_script(
            &self,
        ) -> HashMap<LBtcScript, SendChainSwapImmutableData> {
            self.send_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| (imm.lockup_script.clone(), imm))
                .collect()
        }

        fn send_chain_composite_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<Txid>>,
            btc_script_to_history_map: &HashMap<BtcScript, Vec<Txid>>,
        ) -> HashMap<String, SendChainSwapCompositeData> {
            let send_chain_swaps_by_lbtc_script = self.send_chain_swaps_by_lbtc_lockup_script();

            let mut data: HashMap<String, SendChainSwapCompositeData> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_lockup_script, lbtc_script_history)| {
                    if let Some(imm) = send_chain_swaps_by_lbtc_script.get(lbtc_lockup_script) {
                        let btc_script_history = btc_script_to_history_map.get(&imm.claim_script).cloned().unwrap_or_default();

                        data.insert(imm.swap_id.clone(), SendChainSwapCompositeData {
                            immutable_data: imm.clone(),
                            lbtc_lockup_script_history: lbtc_script_history.clone(),
                            btc_claim_script_history: btc_script_history,
                        });
                    }
                });
            data
        }

        fn receive_chain_swaps_by_lbtc_claim_script(
            &self,
        ) -> HashMap<LBtcScript, ReceiveChainSwapImmutableData> {
            self.receive_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| (imm.claim_script.clone(), imm))
                .collect()
        }

        fn receive_chain_composite_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<Txid>>,
            btc_script_to_history_map: &HashMap<BtcScript, Vec<Txid>>,
        ) -> HashMap<String, ReceiveChainSwapCompositeData> {
            let receive_chain_swaps_by_lbtc_script = self.receive_chain_swaps_by_lbtc_claim_script();

            let mut data: HashMap<String, ReceiveChainSwapCompositeData> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script_pk, lbtc_script_history)| {
                    if let Some(imm) = receive_chain_swaps_by_lbtc_script.get(lbtc_script_pk) {
                        let btc_script_history = btc_script_to_history_map.get(&imm.lockup_script).cloned().unwrap_or_default();

                        data.insert(imm.swap_id.clone(), ReceiveChainSwapCompositeData {
                            immutable_data: imm.clone(),
                            lbtc_claim_script_history: lbtc_script_history.clone(),
                            btc_lockup_script_history: btc_script_history.clone()
                        });
                    }
                });
            data
        }

        fn get_all_swap_lbtc_scripts(&self) -> Vec<LBtcScript> {
            let send_swap_scripts: Vec<LBtcScript> =
                self.send_swap_immutable_db_by_swap_id.clone().into_values().map(|imm| imm.script).collect();
            let receive_swap_scripts: Vec<LBtcScript> = self
                .receive_swap_immutable_db_by_swap_id_.clone().into_values().map(|imm| imm.script)
                .collect();
            let send_chain_swap_lbtc_lockup_scripts: Vec<LBtcScript> = self
                .send_chain_swap_immutable_db_by_swap_id.clone().into_values().map(|imm| imm.lockup_script)
                .collect();
            let receive_chain_swap_lbtc_claim_scripts: Vec<LBtcScript> = self
                .receive_chain_swap_immutable_db_by_swap_id.clone().into_values().map(|imm| imm.claim_script)
                .collect();

            let mut swap_scripts = send_swap_scripts.clone();
            swap_scripts.extend(receive_swap_scripts.clone());
            swap_scripts.extend(send_chain_swap_lbtc_lockup_scripts.clone());
            swap_scripts.extend(receive_chain_swap_lbtc_claim_scripts.clone());
            swap_scripts
        }

        fn get_all_swap_btc_scripts(&self) -> Vec<BtcScript> {
            let send_chain_swap_btc_claim_scripts: Vec<BtcScript> = self
                .send_chain_swap_immutable_db_by_swap_id.clone().into_values().map(|imm| imm.claim_script)
                .collect();
            let receive_chain_swap_btc_lockup_scripts: Vec<BtcScript> = self
                .receive_chain_swap_immutable_db_by_swap_id.clone().into_values().map(|imm| imm.lockup_script)
                .collect();

            let mut swap_scripts = send_chain_swap_btc_claim_scripts.clone();
            swap_scripts.extend(receive_chain_swap_btc_lockup_scripts.clone());
            swap_scripts
        }
    }

    pub(crate) struct SwapsListWithHistories {
        pub(crate) send_swaps_composite_data_by_swap_id: HashMap<String, SendSwapCompositeData>,
        pub(crate) receive_swaps_composite_data_by_swap_id: HashMap<String, ReceiveSwapCompositeData>,
        pub(crate) send_chain_swaps_composite_data_by_swap_id: HashMap<String, SendChainSwapCompositeData>,
        pub(crate) receive_chain_swaps_composite_data_by_swap_id: HashMap<String, ReceiveChainSwapCompositeData>,
    }

    impl LiquidSdk {
        pub(crate) async fn get_swaps_list(&self) -> Result<SwapsList> {
            let send_swaps = self.persister.list_send_swaps()?;
            let receive_swaps = self.persister.list_receive_swaps()?;
            let chain_swaps = self.persister.list_chain_swaps()?;
            let (send_chain_swaps, receive_chain_swaps): (Vec<ChainSwap>, Vec<ChainSwap>) =
                chain_swaps
                    .into_iter()
                    .partition(|swap| swap.direction == Direction::Outgoing);

            SwapsList::init(send_swaps, receive_swaps, send_chain_swaps, receive_chain_swaps)
        }

        /// Extends the given [SwapList] with the script histories fetched from the [LiquidChainService]
        pub(crate) async fn get_swaps_list_with_histories(
            &self,
            swaps_list: SwapsList,
        ) -> Result<SwapsListWithHistories> {
            let swap_lbtc_scripts = swaps_list.get_all_swap_lbtc_scripts();

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
            let lbtc_script_to_history_map: HashMap<LBtcScript, Vec<Txid>> = swap_lbtc_scripts
                .into_iter()
                .zip(lbtc_script_histories.into_iter())
                .map(|(k, v)|(k, v.into_iter().map(|h| h.txid).collect()))
                .collect();

            let swap_btc_scripts = swaps_list.get_all_swap_btc_scripts();
            let btc_script_histories = self
                .bitcoin_chain_service
                .lock()
                .await
                .get_scripts_history(&swap_btc_scripts.iter().map(|x| x.as_script()).collect::<Vec<&lwk_wollet::bitcoin::Script>>())?;
            let btc_swap_scripts_len = swap_btc_scripts.len();
            let btc_script_histories_len = btc_script_histories.len();
            ensure!(
                btc_swap_scripts_len == btc_script_histories_len,
                anyhow!("Got {btc_script_histories_len} BTC script histories, expected {btc_swap_scripts_len}")
            );
            let btc_script_to_history_map: HashMap<BtcScript, Vec<Txid>> = swap_btc_scripts
                .into_iter()
                .zip(btc_script_histories.into_iter())
                .map(|(k, v)|(k, v.into_iter().map(|h| h.txid).collect()))
                .collect();

            // For each swap type, expand the immutable data with the script histories
            let send_composite = swaps_list.send_composite_by_swap_id(&lbtc_script_to_history_map);
            let receive_composite = swaps_list.receive_composite_by_swap_id(&lbtc_script_to_history_map);
            let send_chain_swaps_composite = swaps_list.send_chain_composite_by_swap_id(&lbtc_script_to_history_map,  &btc_script_to_history_map);
            let receive_chain_swaps_composite = swaps_list.receive_chain_composite_by_swap_id(&lbtc_script_to_history_map,  &btc_script_to_history_map);

            Ok(SwapsListWithHistories {
                send_swaps_composite_data_by_swap_id: send_composite,
                receive_swaps_composite_data_by_swap_id: receive_composite,
                send_chain_swaps_composite_data_by_swap_id: send_chain_swaps_composite,
                receive_chain_swaps_composite_data_by_swap_id: receive_chain_swaps_composite
            })
        }
    }
}
