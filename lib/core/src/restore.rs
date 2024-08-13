//! This module provides functionality for restoring the swap tx IDs from onchain data

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use log::{error, info};
use lwk_wollet::elements::Txid;
use lwk_wollet::WalletTx;
use sdk_common::bitcoin::hashes::hex::ToHex;

use crate::prelude::*;
use crate::restore::immutable::*;

/// A map of all our known LWK onchain txs, indexed by tx ID. Essentially our own cache of the LWK txs.
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

trait PartialSwapState {
    /// Determine partial swap state, based on recovered chain data.
    /// For example, it cannot distinguish between [PaymentState::Created] and [PaymentState::TimedOut],
    /// and in some cases, between [PaymentState::Created] and [PaymentState::Failed]. In these
    /// cases, it defaults to [PaymentState::Created].
    fn get_partial_state(&self) -> PaymentState;
}

pub(crate) struct RecoveredOnchainDataSend {
    lockup_tx_id: Option<HistoryTxId>,
    claim_tx_id: Option<HistoryTxId>,
    refund_tx_id: Option<HistoryTxId>,
}
impl PartialSwapState for RecoveredOnchainDataSend {
    fn get_partial_state(&self) -> PaymentState {
        match &self.lockup_tx_id {
            Some(_) => match &self.claim_tx_id {
                Some(_) => PaymentState::Complete,
                None => match &self.refund_tx_id {
                    Some(refund_tx_id) => match refund_tx_id.confirmed {
                        true => PaymentState::Failed,
                        false => PaymentState::RefundPending,
                    },
                    None => PaymentState::Pending,
                },
            },
            None => PaymentState::TimedOut,
        }
    }
}

pub(crate) struct RecoveredOnchainDataReceive {
    lockup_tx_id: Option<HistoryTxId>,
    claim_tx_id: Option<HistoryTxId>,
}
impl PartialSwapState for RecoveredOnchainDataReceive {
    fn get_partial_state(&self) -> PaymentState {
        match (&self.lockup_tx_id, &self.claim_tx_id) {
            (Some(_), Some(claim_tx_id)) => match claim_tx_id.confirmed {
                true => PaymentState::Complete,
                false => PaymentState::Pending,
            },
            (Some(_), None) => PaymentState::Pending,
            // TODO How to distinguish between Failed and Created (if in both cases, no lockup or claim tx present)
            //   See https://docs.boltz.exchange/v/api/lifecycle#reverse-submarine-swaps
            _ => PaymentState::Created,
        }
    }
}

pub(crate) struct RecoveredOnchainDataChainSend {
    lbtc_user_lockup_tx_id: Option<HistoryTxId>,
    lbtc_refund_tx_id: Option<HistoryTxId>,
    btc_server_lockup_tx_id: Option<HistoryTxId>,
    btc_claim_tx_id: Option<HistoryTxId>,
}
impl PartialSwapState for RecoveredOnchainDataChainSend {
    fn get_partial_state(&self) -> PaymentState {
        // TODO How to detect TimedOut state?
        //     TimedOut: This covers the case when the swap state is still Created and the swap fails to reach the
        //     Pending state in time. The TimedOut state indicates the lockup tx should never be broadcast.
        match &self.lbtc_user_lockup_tx_id {
            Some(_) => match &self.btc_claim_tx_id {
                Some(_) => PaymentState::Complete,
                None => match &self.lbtc_refund_tx_id {
                    Some(tx) => match tx.confirmed {
                        true => PaymentState::Failed,
                        false => PaymentState::RefundPending,
                    },
                    None => PaymentState::Created,
                },
            },
            None => PaymentState::Created,
        }
    }
}

pub(crate) struct RecoveredOnchainDataChainReceive {
    lbtc_server_lockup_tx_id: Option<HistoryTxId>,
    lbtc_server_claim_tx_id: Option<HistoryTxId>,
    btc_user_lockup_tx_id: Option<HistoryTxId>,
    btc_refund_tx_id: Option<HistoryTxId>,
}
impl PartialSwapState for RecoveredOnchainDataChainReceive {
    fn get_partial_state(&self) -> PaymentState {
        match &self.btc_user_lockup_tx_id {
            Some(_) => match &self.lbtc_server_claim_tx_id {
                Some(_) => PaymentState::Complete,
                None => match &self.btc_refund_tx_id {
                    Some(tx) => match tx.confirmed {
                        true => PaymentState::Failed,
                        false => PaymentState::RefundPending,
                    },
                    None => PaymentState::Created,
                },
            },
            None => PaymentState::Created,
        }
    }
}

