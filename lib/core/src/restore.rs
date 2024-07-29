//! This module provides functionality for restoring the swap tx IDs from onchain data

use std::collections::HashMap;

use anyhow::{anyhow, ensure, Result};
use boltz_client::boltz::CreateReverseResponse;
use boltz_client::LBtcSwapScript;
use log::{error, info};
use lwk_wollet::elements::{Script, Txid};
use lwk_wollet::{History, WalletTx};

use crate::restore::immutable::SwapsList;
use crate::sdk::LiquidSdk;

/// A map of all our known onchain txs, indexed by tx ID
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

pub(crate) struct RecoveredOnchainData {
    send: RecoveredOnchainDataSend,
    receive: RecoveredOnchainDataReceive,
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
        let send_swap_scripts: Vec<&Script> = immutable_db
            .send_db
            .iter()
            .map(|(_, (_, script))| script)
            .collect();
        let receive_swap_scripts: Vec<&Script> = immutable_db
            .receive_db
            .iter()
            .map(|(_, (_, _, script))| script)
            .collect();
        let mut swap_scripts = send_swap_scripts.clone();
        swap_scripts.extend(receive_swap_scripts.clone());

        let script_histories = self
            .liquid_chain_service
            .lock()
            .await
            .get_scripts_history(swap_scripts.as_slice())
            .await?;
        let swap_scripts_len = swap_scripts.len();
        let script_histories_len = script_histories.len();
        ensure!(
            swap_scripts_len == script_histories_len,
            anyhow!("Got {script_histories_len} script histories, expected {swap_scripts_len}")
        );
        let script_to_history_map: Vec<(&Script, Vec<History>)> = swap_scripts
            .into_iter()
            .zip(script_histories.into_iter())
            .collect();

        let mut send_swaps_script_to_history_map: HashMap<Script, Vec<History>> = HashMap::new();
        let mut receive_swaps_script_to_history_map: HashMap<Script, Vec<History>> = HashMap::new();
        script_to_history_map
            .into_iter()
            .for_each(|(script, history)| {
                if send_swap_scripts.contains(&script) {
                    send_swaps_script_to_history_map.insert(script.clone(), history);
                } else if receive_swap_scripts.contains(&script) {
                    receive_swaps_script_to_history_map.insert(script.clone(), history);
                }
                else {
                    error!("Found script that doesn't belong to either Send or Receive swaps: {script:?}");
                }
            });

        let mut send_composite_data: HashMap<String, (LBtcSwapScript, Script, &Vec<History>)> =
            HashMap::new();
        for (swap_id, (script, script_pk)) in immutable_db.send_db {
            match send_swaps_script_to_history_map.get(&script_pk) {
                None => error!("No history found for script pk {script_pk:?}"),
                Some(history) => {
                    send_composite_data.insert(swap_id, (script, script_pk, history));
                }
            }
        }
        let mut receive_composite_data: HashMap<
            String,
            (CreateReverseResponse, LBtcSwapScript, Script, &Vec<History>),
        > = HashMap::new();
        for (swap_id, (create_resp, script, script_pk)) in immutable_db.receive_db {
            match receive_swaps_script_to_history_map.get(&script_pk) {
                None => error!("No history found for script pk {script_pk:?}"),
                Some(history) => {
                    receive_composite_data
                        .insert(swap_id, (create_resp, script, script_pk, history));
                }
            }
        }

        let recovered_send_data = self
            .recover_send_swap_tx_ids(&tx_map, &send_composite_data)
            .await?;
        let recovered_receive_data = self
            .recover_receive_swap_tx_ids(&tx_map, &receive_composite_data)
            .await?;

