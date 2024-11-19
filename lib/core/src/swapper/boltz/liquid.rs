use std::str::FromStr;

use boltz_client::{
    boltz::SwapTxKind,
    elements::Transaction,
    util::{liquid_genesis_hash, secrets::Preimage},
    Amount, Bolt11Invoice, ElementsAddress as Address, LBtcSwapTx,
};
use log::info;

use crate::{
    ensure_sdk,
    error::{PaymentError, SdkError},
    prelude::{
        ChainSwap, Direction, LiquidNetwork, ReceiveSwap, Swap, Utxo,
        LOWBALL_FEE_RATE_SAT_PER_VBYTE, STANDARD_FEE_RATE_SAT_PER_VBYTE,
    },
};

use super::BoltzSwapper;

impl BoltzSwapper {
    pub(crate) fn validate_send_swap_preimage(
        &self,
        swap_id: &str,
        invoice: &str,
        preimage: &str,
    ) -> Result<(), PaymentError> {
        Self::verify_payment_hash(preimage, invoice)?;
        info!("Preimage is valid for Send Swap {swap_id}");
        Ok(())
    }

    pub(crate) fn verify_payment_hash(preimage: &str, invoice: &str) -> Result<(), PaymentError> {
        let preimage = Preimage::from_str(preimage)?;
        let preimage_hash = preimage.sha256.to_string();

        let invoice_payment_hash = match Bolt11Invoice::from_str(invoice) {
            Ok(invoice) => Ok(invoice.payment_hash().to_string()),
            Err(_) => match crate::utils::parse_bolt12_invoice(invoice) {
                Ok(invoice) => Ok(invoice.payment_hash().to_string()),
                Err(e) => Err(PaymentError::Generic {
                    err: format!("Could not parse invoice: {e:?}"),
                }),
            },
        }?;

        ensure_sdk!(
            invoice_payment_hash == preimage_hash,
            PaymentError::InvalidPreimage
        );

        Ok(())
    }

    pub(crate) fn new_receive_claim_tx(
        &self,
        swap: &ReceiveSwap,
        claim_address: String,
    ) -> Result<Transaction, PaymentError> {
        let swap_script = swap.get_swap_script()?;

        let claim_tx_wrapper = LBtcSwapTx::new_claim(
            swap_script,
            claim_address,
            &self.liquid_electrum_config,
            self.boltz_url.clone(),
            swap.id.clone(),
        )?;

        let signed_tx = claim_tx_wrapper.sign_claim(
            &swap.get_claim_keypair()?,
            &Preimage::from_str(&swap.preimage)?,
            Amount::from_sat(swap.claim_fees_sat),
            self.get_cooperative_details(swap.id.clone(), None, None),
        )?;

        Ok(signed_tx)
    }

    pub(crate) fn new_incoming_chain_claim_tx(
        &self,
        swap: &ChainSwap,
        claim_address: String,
    ) -> Result<Transaction, PaymentError> {
        let claim_keypair = swap.get_claim_keypair()?;
        let swap_script = swap.get_claim_swap_script()?.as_liquid_script()?;
        let claim_tx_wrapper = LBtcSwapTx::new_claim(
            swap_script,
            claim_address,
            &self.liquid_electrum_config,
            self.boltz_url.clone(),
            swap.id.clone(),
        )?;

        let (partial_sig, pub_nonce) = self.get_claim_partial_sig(swap)?;

        let signed_tx = claim_tx_wrapper.sign_claim(
            &claim_keypair,
            &Preimage::from_str(&swap.preimage)?,
            Amount::from_sat(swap.claim_fees_sat),
            self.get_cooperative_details(swap.id.clone(), Some(pub_nonce), Some(partial_sig)),
        )?;

        Ok(signed_tx)
    }