pub(crate) struct RecoveredOnchainData {
    send: HashMap<String, RecoveredOnchainDataSend>,
    receive: HashMap<String, RecoveredOnchainDataReceive>,
    chain_send: HashMap<String, RecoveredOnchainDataChainSend>,
    chain_receive: HashMap<String, RecoveredOnchainDataChainReceive>,
}

impl LiquidSdk {
    /// For each swap, recovers data from chain services.
    ///
    /// The returned data include txs and the partial swap state. This is a partial state, because
    /// certain swap states cannot be determined based on initial and onchain data (e.g. [PaymentState::TimedOut])
    ///
    /// The caller is expected to merge this data with any other data available, then persist the
    /// reconstructed swap.
    pub(crate) async fn recover_from_onchain(&self, tx_map: TxMap) -> Result<RecoveredOnchainData> {
        // Immutable DB, as fetched from endpoint
        let imm_db = self.get_swaps_list().await?;

        // Recovered onchain data (txs) per swap
        let recovered = self.get_onchain_data(tx_map, &imm_db).await?;

        // Validation
        // Checks if recovered data (txs, partial state) matches known data
        for send_swap_id in imm_db.send_swap_immutable_db_by_swap_id.keys() {
            let full_swap = self
                .persister
                .fetch_send_swap_by_id(send_swap_id)
                .ok()
                .flatten();
            let recovered_data = recovered.send.get(send_swap_id);

            info!("[Restore Send] Validating {send_swap_id}");
            match (full_swap, recovered_data) {
                (Some(expected), Some(recovered)) => {
                    let exp_lockup_tx_id = expected.lockup_tx_id;
                    let rec_lockup_tx_id = recovered.lockup_tx_id.as_ref().map(|h| h.txid.to_hex());
                    info!("lockup_tx_id: {exp_lockup_tx_id:?} / {rec_lockup_tx_id:?}");

                    let exp_refund_tx_id = expected.refund_tx_id;
                    let rec_refund_tx_id = recovered.refund_tx_id.as_ref().map(|h| h.txid.to_hex());
                    info!("refund_tx_id: {exp_refund_tx_id:?} / {rec_refund_tx_id:?}");

                    let exp_state = expected.state;
                    let rec_state = recovered.get_partial_state();
                    info!("state: {exp_state:?} / {rec_state:?}");
                }
                (Some(_), None) => error!("No recovered data for Send Swap {send_swap_id}"),
                (None, Some(_)) => error!("No expected data for Send Swap {send_swap_id}"),
                (None, None) => error!("Unknown Send Swap {send_swap_id}"),
            }
            info!("[Restore Send] Validated {send_swap_id}");
        }
        for receive_swap_id in imm_db.receive_swap_immutable_db_by_swap_id_.keys() {
            let full_swap = self
                .persister
                .fetch_receive_swap_by_id(receive_swap_id)
                .ok()
                .flatten();
            let recovered_data = recovered.receive.get(receive_swap_id);

            info!("[Restore Receive] Validating {receive_swap_id}");
            match (full_swap, recovered_data) {
                (Some(expected), Some(recovered)) => {
                    let exp_claim_tx_id = expected.claim_tx_id;
                    let rec_claim_tx_id = recovered.claim_tx_id.as_ref().map(|h| h.txid.to_hex());
                    info!("claim_tx_id: {exp_claim_tx_id:?} / {rec_claim_tx_id:?}");

                    let exp_state = expected.state;
                    let rec_state = recovered.get_partial_state();
                    info!("state: {exp_state:?} / {rec_state:?}");
                }
                (Some(_), None) => error!("No recovered data for Receive Swap {receive_swap_id}"),
                (None, Some(_)) => error!("No expected data for Receive Swap {receive_swap_id}"),
                (None, None) => error!("Unknown Receive Swap {receive_swap_id}"),
            }
            info!("[Restore Receive] Validated {receive_swap_id}");
        }
        for send_chain_swap_id in imm_db.send_chain_swap_immutable_db_by_swap_id.keys() {
            let full_swap = self
                .persister
                .fetch_chain_swap_by_id(send_chain_swap_id)
                .ok()
                .flatten();
            let recovered_data = recovered.chain_send.get(send_chain_swap_id);

            info!("[Restore Chain Send] Validating {send_chain_swap_id}");
            match (full_swap, recovered_data) {
                (Some(expected), Some(recovered)) => {
                    let exp_lbtc_user_lockup_tx_id = expected.user_lockup_tx_id;
                    let rec_lbtc_user_lockup_tx_id = recovered
                        .lbtc_user_lockup_tx_id
                        .as_ref()
                        .map(|h| h.txid.to_hex());
                    info!("lbtc_user_lockup_tx_id: {exp_lbtc_user_lockup_tx_id:?} / {rec_lbtc_user_lockup_tx_id:?}");

                    let exp_lbtc_refund_tx_id = expected.refund_tx_id;
                    let rec_lbtc_refund_tx_id = recovered
                        .lbtc_refund_tx_id
                        .as_ref()
                        .map(|h| h.txid.to_hex());
                    info!(
                        "lbtc_refund_tx_id: {exp_lbtc_refund_tx_id:?} / {rec_lbtc_refund_tx_id:?}"
                    );

                    let exp_btc_server_lockup_tx_id = expected.server_lockup_tx_id;
                    let rec_btc_server_lockup_tx_id = recovered
                        .btc_server_lockup_tx_id
                        .as_ref()
                        .map(|h| h.txid.to_hex());
                    info!("btc_server_lockup_tx_id: {exp_btc_server_lockup_tx_id:?} / {rec_btc_server_lockup_tx_id:?}");

                    let exp_btc_claim_tx_id = expected.claim_tx_id;
                    let rec_btc_claim_tx_id =
                        recovered.btc_claim_tx_id.as_ref().map(|h| h.txid.to_hex());
                    info!("btc_claim_tx_id: {exp_btc_claim_tx_id:?} / {rec_btc_claim_tx_id:?}");

                    let exp_state = expected.state;
                    let rec_state = recovered.get_partial_state();
                    info!("state: {exp_state:?} / {rec_state:?}");
                }
                (Some(_), None) => {
                    error!("No recovered data for Send Chain Swap {send_chain_swap_id}")
                }
                (None, Some(_)) => {
                    error!("No expected data for Send Chain Swap {send_chain_swap_id}")
                }
                (None, None) => error!("Unknown Send Chain Swap {send_chain_swap_id}"),
            }
            info!("[Restore Chain Send] Validated {send_chain_swap_id}");
        }
        for receive_chain_swap_id in imm_db.receive_chain_swap_immutable_db_by_swap_id.keys() {
            let full_swap = self
                .persister
                .fetch_chain_swap_by_id(receive_chain_swap_id)
                .ok()
                .flatten();
            let recovered_data = recovered.chain_receive.get(receive_chain_swap_id);

            info!("[Restore Chain Receive] Validating {receive_chain_swap_id}");
            match (full_swap, recovered_data) {
                (Some(expected), Some(recovered)) => {
                    let exp_lbtc_server_lockup_tx_id = expected.server_lockup_tx_id;
                    let rec_lbtc_server_lockup_tx_id = recovered
                        .lbtc_server_lockup_tx_id
                        .as_ref()
                        .map(|h| h.txid.to_hex());
                    info!("lbtc_server_lockup_tx_id: {exp_lbtc_server_lockup_tx_id:?} / {rec_lbtc_server_lockup_tx_id:?}");

                    let exp_lbtc_server_claim_tx_id = expected.claim_tx_id;
                    let rec_lbtc_server_claim_tx_id = recovered
                        .lbtc_server_claim_tx_id
                        .as_ref()
                        .map(|h| h.txid.to_hex());
                    info!("lbtc_server_claim_tx_id: {exp_lbtc_server_claim_tx_id:?} / {rec_lbtc_server_claim_tx_id:?}");

                    let exp_btc_user_lockup_tx_id = expected.user_lockup_tx_id;
                    let rec_btc_user_lockup_tx_id = recovered
                        .btc_user_lockup_tx_id
                        .as_ref()
                        .map(|h| h.txid.to_hex());
                    info!("btc_user_lockup_tx_id: {exp_btc_user_lockup_tx_id:?} / {rec_btc_user_lockup_tx_id:?}");

                    let exp_btc_refund_tx_id = expected.refund_tx_id;
                    let rec_btc_refund_tx_id =
                        recovered.btc_refund_tx_id.as_ref().map(|h| h.txid.to_hex());
                    info!("btc_refund_tx_id: {exp_btc_refund_tx_id:?} / {rec_btc_refund_tx_id:?}");

                    let exp_state = expected.state;
                    let rec_state = recovered.get_partial_state();
                    info!("state: {exp_state:?} / {rec_state:?}");
                }
                (Some(_), None) => {
                    error!("No recovered data for Receive Chain Swap {receive_chain_swap_id}")
                }
                (None, Some(_)) => {
                    error!("No expected data for Receive Chain Swap {receive_chain_swap_id}")
                }
                (None, None) => error!("Unknown Receive Chain Swap {receive_chain_swap_id}"),
            }
            info!("[Restore Chain Receive] Validated {receive_chain_swap_id}");
        }

        Ok(recovered)
    }

