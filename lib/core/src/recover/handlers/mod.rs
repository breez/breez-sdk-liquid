mod handle_chain_receive_swap;
mod handle_chain_send_swap;
mod handle_receive_swap;
mod handle_send_swap;
mod tests;

use lwk_wollet::elements::Txid;

pub(crate) use self::handle_chain_receive_swap::ChainReceiveSwapHandler;
pub(crate) use self::handle_chain_send_swap::ChainSendSwapHandler;
pub(crate) use self::handle_receive_swap::ReceiveSwapHandler;
pub(crate) use self::handle_send_swap::SendSwapHandler;

use super::model::TxMap;
use crate::model::LBtcHistory;

/// Helper function for determining lockup and claim transactions in incoming swaps
pub(crate) fn determine_incoming_lockup_and_claim_txs(
    history: &[LBtcHistory],
    tx_map: &TxMap,
) -> (Option<LBtcHistory>, Option<LBtcHistory>) {
    log::debug!(
        "[determine_lockup_claim] History len={}, incoming_tx_map size={}, outgoing_tx_map size={}",
        history.len(),
        tx_map.incoming_tx_map.len(),
        tx_map.outgoing_tx_map.len()
    );

    for (i, h) in history.iter().enumerate() {
        let in_incoming = tx_map.incoming_tx_map.contains_key(&h.txid);
        let in_outgoing = tx_map.outgoing_tx_map.contains_key(&h.txid);
        log::debug!(
            "[determine_lockup_claim] history[{}]: txid={}, height={}, in_incoming={}, in_outgoing={}",
            i, h.txid, h.height, in_incoming, in_outgoing
        );
    }

    match history.len() {
        // Only lockup tx available
        1 => (Some(history[0].clone()), None),
        2 => {
            let first = history[0].clone();
            let second = history[1].clone();

            if tx_map.incoming_tx_map.contains_key::<Txid>(&first.txid) {
                // If the first tx is a known incoming tx, it's the claim tx and the second is the lockup
                log::debug!(
                    "[determine_lockup_claim] Result: first tx {} is claim (in incoming_tx_map), second tx {} is lockup",
                    first.txid, second.txid
                );
                (Some(second), Some(first))
            } else if tx_map.incoming_tx_map.contains_key::<Txid>(&second.txid) {
                // If the second tx is a known incoming tx, it's the claim tx and the first is the lockup
                log::debug!(
                    "[determine_lockup_claim] Result: second tx {} is claim (in incoming_tx_map), first tx {} is lockup",
                    second.txid, first.txid
                );
                (Some(first), Some(second))
            } else {
                // If none of the 2 txs is the claim tx, then the txs are lockup and swapper refund
                // If so, we expect them to be confirmed at different heights
                log::debug!(
                    "[determine_lockup_claim] Neither tx in incoming_tx_map - assuming lockup + server refund scenario"
                );
                let first_conf_height = first.height;
                let second_conf_height = second.height;
                match (first.confirmed(), second.confirmed()) {
                    // If they're both confirmed, the one with the lowest confirmation height is the lockup
                    (true, true) => {
                        log::debug!(
                            "[determine_lockup_claim] Both confirmed: first height={}, second height={}, returning lockup only (no claim)",
                            first_conf_height, second_conf_height
                        );
                        match first_conf_height < second_conf_height {
                            true => (Some(first), None),
                            false => (Some(second), None),
                        }
                    }

                    // If only one tx is confirmed, then that is the lockup
                    (true, false) => {
                        log::debug!(
                            "[determine_lockup_claim] Only first confirmed, returning lockup only (no claim)"
                        );
                        (Some(first), None)
                    }
                    (false, true) => {
                        log::debug!(
                            "[determine_lockup_claim] Only second confirmed, returning lockup only (no claim)"
                        );
                        (Some(second), None)
                    }

                    // If neither is confirmed, this is an edge-case, and the most likely cause is an
                    // out of date wallet tx_map that doesn't yet include one of the txs.
                    (false, false) => {
                        log::warn!(
                            "Found 2 unconfirmed txs in the claim script history. \
                          Unable to determine if they include a swapper refund or a user claim"
                        );
                        (None, None)
                    }
                }
            }
        }
        n => {
            log::warn!("Unexpected script history length {n} while recovering data for swap");
            (None, None)
        }
    }
}
