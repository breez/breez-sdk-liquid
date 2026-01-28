use std::str::FromStr;

use boltz_client::{
    boltz::SwapTxKind, elements::Transaction, fees::Fee, network::LiquidClient,
    util::secrets::Preimage, ElementsAddress as Address, LBtcSwapTx,
};
use log::info;

use crate::{
    ensure_sdk,
    error::{PaymentError, SdkError},
    prelude::{ChainSwap, Direction, ReceiveSwap, Swap, Utxo, LIQUID_FEE_RATE_SAT_PER_VBYTE},
    utils,
};

use super::{BoltzSwapper, ProxyUrlFetcher};

impl<P: ProxyUrlFetcher> BoltzSwapper<P> {
    pub(crate) fn validate_send_swap_preimage(
        &self,
        swap_id: &str,
        invoice: &str,
        preimage: &str,
    ) -> Result<(), PaymentError> {
        utils::verify_payment_hash(preimage, invoice)?;
        info!("Preimage is valid for Send Swap {swap_id}");
        Ok(())
    }

    pub(crate) async fn new_receive_claim_tx(
        &self,
        swap: &ReceiveSwap,
        claim_address: String,
        is_cooperative: bool,
    ) -> Result<Transaction, PaymentError> {
        let liquid_client = self.get_liquid_client()?;
        let swap_script = swap.get_swap_script()?;

        let claim_tx_wrapper = LBtcSwapTx::new_claim(
            swap_script,
            claim_address,
            liquid_client,
            &self.get_boltz_client().await?.inner,
            swap.id.clone(),
        )
        .await?;

        let cooperative_details = if is_cooperative {
            self.get_cooperative_details(swap.id.clone(), None).await?
        } else {
            None
        };

        let fee = if is_cooperative {
            Fee::Absolute(swap.claim_fees_sat)
        } else {
            Fee::Relative(LIQUID_FEE_RATE_SAT_PER_VBYTE)
        };

        let signed_tx = claim_tx_wrapper
            .sign_claim(
                &swap.get_claim_keypair()?,
                &Preimage::from_str(&swap.preimage)?,
                fee,
                cooperative_details,
                true,
            )
            .await?;

        Ok(signed_tx)
    }

    pub(crate) async fn new_incoming_chain_claim_tx(
        &self,
        swap: &ChainSwap,
        claim_address: String,
        is_cooperative: bool,
    ) -> Result<Transaction, PaymentError> {
        let liquid_client = self.get_liquid_client()?;
        let claim_keypair = swap.get_claim_keypair()?;
        let swap_script = swap.get_claim_swap_script()?.as_liquid_script()?;
        let claim_tx_wrapper = LBtcSwapTx::new_claim(
            swap_script,
            claim_address,
            liquid_client,
            &self.get_boltz_client().await?.inner,
            swap.id.clone(),
        )
        .await?;

        let cooperative_details = if is_cooperative {
            let signature = self.get_claim_partial_sig(swap).await?;
            self.get_cooperative_details(swap.id.clone(), signature)
                .await?
        } else {
            None
        };

        let fee = if is_cooperative {
            Fee::Absolute(swap.claim_fees_sat)
        } else {
            Fee::Relative(LIQUID_FEE_RATE_SAT_PER_VBYTE)
        };

        let signed_tx = claim_tx_wrapper
            .sign_claim(
                &claim_keypair,
                &Preimage::from_str(&swap.preimage)?,
                fee,
                cooperative_details,
                true,
            )
            .await?;

        Ok(signed_tx)
    }

    fn calculate_refund_fees(&self, refund_tx_size: usize) -> u64 {
        (refund_tx_size as f64 * LIQUID_FEE_RATE_SAT_PER_VBYTE).ceil() as u64
    }

    pub(crate) async fn new_lbtc_refund_wrapper(
        &self,
        swap: &Swap,
        refund_address: &str,
    ) -> Result<LBtcSwapTx, SdkError> {
        let liquid_client = self.get_liquid_client()?;
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
                        liquid_client,
                        &self.get_boltz_client().await?.inner,
                        swap.id.clone(),
                    )
                    .await
                }
            },
            Swap::Send(swap) => {
                let swap_script = swap.get_swap_script()?;
                LBtcSwapTx::new_refund(
                    swap_script,
                    refund_address,
                    liquid_client,
                    &self.get_boltz_client().await?.inner,
                    swap.id.clone(),
                )
                .await
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

    pub(crate) async fn new_lbtc_refund_tx(
        &self,
        swap: &Swap,
        refund_address: &str,
        utxos: Vec<Utxo>,
        is_cooperative: bool,
    ) -> Result<Transaction, SdkError> {
        let liquid_client = self.get_liquid_client()?;

        let (swap_script, refund_keypair) = match swap {
            Swap::Chain(swap) => {
                ensure_sdk!(
                    swap.direction == Direction::Outgoing,
                    SdkError::generic("Cannot create LBTC refund tx for incoming Chain swaps")
                );

                (
                    swap.get_lockup_swap_script()?.as_liquid_script()?,
                    swap.get_refund_keypair()?,
                )
            }
            Swap::Send(swap) => (swap.get_swap_script()?, swap.get_refund_keypair()?),
            Swap::Receive(_) => {
                return Err(SdkError::generic(
                    "Cannot create LBTC refund tx for Receive swaps.",
                ));
            }
        };
        let swap_id = swap.id();

        let address = Address::from_str(refund_address)
            .map_err(|err| SdkError::generic(format!("Could not parse address: {err:?}")))?;

        let genesis_hash = liquid_client.get_genesis_hash().await?;

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

        let refund_tx_size = refund_tx.size(&refund_keypair, is_cooperative, true)?;
        let broadcast_fees_sat = self.calculate_refund_fees(refund_tx_size);

        let cooperative = match is_cooperative {
            true => self.get_cooperative_details(swap_id.clone(), None).await?,
            false => None,
        };

        let signed_tx = refund_tx
            .sign_refund(
                &refund_keypair,
                Fee::Absolute(broadcast_fees_sat),
                cooperative,
                true,
            )
            .await?;
        Ok(signed_tx)
    }
}