    pub(crate) async fn get_onchain_data(
        &self,
        tx_map: TxMap,
        immutable_db: &SwapsList,
    ) -> Result<RecoveredOnchainData> {
        let histories = self.fetch_swaps_histories(immutable_db).await?;

        let recovered_send_data = self
            .recover_send_swap_tx_ids(&tx_map, histories.send)
            .await?;
        let recovered_receive_data = self
            .recover_receive_swap_tx_ids(&tx_map, histories.receive)
            .await?;
        let recovered_chain_send_data = self
            .recover_send_chain_swap_tx_ids(
                &tx_map,
                histories.send_chain,
                &immutable_db.send_chain_swap_immutable_db_by_swap_id,
            )
            .await?;
        let recovered_chain_receive_data = self
            .recover_receive_chain_swap_tx_ids(
                &tx_map,
                histories.receive_chain,
                &immutable_db.receive_chain_swap_immutable_db_by_swap_id,
            )
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
        send_histories_by_swap_id: HashMap<String, SendSwapHistory>,
    ) -> Result<HashMap<String, RecoveredOnchainDataSend>> {
        let mut res: HashMap<String, RecoveredOnchainDataSend> = HashMap::new();
        for (swap_id, history) in send_histories_by_swap_id {
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

    /// Reconstruct Receive Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_receive_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        receive_histories_by_swap_id: HashMap<String, ReceiveSwapHistory>,
    ) -> Result<HashMap<String, RecoveredOnchainDataReceive>> {
        let mut res: HashMap<String, RecoveredOnchainDataReceive> = HashMap::new();
        for (swap_id, history) in receive_histories_by_swap_id {
            let (lockup_tx_id, claim_tx_id) = match history.len() {
                2 => {
                    let first = history[0].clone();
                    let second = history[1].clone();

                    // If a history tx is a known incoming tx, it's the claim tx
                    match tx_map.incoming_tx_map.contains_key::<Txid>(&first.txid) {
                        true => (Some(second), Some(first)),
                        false => (Some(first), Some(second)),
                    }
                }
                n => {
                    error!("Script history with unexpected length {n} found while recovering data for Receive Swap {swap_id}");
                    (None, None)
                }
            };

            res.insert(
                swap_id,
                RecoveredOnchainDataReceive {
                    lockup_tx_id,
                    claim_tx_id,
                },
            );
        }

        Ok(res)
    }

    /// Reconstruct Chain Send Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_send_chain_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        chain_send_histories_by_swap_id: HashMap<String, SendChainSwapHistory>,
        send_chain_swap_immutable_db_by_swap_id: &HashMap<String, SendChainSwapImmutableData>,
    ) -> Result<HashMap<String, RecoveredOnchainDataChainSend>> {
        let mut res: HashMap<String, RecoveredOnchainDataChainSend> = HashMap::new();
        for (swap_id, history) in chain_send_histories_by_swap_id {
            info!("[Recover Chain Send] Checking swap {swap_id}");

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

                    let btc_lockup_script = send_chain_swap_immutable_db_by_swap_id
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
                    error!("BTC script history with unexpected length {n} found while recovering data for Chain Send Swap {swap_id}");
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

    /// Reconstruct Chain Receive Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_receive_chain_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        chain_receive_histories_by_swap_id: HashMap<String, ReceiveChainSwapHistory>,
        receive_chain_swap_immutable_db_by_swap_id: &HashMap<String, ReceiveChainSwapImmutableData>,
    ) -> Result<HashMap<String, RecoveredOnchainDataChainReceive>> {
        let mut res: HashMap<String, RecoveredOnchainDataChainReceive> = HashMap::new();
        for (swap_id, history) in chain_receive_histories_by_swap_id {
            info!("[Recover Chain Receive] Checking swap {swap_id}");

            let (lbtc_server_lockup_tx_id, lbtc_server_claim_tx_id) = match history
                .lbtc_claim_script_history
                .len()
            {
                // Only lockup tx available
                1 => (Some(history.lbtc_claim_script_history[0].clone()), None),

                2 => {
                    let first = &history.lbtc_claim_script_history[0];
                    let second = &history.lbtc_claim_script_history[1];

                    // If a history tx is a known incoming tx, it's the claim tx
                    let (lockup_tx_id, claim_tx_id) =
                        match tx_map.incoming_tx_map.contains_key::<Txid>(&first.txid) {
                            true => (second, first),
                            false => (first, second),
                        };
                    (Some(lockup_tx_id.clone()), Some(claim_tx_id.clone()))
                }
                n => {
                    error!("L-BTC script history with unexpected length {n} found while recovering data for Chain Receive Swap {swap_id}");
                    (None, None)
                }
            };

            let (btc_user_lockup_tx_id, btc_refund_tx_id) = match history
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

                    let btc_lockup_script = receive_chain_swap_immutable_db_by_swap_id
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

                    // The btc_lockup_script_history can contain 3 kinds of txs, of which only 2 are expected:
                    // - 1) btc_user_lockup_tx_id (initial BTC funds sent by the sender)
                    // - 2A) btc_server_claim_tx_id (the swapper tx that claims the BTC funds, in Success case)
                    // - 2B) btc_refund_tx_id (refund tx we initiate, in Failure case)
                    // TODO How to tell the BTC server claim (2A) apart from the BTC refund (2B)?
                    match is_first_tx_lockup_tx {
                        true => (Some(first_tx_id), Some(second_tx_id)),
                        false => (Some(second_tx_id), Some(first_tx_id)),
                    }
                }
                n => {
                    error!("BTC script history with unexpected length {n} found while recovering data for Chain Receive Swap {swap_id}");
                    (None, None)
                }
            };

            res.insert(
                swap_id,
                RecoveredOnchainDataChainReceive {
                    lbtc_server_lockup_tx_id,
                    lbtc_server_claim_tx_id,
                    btc_user_lockup_tx_id,
                    btc_refund_tx_id,
                },
            );
        }

