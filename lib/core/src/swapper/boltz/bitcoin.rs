use std::str::FromStr;

use boltz_client::{
    bitcoin::{address::Address, Transaction},
    boltz::SwapTxKind,
    util::secrets::Preimage,
    BtcSwapScript, BtcSwapTx, Keypair,
};

use crate::{
    ensure_sdk,
    error::{PaymentError, SdkError},
    prelude::{ChainSwap, Direction, Swap, Utxo},
};

use super::BoltzSwapper;

impl BoltzSwapper {
    pub(crate) fn new_btc_refund_wrapper(
        &self,
        swap: &Swap,
        refund_address: &String,
    ) -> Result<BtcSwapTx, SdkError> {
        let refund_wrapper = match swap {
            Swap::Chain(swap) => match swap.direction {
                Direction::Incoming => {
                    let swap_script = swap.get_lockup_swap_script()?;
                    // TODO Update boltz-client to build refund tx with all utxos
                    BtcSwapTx::new_refund(
                        swap_script.as_bitcoin_script()?,
                        refund_address,
                        &self.liquid_electrum_config,
                        self.boltz_url.clone(),
                        swap.id.clone(),
                    )
                }
                Direction::Outgoing => {
                    return Err(SdkError::Generic {
                        err: format!(
                            "Cannot create Bitcoin refund wrapper for outgoing Chain swap {}",
                            swap.id
                        ),
                    });
                }
            },
            _ => {
                return Err(SdkError::Generic {
                    err: format!(
                        "Cannot create Bitcoin refund wrapper for swap {}",
                        swap.id()
                    ),
                });
            }
        }?;
        Ok(refund_wrapper)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_btc_refund_tx(
        &self,
        swap_id: String,
        swap_script: BtcSwapScript,
        refund_address: &str,
        refund_keypair: &Keypair,
        utxos: Vec<Utxo>,
        broadcast_fee_rate_sat_per_vb: f64,
        is_cooperative: bool,
    ) -> Result<Transaction, SdkError> {
        let address = Address::from_str(refund_address).map_err(|err| SdkError::Generic {
            err: format!("Could not parse address: {err:?}"),
        })?;

        ensure_sdk!(
            address.is_valid_for_network(self.config.network.into()),
            SdkError::Generic {
                err: "Address network validation failed".to_string()
            }
        );

        let utxo = utxos
            .first()
            .and_then(|utxo| utxo.as_bitcoin().cloned())
            .ok_or(SdkError::Generic {
                err: "No UTXO found".to_string(),
            })?;

        let refund_tx = BtcSwapTx {
            kind: SwapTxKind::Refund,
            swap_script,
            output_address: address.assume_checked(),
            utxo,
        };

        let refund_tx_size = refund_tx.size(refund_keypair, &Preimage::new())?;
        let broadcast_fees_sat = (refund_tx_size as f64 * broadcast_fee_rate_sat_per_vb) as u64;

        let cooperative = match is_cooperative {
            true => self.get_cooperative_details(swap_id, None, None),
            false => None,
        };

        let signed_tx = refund_tx.sign_refund(refund_keypair, broadcast_fees_sat, cooperative)?;
        Ok(signed_tx)
    }

    pub(crate) fn new_outgoing_chain_claim_tx(
        &self,
        swap: &ChainSwap,
    ) -> Result<Transaction, PaymentError> {
        let claim_keypair = swap.get_claim_keypair()?;
        let claim_swap_script = swap.get_claim_swap_script()?.as_bitcoin_script()?;
        let claim_tx_wrapper = BtcSwapTx::new_claim(
            claim_swap_script,
            swap.claim_address.clone(),
            &self.bitcoin_electrum_config,
            self.boltz_url.clone(),
            swap.id.clone(),
        )?;

        let (partial_sig, pub_nonce) = self.get_claim_partial_sig(swap)?;

        let signed_tx = claim_tx_wrapper.sign_claim(
            &claim_keypair,
            &Preimage::from_str(&swap.preimage)?,
            swap.claim_fees_sat,
            self.get_cooperative_details(swap.id.clone(), Some(pub_nonce), Some(partial_sig)),
        )?;

        Ok(signed_tx)
    }
}
