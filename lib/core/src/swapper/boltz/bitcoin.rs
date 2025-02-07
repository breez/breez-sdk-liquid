use std::str::FromStr;

use boltz_client::{bitcoin::Transaction, fees::Fee, util::secrets::Preimage, BtcSwapTx};

use crate::{
    error::{PaymentError, SdkError},
    prelude::{ChainSwap, Direction, Swap},
};

use super::{BoltzSwapper, ProxyUrlFetcher};

impl<P: ProxyUrlFetcher> BoltzSwapper<P> {
    pub(crate) async fn new_btc_refund_wrapper(
        &self,
        swap: &Swap,
        refund_address: &str,
    ) -> Result<BtcSwapTx, SdkError> {
        let refund_wrapper = match swap {
            Swap::Chain(swap) => match swap.direction {
                Direction::Incoming => {
                    let swap_script = swap.get_lockup_swap_script()?;
                    BtcSwapTx::new_refund(
                        swap_script.as_bitcoin_script()?,
                        refund_address,
                        &self.bitcoin_electrum_config,
                        self.get_url().await?,
                        swap.id.clone(),
                    )
                }
                Direction::Outgoing => {
                    return Err(SdkError::generic(format!(
                        "Cannot create Bitcoin refund wrapper for outgoing Chain swap {}",
                        swap.id
                    )));
                }
            },
            _ => {
                return Err(SdkError::generic(format!(
                    "Cannot create Bitcoin refund wrapper for swap {}",
                    swap.id()
                )));
            }
        }?;
        Ok(refund_wrapper)
    }

    pub(crate) async fn new_btc_refund_tx(
        &self,
        chain_swap: &ChainSwap,
        refund_address: &str,
        broadcast_fee_rate_sat_per_vb: f64,
        is_cooperative: bool,
    ) -> Result<Transaction, SdkError> {
        let swap = Swap::Chain(chain_swap.clone());
        let refund_tx_wrapper = self.new_btc_refund_wrapper(&swap, refund_address).await?;
        let refund_keypair = chain_swap.get_refund_keypair()?;
        let cooperative = match is_cooperative {
            true => {
                self.get_cooperative_details(chain_swap.id.clone(), None, None)
                    .await?
            }
            false => None,
        };

        let signed_tx = refund_tx_wrapper.sign_refund(
            &refund_keypair,
            Fee::Relative(broadcast_fee_rate_sat_per_vb),
            cooperative,
        )?;
        Ok(signed_tx)
    }

    pub(crate) async fn new_outgoing_chain_claim_tx(
        &self,
        swap: &ChainSwap,
        claim_address: String,
    ) -> Result<Transaction, PaymentError> {
        let claim_keypair = swap.get_claim_keypair()?;
        let claim_swap_script = swap.get_claim_swap_script()?.as_bitcoin_script()?;
        let claim_tx_wrapper = BtcSwapTx::new_claim(
            claim_swap_script,
            claim_address,
            &self.bitcoin_electrum_config,
            self.get_url().await?,
            swap.id.clone(),
        )?;

        let (partial_sig, pub_nonce) = self.get_claim_partial_sig(swap).await?;

        let signed_tx = claim_tx_wrapper.sign_claim(
            &claim_keypair,
            &Preimage::from_str(&swap.preimage)?,
            Fee::Absolute(swap.claim_fees_sat),
            self.get_cooperative_details(swap.id.clone(), Some(pub_nonce), Some(partial_sig))
                .await?,
        )?;

        Ok(signed_tx)
    }
}