        Ok(res)
    }
}

/// Methods to simulate the immutable DB data available from real-time sync
// TODO Remove once real-time sync is integrated
pub(crate) mod immutable {
    use std::collections::HashMap;

    use anyhow::{anyhow, ensure, Result};
    use boltz_client::{BtcSwapScript, LBtcSwapScript};
    use log::{error, info};
    use lwk_wollet::elements::Txid;
    use lwk_wollet::History;

    use crate::prelude::*;
    use crate::sdk::LiquidSdk;

    type BtcScript = lwk_wollet::bitcoin::ScriptBuf;
    type LBtcScript = lwk_wollet::elements::Script;

    pub(crate) type SendSwapHistory = Vec<HistoryTxId>;
    pub(crate) type ReceiveSwapHistory = Vec<HistoryTxId>;

    #[derive(Clone)]
    pub(crate) struct HistoryTxId {
        pub txid: Txid,
        pub confirmed: bool,
    }
    impl From<History> for HistoryTxId {
        fn from(value: History) -> Self {
            Self::from(&value)
        }
    }
    impl From<&History> for HistoryTxId {
        fn from(value: &History) -> Self {
            Self {
                txid: value.txid,
                confirmed: value.height > 0,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub(crate) struct SendSwapImmutableData {
        pub(crate) swap_id: String,
        pub(crate) swap_script: LBtcSwapScript,
        pub(crate) script: LBtcScript,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub(crate) struct ReceiveSwapImmutableData {
        pub(crate) swap_id: String,
        pub(crate) swap_script: LBtcSwapScript,
        pub(crate) script: LBtcScript,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub(crate) struct SendChainSwapImmutableData {
        swap_id: String,
        lockup_swap_script: LBtcSwapScript,
        lockup_script: LBtcScript,
        claim_swap_script: BtcSwapScript,
        pub(crate) claim_script: BtcScript,
    }

    pub(crate) struct SendChainSwapHistory {
        pub(crate) lbtc_lockup_script_history: Vec<HistoryTxId>,
        pub(crate) btc_claim_script_history: Vec<HistoryTxId>,
        pub(crate) btc_claim_script_txs: Vec<boltz_client::bitcoin::Transaction>,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub(crate) struct ReceiveChainSwapImmutableData {
        swap_id: String,
        lockup_swap_script: BtcSwapScript,
        pub(crate) lockup_script: BtcScript,
        claim_swap_script: LBtcSwapScript,
        claim_script: LBtcScript,
    }

    pub(crate) struct ReceiveChainSwapHistory {
        pub(crate) lbtc_claim_script_history: Vec<HistoryTxId>,
        pub(crate) btc_lockup_script_history: Vec<HistoryTxId>,
        pub(crate) btc_lockup_script_txs: Vec<boltz_client::bitcoin::Transaction>,
    }

    /// Swap data received from the immutable DB
    pub(crate) struct SwapsList {
        pub(crate) send_swap_immutable_db_by_swap_id: HashMap<String, SendSwapImmutableData>,
        pub(crate) receive_swap_immutable_db_by_swap_id_: HashMap<String, ReceiveSwapImmutableData>,
        pub(crate) send_chain_swap_immutable_db_by_swap_id:
            HashMap<String, SendChainSwapImmutableData>,
        pub(crate) receive_chain_swap_immutable_db_by_swap_id:
            HashMap<String, ReceiveChainSwapImmutableData>,
    }

    impl SwapsList {
        fn init(
            send_swaps: Vec<SendSwap>,
            receive_swaps: Vec<ReceiveSwap>,
            send_chain_swaps: Vec<ChainSwap>,
            receive_chain_swaps: Vec<ChainSwap>,
        ) -> Result<Self> {
            let send_swap_immutable_db_by_swap_id: HashMap<String, SendSwapImmutableData> =
                send_swaps
                    .iter()
                    .filter_map(|swap| match swap.get_swap_script() {
                        Ok(swap_script) => match &swap_script.funding_addrs {
                            Some(address) => Some((
                                swap.id.clone(),
                                SendSwapImmutableData {
                                    swap_id: swap.id.clone(),
                                    swap_script: swap_script.clone(),
                                    script: address.script_pubkey(),
                                },
                            )),
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
                        let swap_id = &swap.id;

                        let swap_script = swap
                            .get_swap_script()
                            .map_err(|e| {
                                error!("Failed to get swap script for Receive Swap {swap_id}: {e}")
                            })
                            .ok()?;

                        match &swap_script.funding_addrs {
                            Some(address) => Some((
                                swap.id.clone(),
                                ReceiveSwapImmutableData {
                                    swap_id: swap.id.clone(),
                                    swap_script: swap_script.clone(),
                                    script: address.script_pubkey(),
                                },
                            )),
                            None => {
                                error!("No funding address found for Receive Swap {}", swap.id);
                                None
                            }
                        }
                    })
                    .collect();
            let receive_swap_immutable_db_size = receive_swap_immutable_db_by_swap_id_.len();
            info!("Receive Swap immutable DB: {receive_swap_immutable_db_size} rows");

            let send_chain_swap_immutable_db_by_swap_id: HashMap<String, SendChainSwapImmutableData> =
                send_chain_swaps.iter().filter_map(|swap| {
                    let swap_id = &swap.id;

                    let lockup_swap_script = swap.get_lockup_swap_script()
                        .map_err(|e| error!("Failed to get lockup swap script for swap {swap_id}: {e}"))
                        .map(|s| s.as_liquid_script().ok())
                        .ok()
                        .flatten()?;
                    let claim_swap_script = swap.get_claim_swap_script()
                        .map_err(|e| error!("Failed to get claim swap script for swap {swap_id}: {e}"))
                        .map(|s| s.as_bitcoin_script().ok()).ok().flatten()?;

                    let maybe_lockup_script = lockup_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());
                    let maybe_claim_script = claim_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());

                    match (maybe_lockup_script, maybe_claim_script) {
                        (Some(lockup_script), Some(claim_script)) => {
                            Some((swap.id.clone(), SendChainSwapImmutableData {
                                swap_id: swap.id.clone(),
                                lockup_swap_script,
                                lockup_script,
                                claim_swap_script,
                                claim_script,
                            }))
                        }
                        (lockup_script, claim_script) => {
                            error!("Failed to get lockup or claim script for swap {swap_id}. Lockup script: {lockup_script:?}. Claim script: {claim_script:?}");
                            None
                        }
                    }
                })
                .collect();
            let send_chain_swap_immutable_db_size = send_chain_swap_immutable_db_by_swap_id.len();
            info!("Send Chain Swap immutable DB: {send_chain_swap_immutable_db_size} rows");

            let receive_chain_swap_immutable_db_by_swap_id: HashMap<String, ReceiveChainSwapImmutableData> =
                receive_chain_swaps.iter().filter_map(|swap| {
                    let swap_id = &swap.id;

                    let lockup_swap_script = swap.get_lockup_swap_script()
                        .map_err(|e| error!("Failed to get lockup swap script for swap {swap_id}: {e}"))
                        .map(|s| s.as_bitcoin_script().ok()).ok().flatten()?;
                    let claim_swap_script = swap.get_claim_swap_script()
                        .map_err(|e| error!("Failed to get claim swap script for swap {swap_id}: {e}"))
                        .map(|s| s.as_liquid_script().ok()).ok().flatten()?;

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
                        (lockup_script, claim_script) => {
                            error!("Failed to get lockup or claim script for swap {swap_id}. Lockup script: {lockup_script:?}. Claim script: {claim_script:?}");
                            None
                        }
                    }
                })
                .collect();
            let receive_chain_swap_immutable_db_size =
                receive_chain_swap_immutable_db_by_swap_id.len();
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

        fn send_histories_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
        ) -> HashMap<String, SendSwapHistory> {
            let send_swaps_by_script = self.send_swaps_by_script();

            let mut data: HashMap<String, SendSwapHistory> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script, lbtc_script_history)| {
                    if let Some(imm) = send_swaps_by_script.get(lbtc_script) {
                        data.insert(imm.swap_id.clone(), lbtc_script_history.clone());
                    }
                });
            data
        }

        fn receive_swaps_by_script(&self) -> HashMap<LBtcScript, ReceiveSwapImmutableData> {
            self.receive_swap_immutable_db_by_swap_id_
                .clone()
                .into_values()
                .map(|imm| (imm.script.clone(), imm))
                .collect()
        }

        fn receive_histories_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
        ) -> HashMap<String, ReceiveSwapHistory> {
            let receive_swaps_by_script = self.receive_swaps_by_script();

            let mut data: HashMap<String, ReceiveSwapHistory> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script, lbtc_script_history)| {
                    if let Some(imm) = receive_swaps_by_script.get(lbtc_script) {
                        data.insert(imm.swap_id.clone(), lbtc_script_history.clone());
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

        fn send_chain_histories_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
            btc_script_to_history_map: &HashMap<BtcScript, Vec<HistoryTxId>>,
            btc_script_to_txs_map: &HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>>,
        ) -> HashMap<String, SendChainSwapHistory> {
            let send_chain_swaps_by_lbtc_script = self.send_chain_swaps_by_lbtc_lockup_script();

            let mut data: HashMap<String, SendChainSwapHistory> = HashMap::new();
            lbtc_script_to_history_map.iter().for_each(
                |(lbtc_lockup_script, lbtc_script_history)| {
                    if let Some(imm) = send_chain_swaps_by_lbtc_script.get(lbtc_lockup_script) {
                        let btc_script_history = btc_script_to_history_map
                            .get(&imm.claim_script)
                            .cloned()
                            .unwrap_or_default();
                        let btc_script_txs = btc_script_to_txs_map
                            .get(&imm.claim_script)
                            .cloned()
                            .unwrap_or_default();

                        data.insert(
                            imm.swap_id.clone(),
                            SendChainSwapHistory {
                                lbtc_lockup_script_history: lbtc_script_history.clone(),
                                btc_claim_script_history: btc_script_history,
                                btc_claim_script_txs: btc_script_txs,
                            },
                        );
                    }
                },
            );
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

        fn receive_chain_histories_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
            btc_script_to_history_map: &HashMap<BtcScript, Vec<HistoryTxId>>,
            btc_script_to_txs_map: &HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>>,
        ) -> HashMap<String, ReceiveChainSwapHistory> {
            let receive_chain_swaps_by_lbtc_script =
                self.receive_chain_swaps_by_lbtc_claim_script();

            let mut data: HashMap<String, ReceiveChainSwapHistory> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script_pk, lbtc_script_history)| {
                    if let Some(imm) = receive_chain_swaps_by_lbtc_script.get(lbtc_script_pk) {
                        let btc_script_history = btc_script_to_history_map
                            .get(&imm.lockup_script)
                            .cloned()
                            .unwrap_or_default();
                        let btc_script_txs = btc_script_to_txs_map
                            .get(&imm.lockup_script)
                            .cloned()
                            .unwrap_or_default();

                        data.insert(
                            imm.swap_id.clone(),
                            ReceiveChainSwapHistory {
                                lbtc_claim_script_history: lbtc_script_history.clone(),
                                btc_lockup_script_history: btc_script_history,
                                btc_lockup_script_txs: btc_script_txs,
                            },
                        );
                    }
                });
            data
        }