        Ok(RecoveredOnchainData {
            send: recovered_send_data,
            receive: recovered_receive_data,
        })
    }

    /// Reconstruct Send Swap tx IDs from the onchain data and the immutable DB data
    pub(crate) async fn recover_send_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        send_composite_data: &HashMap<String, (LBtcSwapScript, Script, &Vec<History>)>,
    ) -> Result<RecoveredOnchainDataSend> {
        let mut lockup_tx_map: HashMap<String, Txid> = HashMap::new();
        let mut refund_tx_map: HashMap<String, Txid> = HashMap::new();

        for (swap_id, (_script, _script_pk, history)) in send_composite_data {
            // If a history tx is one of our outgoing txs, it's a lockup tx
            let maybe_lockup_tx_id = history
                .iter()
                .find(|&tx| tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid))
                .map(|h| h.txid);
            match maybe_lockup_tx_id {
                None => {
                    error!("No lockup tx found when recovering data for Send Swap {swap_id}")
                }
                Some(lockup_tx_id) => {
                    lockup_tx_map.insert(swap_id.clone(), lockup_tx_id);
                }
            }

            // If a history tx is one of our incoming txs, it's a refund tx
            let maybe_refund_tx_id = history
                .iter()
                .find(|tx| tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
                .map(|h| h.txid);
            if let Some(refund_tx_id) = maybe_refund_tx_id {
                refund_tx_map.insert(swap_id.clone(), refund_tx_id);
            }
        }

        info!(
            "[Recover Send] Found {} lockup txs from onchain data",
            lockup_tx_map.len()
        );
        info!(
            "[Recover Send] Found {} refund txs from onchain data",
            refund_tx_map.len()
        );

        Ok(RecoveredOnchainDataSend {
            lockup_tx_ids: lockup_tx_map,
            refund_tx_ids: refund_tx_map,
        })
    }

    /// Reconstruct Receive Swap tx IDs from the onchain data and the immutable DB data
    pub(crate) async fn recover_receive_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        receive_composite_data: &HashMap<
            String,
            (CreateReverseResponse, LBtcSwapScript, Script, &Vec<History>),
        >,
    ) -> Result<RecoveredOnchainDataReceive> {
        let mut lockup_claim_tx_ids_map: HashMap<String, (Txid, Txid)> = HashMap::new();

        for (swap_id, (_create_resp, _script, _script_pk, history)) in receive_composite_data {
            match history.len() {
                2 => {
                    let first = &history[0];
                    let second = &history[1];

                    // If a history tx is a known incoming txs, it's the claim tx
                    let (lockup_tx, claim_tx) =
                        match tx_map.incoming_tx_map.contains_key::<Txid>(&first.txid) {
                            true => (second, first),
                            false => (first, second),
                        };
                    lockup_claim_tx_ids_map
                        .insert(swap_id.clone(), (lockup_tx.txid, claim_tx.txid));
                }
                n => {
                    error!("Script history with unexpected length {n} found while recovering data for Receive Swap {swap_id}")
                }
            }
        }

        info!(
            "[Recover Receive] Found {} lockup and claim tx pairs from onchain data",
            lockup_claim_tx_ids_map.len()
        );

        Ok(RecoveredOnchainDataReceive {
            lockup_claim_tx_ids: lockup_claim_tx_ids_map,
        })
    }
}

/// Methods to simulate the immutable DB data available from real-time sync
// TODO Remove once real-time sync is integrated
pub(crate) mod immutable {
    use std::collections::HashMap;

    use anyhow::Result;
    use boltz_client::boltz::CreateReverseResponse;
    use boltz_client::LBtcSwapScript;
    use log::{error, info};
    use lwk_wollet::elements::Script;

    use crate::sdk::LiquidSdk;

    /// Swap data received from the immutable DB
    pub(crate) struct SwapsList {
        pub(crate) send_db: HashMap<String, (LBtcSwapScript, Script)>,
        pub(crate) receive_db: HashMap<String, (CreateReverseResponse, LBtcSwapScript, Script)>,
    }

    impl LiquidSdk {
        pub(crate) async fn get_swaps_list(&self) -> Result<SwapsList> {
            let con = self.persister.get_connection()?;

            // Send Swap scripts by swap ID
            let send_swap_immutable_db: HashMap<String, (LBtcSwapScript, Script)> = self
                .persister
                .list_send_swaps(&con, vec![])?
                .iter()
                .filter_map(|swap| match swap.get_swap_script() {
                    Ok(script) => match &script.funding_addrs {
                        Some(address) => {
                            Some((swap.id.clone(), (script.clone(), address.script_pubkey())))
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
            let send_swap_immutable_db_size = send_swap_immutable_db.len();
            info!("Send Swap immutable DB: {send_swap_immutable_db_size} rows");

            let receive_swap_immutable_db: HashMap<String, (CreateReverseResponse, LBtcSwapScript, Script)> =
                self.persister
                    .list_receive_swaps(&con, vec![])?
                    .iter()
                    .filter_map(|swap| {
                        let create_response = swap.get_boltz_create_response();
                        let swap_script = swap.get_swap_script();
                        let swap_id = &swap.id;

                        match (&create_response, &swap_script) {
                            (Ok(response), Ok(script)) => {
                                match &script.funding_addrs {
                                    Some(address) => {
                                        Some((swap.id.clone(), (response.clone(), script.clone(), address.script_pubkey())))
                                    },
                                    None => {
                                        error!("No funding address found for Receive Swap {}", swap.id);
                                        None
                                    }
                                }
                            },
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
            let receive_swap_immutable_db_size = receive_swap_immutable_db.len();
            info!("Receive Swap immutable DB: {receive_swap_immutable_db_size} rows");

            Ok(SwapsList {
                send_db: send_swap_immutable_db,
                receive_db: receive_swap_immutable_db,
            })
        }
    }
}