    fn calculate_refund_fees(&self, refund_tx_size: usize) -> u64 {
        let fee_rate = match self.config.network {
            LiquidNetwork::Mainnet => LOWBALL_FEE_RATE_SAT_PER_VBYTE,
            LiquidNetwork::Testnet => STANDARD_FEE_RATE_SAT_PER_VBYTE,
        };
        (refund_tx_size as f64 * fee_rate).ceil() as u64
    }

    pub(crate) fn new_lbtc_refund_wrapper(
        &self,
        swap: &Swap,
        refund_address: &String,
    ) -> Result<LBtcSwapTx, SdkError> {
        let refund_wrapper = match swap {
            Swap::Chain(swap) => match swap.direction {
                Direction::Incoming => {
                    return Err(SdkError::generic(format!(
                        "Cannot create Liquid refund wrapper for incoming Chain swap {}",
                        swap.id
                    )));
                }
                Direction::Outgoing => {
                    let swap_script = swap.get_lockup_swap_script()?;
                    LBtcSwapTx::new_refund(
                        swap_script.as_liquid_script()?,
                        refund_address,
                        &self.liquid_electrum_config,
                        self.boltz_url.clone(),
                        swap.id.clone(),
                    )
                }
            },
            Swap::Send(swap) => {
                let swap_script = swap.get_swap_script()?;
                LBtcSwapTx::new_refund(
                    swap_script,
                    refund_address,
                    &self.liquid_electrum_config,
                    self.boltz_url.clone(),
                    swap.id.clone(),
                )
            }
            Swap::Receive(swap) => {
                return Err(SdkError::generic(format!(
                    "Cannot create Liquid refund wrapper for Receive swap {}",
                    swap.id
                )));
            }
        }?;
        Ok(refund_wrapper)
    }

    pub(crate) fn new_lbtc_refund_tx(
        &self,
        swap: &Swap,
        refund_address: &str,
        utxos: Vec<Utxo>,
        is_cooperative: bool,
    ) -> Result<Transaction, SdkError> {
        let (swap_script, refund_keypair, preimage) = match swap {
            Swap::Chain(swap) => {
                ensure_sdk!(
                    swap.direction == Direction::Outgoing,
                    SdkError::generic("Cannot create LBTC refund tx for incoming Chain swaps")
                );

                (
                    swap.get_lockup_swap_script()?.as_liquid_script()?,
                    swap.get_refund_keypair()?,
                    Preimage::from_str(&swap.preimage)?,
                )
            }
            Swap::Send(swap) => (
                swap.get_swap_script()?,
                swap.get_refund_keypair()?,
                Preimage::new(),
            ),
            Swap::Receive(_) => {
                return Err(SdkError::generic(
                    "Cannot create LBTC refund tx for Receive swaps.",
                ));
            }
        };
        let swap_id = swap.id();

        let address = Address::from_str(refund_address)
            .map_err(|err| SdkError::generic(format!("Could not parse address: {err:?}")))?;

        let genesis_hash = liquid_genesis_hash(&self.liquid_electrum_config)?;

        let (funding_outpoint, funding_tx_out) =
            *utxos
                .first()
                .and_then(|utxo| utxo.as_liquid())
                .ok_or(SdkError::generic("No refundable UTXOs found"))?;

        let refund_tx = LBtcSwapTx {
            kind: SwapTxKind::Refund,
            swap_script,
            output_address: address,
            funding_outpoint,
            funding_utxo: funding_tx_out,
            genesis_hash,
        };

        let refund_tx_size = refund_tx.size(&refund_keypair, &preimage)?;
        let broadcast_fees_sat = self.calculate_refund_fees(refund_tx_size);

        let cooperative = match is_cooperative {
            true => self.get_cooperative_details(swap_id.clone(), None, None),
            false => None,
        };

        let signed_tx = refund_tx.sign_refund(
            &refund_keypair,
            Amount::from_sat(broadcast_fees_sat),
            cooperative,
        )?;
        Ok(signed_tx)
    }
}