        fn get_all_swap_lbtc_scripts(&self) -> Vec<LBtcScript> {
            let send_swap_scripts: Vec<LBtcScript> = self
                .send_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.script)
                .collect();
            let receive_swap_scripts: Vec<LBtcScript> = self
                .receive_swap_immutable_db_by_swap_id_
                .clone()
                .into_values()
                .map(|imm| imm.script)
                .collect();
            let send_chain_swap_lbtc_lockup_scripts: Vec<LBtcScript> = self
                .send_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.lockup_script)
                .collect();
            let receive_chain_swap_lbtc_claim_scripts: Vec<LBtcScript> = self
                .receive_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.claim_script)
                .collect();

            let mut swap_scripts = send_swap_scripts.clone();
            swap_scripts.extend(receive_swap_scripts.clone());
            swap_scripts.extend(send_chain_swap_lbtc_lockup_scripts.clone());
            swap_scripts.extend(receive_chain_swap_lbtc_claim_scripts.clone());
            swap_scripts
        }

        fn get_all_swap_btc_scripts(&self) -> Vec<BtcScript> {
            let send_chain_swap_btc_claim_scripts: Vec<BtcScript> = self
                .send_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.claim_script)
                .collect();
            let receive_chain_swap_btc_lockup_scripts: Vec<BtcScript> = self
                .receive_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.lockup_script)
                .collect();

            let mut swap_scripts = send_chain_swap_btc_claim_scripts.clone();
            swap_scripts.extend(receive_chain_swap_btc_lockup_scripts.clone());
            swap_scripts
        }
    }

    pub(crate) struct SwapsHistories {
        pub(crate) send: HashMap<String, SendSwapHistory>,
        pub(crate) receive: HashMap<String, ReceiveSwapHistory>,
        pub(crate) send_chain: HashMap<String, SendChainSwapHistory>,
        pub(crate) receive_chain: HashMap<String, ReceiveChainSwapHistory>,
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

            SwapsList::init(
                send_swaps,
                receive_swaps,
                send_chain_swaps,
                receive_chain_swaps,
            )
        }

        /// For a given [SwapList], this fetches the script histories from the chain services
        pub(crate) async fn fetch_swaps_histories(
            &self,
            swaps_list: &SwapsList,
        ) -> Result<SwapsHistories> {
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
            let lbtc_script_to_history_map: HashMap<LBtcScript, Vec<HistoryTxId>> =
                swap_lbtc_scripts
                    .into_iter()
                    .zip(lbtc_script_histories.into_iter())
                    .map(|(k, v)| (k, v.into_iter().map(HistoryTxId::from).collect()))
                    .collect();

            let swap_btc_scripts = swaps_list.get_all_swap_btc_scripts();
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
    }
}
