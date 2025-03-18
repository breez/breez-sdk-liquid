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
use crate::{elements, model::History};

/// Helper function for determining lockup and claim transactions in incoming swaps
pub(crate) fn determine_incoming_lockup_and_claim_txs(
    history: &[History<elements::Txid>],
    tx_map: &TxMap,
) -> (
    Option<History<elements::Txid>>,
    Option<History<elements::Txid>>,
) {
    match history.len() {
        // Only lockup tx available
        1 => (Some(history[0].clone()), None),
        2 => {
            let first = history[0].clone();
            let second = history[1].clone();

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
