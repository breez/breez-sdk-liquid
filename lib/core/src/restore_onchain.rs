/// Methods to simulate the immutable DB data available from real-time sync
// TODO Remove once real-time sync is integrated
pub(crate) mod immutable {
    use std::collections::HashMap;

    use anyhow::Result;
    use boltz_client::boltz::CreateReverseResponse;
    use boltz_client::LBtcSwapScript;
    use log::{error, info};

    use crate::sdk::LiquidSdk;

    pub(crate) struct ImmutableDb {
        pub(crate) send_db: HashMap<String, LBtcSwapScript>,
        pub(crate) receive_db: HashMap<String, (CreateReverseResponse, LBtcSwapScript)>,
    }

    impl LiquidSdk {
        pub(crate) async fn get_immutable_db_data(&self) -> Result<ImmutableDb> {
            let con = self.persister.get_connection()?;

            // Send Swap scripts by swap ID
            let send_swap_immutable_db: HashMap<String, LBtcSwapScript> = self
                .persister
                .list_send_swaps(&con, vec![])?
                .iter()
                .filter_map(|swap| match swap.get_swap_script() {
                    Ok(script) => Some((swap.id.clone(), script)),
                    Err(e) => {
                        error!("Failed to get swap script for Send Swap {}: {e}", swap.id);
                        None
                    }
                })
                .collect();
            let send_swap_immutable_db_size = send_swap_immutable_db.len();
            info!("Send Swap immutable DB: {send_swap_immutable_db_size} rows");

            let receive_swap_immutable_db: HashMap<String, (CreateReverseResponse, LBtcSwapScript)> =
                self.persister
                    .list_receive_swaps(&con, vec![])?
                    .iter()
                    .filter_map(|swap| {
                        let create_response = swap.get_boltz_create_response();
                        let swap_script = swap.get_swap_script();
                        let swap_id = &swap.id;

                        match (create_response, swap_script) {
                            (Ok(response), Ok(script)) => Some((swap.id.clone(), (response, script))),
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

            Ok(ImmutableDb {
                send_db: send_swap_immutable_db,
                receive_db: receive_swap_immutable_db,
            })
        }
    }
}

/// Methods to restore the swap tx IDs from the onchain data
pub(crate) mod onchain {
    use std::collections::HashMap;

    use anyhow::Result;
    use boltz_client::boltz::CreateReverseResponse;
    use boltz_client::LBtcSwapScript;
    use log::{error, info};
    use lwk_wollet::elements::{OutPoint, Txid};
    use lwk_wollet::WalletTx;

    use crate::restore_onchain::immutable::ImmutableDb;
    use crate::sdk::LiquidSdk;

    impl LiquidSdk {
        pub(crate) async fn recover_from_onchain(
            &self,
            tx_map: HashMap<Txid, WalletTx>,
        ) -> Result<()> {
            // TODO Fetch immutable DB data
            let immutable_db = self.get_immutable_db_data().await?;

            self.get_onchain_data(tx_map, immutable_db).await?;

            // TODO Persist updated swaps

            Ok(())
        }
        pub(crate) async fn get_onchain_data(
            &self,
            tx_map: HashMap<Txid, WalletTx>,
            immutable_db: ImmutableDb,
        ) -> Result<()> {
            // Find incoming txs that have a single input and a single output, indexed by input
            let possible_claim_or_refund_txs_by_input: HashMap<OutPoint, Txid> = tx_map
                .iter()
                .filter(|(_, tx)| tx.balance.values().sum::<i64>() > 0)
                .filter(|(_, tx)| tx.tx.output.len() == 2) // Separate vout for fee
                .filter(|(_, tx)| tx.tx.input.len() == 1)
                .filter_map(|(_, wallet_tx)| {
                    wallet_tx
                        .tx
                        .input
                        .first()
                        .map(|input| (input.previous_output, wallet_tx.txid))
                })
                .collect();
            let possible_claim_or_refund_txs_found = possible_claim_or_refund_txs_by_input.len();
            info!("Found {possible_claim_or_refund_txs_found} possible claim or refund txs from onchain data");

            self.recover_send_swap_tx_ids(
                &tx_map,
                &immutable_db.send_db,
                &possible_claim_or_refund_txs_by_input,
            )
            .await?;

            self.recover_receive_swap_tx_ids(
                &immutable_db.receive_db,
                &possible_claim_or_refund_txs_by_input,
            )
            .await?;

            Ok(())
        }

        /// Reconstruct Send Swap tx IDs from the onchain data and the immutable DB data
        pub(crate) async fn recover_send_swap_tx_ids(
            &self,
            tx_map: &HashMap<Txid, WalletTx>,
            send_swap_immutable_db_data: &HashMap<String, LBtcSwapScript>,
            possible_claim_or_refund_txs_by_input: &HashMap<OutPoint, Txid>,
        ) -> Result<()> {
            let outgoing_tx_map: HashMap<&Txid, &WalletTx> = tx_map
                .iter()
                .filter(|(_, tx)| tx.balance.values().sum::<i64>() < 0) // Outgoing
                .collect();

            let lockup_txs_by_swap_id: HashMap<String, WalletTx> = send_swap_immutable_db_data
                .iter()
                .filter_map(|(swap_id, script)| {
                    let script_pk = script.funding_addrs.clone().map(|a| a.script_pubkey());
                    let maybe_lockup_tx = outgoing_tx_map.iter().find_map(|(_, &tx)| {
                        tx.tx
                            .output
                            .iter()
                            .find(|out| Some(out.script_pubkey.clone()) == script_pk)
                            .map(|_| tx)
                    });

                    match maybe_lockup_tx {
                        Some(lockup_tx) => Some((swap_id.clone(), lockup_tx.clone())),
                        None => {
                            error!(
                                "No lockup tx found when recovering data for Send Swap {swap_id}"
                            );
                            None
                        }
                    }
                })
                .collect();
            let lockup_txs_found = lockup_txs_by_swap_id.len();
            info!("Found {lockup_txs_found} lockup txs from onchain data");

            // Find refund tx IDs
            // These are incoming txs with 1 input and 1 output, where the input is a lockup tx output
            let refund_tx_ids_by_swap_id: HashMap<String, Txid> =
                possible_claim_or_refund_txs_by_input
                    .iter()
                    .filter_map(|(refund_tx_in, refund_tx_id)| {
                        // Find lockup tx with a matching output
                        lockup_txs_by_swap_id
                            .iter()
                            .find_map(|(swap_id, lockup_tx)| {
                                let is_match = lockup_tx.outputs.iter().any(|out| {
                                    // Refund fee is considered its own output, so the outpoint vouts don't match
                                    // Therefore we just check the tx IDs
                                    matches!(out, Some(o) if o.outpoint.txid == refund_tx_in.txid)
                                });

                                match is_match {
                                    true => Some((swap_id.clone(), *refund_tx_id)),
                                    false => None,
                                }
                            })
                    })
                    .collect();
            let refund_txs_found = refund_tx_ids_by_swap_id.len();
            info!("Found {refund_txs_found} refund txs from onchain data");

            // TODO Send update with found txids
            // TODO How to set tx IDs without having to also set state?
            // self.send_swap_state_handler
            //     .update_swap_info(&swap_id, Failed, None, None, Some(&refund_tx_id_str))
            //     .await?;

            Ok(())
        }

        /// Reconstruct Receive Swap tx IDs from the onchain data and the immutable DB data
        pub(crate) async fn recover_receive_swap_tx_ids(
            &self,
            receive_swap_immutable_db_data: &HashMap<
                String,
                (CreateReverseResponse, LBtcSwapScript),
            >,
            possible_claim_or_refund_txs_by_input: &HashMap<OutPoint, Txid>,
        ) -> Result<()> {
            let lockup_tx_ids: Vec<Txid> = possible_claim_or_refund_txs_by_input
                .iter()
                .map(|(k, _)| k.txid)
                .collect();
            let lockup_txs = self
                .liquid_chain_service
                .lock()
                .await
                .get_transactions(&lockup_tx_ids)
                .await?;

            let mut lockup_claim_tx_ids_by_swap_id: HashMap<&str, (Txid, Txid)> = HashMap::new();
            for (incoming_tx_prev_out, incoming_tx_id) in possible_claim_or_refund_txs_by_input {
                let possible_lockup_tx_id = incoming_tx_prev_out.txid;
                let possible_claim_tx_id = *incoming_tx_id;

                for lockup_tx in &lockup_txs {
                    if lockup_tx.txid() == possible_lockup_tx_id {
                        for (swap_id, (_, script)) in receive_swap_immutable_db_data {
                            let swap_script =
                                script.funding_addrs.clone().map(|a| a.script_pubkey());
                            let lockup_tx_matches_swap_script = lockup_tx
                                .output
                                .iter()
                                .any(|out| Some(out.script_pubkey.clone()) == swap_script);

                            if lockup_tx_matches_swap_script {
                                lockup_claim_tx_ids_by_swap_id
                                    .insert(swap_id, (possible_lockup_tx_id, possible_claim_tx_id));
                            }
                        }
                    }
                }
            }
            let lockup_claim_txs_found = lockup_claim_tx_ids_by_swap_id.len();
            info!("Found {lockup_claim_txs_found} lockup and claim txs from onchain data");

            Ok(())
        }
    }
}
