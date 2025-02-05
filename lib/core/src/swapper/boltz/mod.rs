use std::{
    future::Future,
    pin::Pin,
    str::FromStr,
    sync::{Arc, OnceLock},
};

use anyhow::Result;
use async_trait::async_trait;
use boltz_client::{
    boltz::{
        BoltzApiClientV2, ChainPair, Cooperative, CreateChainRequest, CreateChainResponse,
        CreateReverseRequest, CreateReverseResponse, CreateSubmarineRequest,
        CreateSubmarineResponse, ReversePair, SubmarineClaimTxResponse, SubmarinePair,
    },
    elements::secp256k1_zkp::{MusigPartialSignature, MusigPubNonce},
    network::{electrum::ElectrumConfig, Chain},
    util::secrets::Preimage,
    Amount,
};
use log::info;
use url::Url;

use crate::{
    error::{PaymentError, SdkError},
    model::LIQUID_FEE_RATE_SAT_PER_VBYTE,
    prelude::{ChainSwap, Config, Direction, LiquidNetwork, SendSwap, Swap, Transaction, Utxo},
};

use self::status_stream::BoltzStatusStream;
use super::{Swapper, SwapperStatusStream};

pub(crate) mod bitcoin;
pub(crate) mod liquid;
pub mod status_stream;

pub(crate) struct BoltzClient {
    url: String,
    referral_id: Option<String>,
    inner: BoltzApiClientV2,
}

pub(crate) type FetchProxyUrlFn =
    dyn Fn() -> Pin<Box<dyn Future<Output = Result<Option<String>>> + Send>> + Send + Sync;

pub struct BoltzSwapper {
    config: Config,
    client: OnceLock<BoltzClient>,
    liquid_electrum_config: ElectrumConfig,
    bitcoin_electrum_config: ElectrumConfig,
    fetch_proxy_url: Arc<FetchProxyUrlFn>,
}

impl BoltzSwapper {
    pub fn new(config: Config, fetch_proxy_url: Arc<FetchProxyUrlFn>) -> Self {
        Self {
            fetch_proxy_url,
            client: OnceLock::new(),
            config: config.clone(),
            liquid_electrum_config: ElectrumConfig::new(
                config.network.into(),
                &config.liquid_electrum_url,
                true,
                true,
                100,
            ),
            bitcoin_electrum_config: ElectrumConfig::new(
                config.network.as_bitcoin_chain(),
                &config.bitcoin_electrum_url,
                true,
                true,
                100,
            ),
        }
    }

    async fn get_client(&self) -> Result<&BoltzClient> {
        if let Some(client) = self.client.get() {
            return Ok(client);
        }

        let (boltz_api_base_url, referral_id) = match &self.config.network {
            LiquidNetwork::Testnet => (None, None),
            LiquidNetwork::Mainnet => match (self.fetch_proxy_url)().await {
                Ok(Some(swapper_proxy_url)) => Url::parse(&swapper_proxy_url)
                    .map(|url| match url.query() {
                        None => (None, None),
                        Some(query) => {
                            let api_base_url =
                                url.domain().map(|domain| format!("https://{domain}/v2"));
                            let parts: Vec<String> = query.split('=').map(Into::into).collect();
                            let referral_id = parts.get(1).cloned();
                            (api_base_url, referral_id)
                        }
                    })
                    .unwrap_or_default(),
                _ => (None, None),
            },
        };

        let boltz_url = boltz_api_base_url.unwrap_or(self.config.default_boltz_url().to_string());

        let client = self.client.get_or_init(|| BoltzClient {
            inner: BoltzApiClientV2::new(&boltz_url),
            url: boltz_url,
            referral_id,
        });
        Ok(client)
    }

    async fn get_url(&self) -> Result<String> {
        Ok(self.get_client().await?.url.clone())
    }

    async fn get_claim_partial_sig(
        &self,
        swap: &ChainSwap,
    ) -> Result<(MusigPartialSignature, MusigPubNonce), PaymentError> {
        let refund_keypair = swap.get_refund_keypair()?;

        // Create a temporary refund tx to an address from the swap lockup chain
        // We need it to calculate the musig partial sig for the claim tx from the other chain
        let lockup_address = &swap.lockup_address;

        let claim_tx_details = self
            .get_client()
            .await?
            .inner
            .get_chain_claim_tx_details(&swap.id)?;
        match swap.direction {
            Direction::Incoming => {
                let refund_tx_wrapper = self
                    .new_btc_refund_wrapper(&Swap::Chain(swap.clone()), lockup_address)
                    .await?;

                refund_tx_wrapper.partial_sign(
                    &refund_keypair,
                    &claim_tx_details.pub_nonce,
                    &claim_tx_details.transaction_hash,
                )
            }
            Direction::Outgoing => {
                let refund_tx_wrapper = self
                    .new_lbtc_refund_wrapper(&Swap::Chain(swap.clone()), lockup_address)
                    .await?;

                refund_tx_wrapper.partial_sign(
                    &refund_keypair,
                    &claim_tx_details.pub_nonce,
                    &claim_tx_details.transaction_hash,
                )
            }
        }
        .map_err(Into::into)
    }

    async fn get_cooperative_details(
        &self,
        swap_id: String,
        pub_nonce: Option<MusigPubNonce>,
        partial_sig: Option<MusigPartialSignature>,
    ) -> Result<Option<Cooperative>> {
        Ok(Some(Cooperative {
            boltz_api: &self.get_client().await?.inner,
            swap_id,
            pub_nonce,
            partial_sig,
        }))
    }
}

#[async_trait]
impl Swapper for BoltzSwapper {
    /// Create a new chain swap
    async fn create_chain_swap(
        &self,
        req: CreateChainRequest,
    ) -> Result<CreateChainResponse, PaymentError> {
        let client = self.get_client().await?;
        let modified_req = CreateChainRequest {
            referral_id: client.referral_id.clone(),
            ..req.clone()
        };
        Ok(client.inner.post_chain_req(modified_req)?)
    }

    /// Create a new send swap
    async fn create_send_swap(
        &self,
        req: CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError> {
        let client = self.get_client().await?;
        let modified_req = CreateSubmarineRequest {
            referral_id: client.referral_id.clone(),
            ..req.clone()
        };
        Ok(client.inner.post_swap_req(&modified_req)?)
    }

    async fn get_chain_pair(
        &self,
        direction: Direction,
    ) -> Result<Option<ChainPair>, PaymentError> {
        let pairs = self.get_client().await?.inner.get_chain_pairs()?;
        let pair = match direction {
            Direction::Incoming => pairs.get_btc_to_lbtc_pair(),
            Direction::Outgoing => pairs.get_lbtc_to_btc_pair(),
        };
        Ok(pair)
    }

    async fn get_chain_pairs(
        &self,
    ) -> Result<(Option<ChainPair>, Option<ChainPair>), PaymentError> {
        let pairs = self.get_client().await?.inner.get_chain_pairs()?;
        let pair_outgoing = pairs.get_lbtc_to_btc_pair();
        let pair_incoming = pairs.get_btc_to_lbtc_pair();
        Ok((pair_outgoing, pair_incoming))
    }

    async fn get_zero_amount_chain_swap_quote(&self, swap_id: &str) -> Result<Amount, SdkError> {
        self.get_client()
            .await?
            .inner
            .get_quote(swap_id)
            .map(|r| Amount::from_sat(r.amount))
            .map_err(Into::into)
    }

    async fn accept_zero_amount_chain_swap_quote(
        &self,
        swap_id: &str,
        server_lockup_sat: u64,
    ) -> Result<(), PaymentError> {
        self.get_client()
            .await?
            .inner
            .accept_quote(swap_id, server_lockup_sat)
            .map_err(Into::into)
    }

    /// Get a submarine pair information
    async fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError> {
        Ok(self
            .get_client()
            .await?
            .inner
            .get_submarine_pairs()?
            .get_lbtc_to_btc_pair())
    }

    /// Get a submarine swap's preimage
    async fn get_submarine_preimage(&self, swap_id: &str) -> Result<String, PaymentError> {
        Ok(self
            .get_client()
            .await?
            .inner
            .get_submarine_preimage(swap_id)?
            .preimage)
    }

    /// Get claim tx details which includes the preimage as a proof of payment.
    /// It is used to validate the preimage before claiming which is the reason why we need to separate
    /// the claim into two steps.
    async fn get_send_claim_tx_details(
        &self,
        swap: &SendSwap,
    ) -> Result<SubmarineClaimTxResponse, PaymentError> {
        let claim_tx_response = self
            .get_client()
            .await?
            .inner
            .get_submarine_claim_tx_details(&swap.id)?;
        info!("Received claim tx details: {:?}", &claim_tx_response);

        self.validate_send_swap_preimage(&swap.id, &swap.invoice, &claim_tx_response.preimage)?;
        Ok(claim_tx_response)
    }

    /// Claim send swap cooperatively. Here the remote swapper is the one that claims.
    /// We are helping to use key spend path for cheaper fees.
    async fn claim_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        claim_tx_response: SubmarineClaimTxResponse,
        refund_address: &str,
    ) -> Result<(), PaymentError> {
        let swap_id = &swap.id;
        let keypair = swap.get_refund_keypair()?;
        let refund_tx_wrapper = self
            .new_lbtc_refund_wrapper(&Swap::Send(swap.clone()), refund_address)
            .await?;

        self.validate_send_swap_preimage(swap_id, &swap.invoice, &claim_tx_response.preimage)?;

        let (partial_sig, pub_nonce) = refund_tx_wrapper.partial_sign(
            &keypair,
            &claim_tx_response.pub_nonce,
            &claim_tx_response.transaction_hash,
        )?;

        self.get_client()
            .await?
            .inner
            .post_submarine_claim_tx_details(&swap_id.to_string(), pub_nonce, partial_sig)?;
        info!("Successfully sent claim details for swap-in {swap_id}");
        Ok(())
    }

    // Create a new receive swap
    async fn create_receive_swap(
        &self,
        req: CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError> {
        let client = self.get_client().await?;
        let modified_req = CreateReverseRequest {
            referral_id: client.referral_id.clone(),
            ..req.clone()
        };
        Ok(client.inner.post_reverse_req(modified_req)?)
    }

    // Get a reverse pair information
    async fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError> {
        Ok(self
            .get_client()
            .await?
            .inner
            .get_reverse_pairs()?
            .get_btc_to_lbtc_pair())
    }

    /// Create a claim transaction for a receive or chain swap
    async fn create_claim_tx(
        &self,
        swap: Swap,
        claim_address: Option<String>,
    ) -> Result<Transaction, PaymentError> {
        let tx = match &swap {
            Swap::Chain(swap) => {
                let Some(claim_address) = claim_address else {
                    return Err(PaymentError::Generic {
                        err: format!(
                            "No claim address was supplied when claiming for Chain swap {}",
                            swap.id
                        ),
                    });
                };
                match swap.direction {
                    Direction::Incoming => Transaction::Liquid(
                        self.new_incoming_chain_claim_tx(swap, claim_address)
                            .await?,
                    ),
                    Direction::Outgoing => Transaction::Bitcoin(
                        self.new_outgoing_chain_claim_tx(swap, claim_address)
                            .await?,
                    ),
                }
            }
            Swap::Receive(swap) => {
                let Some(claim_address) = claim_address else {
                    return Err(PaymentError::Generic {
                        err: format!(
                            "No claim address was supplied when claiming for Receive swap {}",
                            swap.id
                        ),
                    });
                };
                Transaction::Liquid(self.new_receive_claim_tx(swap, claim_address).await?)
            }
            Swap::Send(swap) => {
                return Err(PaymentError::Generic {
                    err: format!(
                        "Failed to create claim tx for Send swap {}: invalid swap type",
                        swap.id
                    ),
                });
            }
        };

        Ok(tx)
    }

    /// Estimate the refund broadcast transaction size and fees in sats for a send or chain swap
    async fn estimate_refund_broadcast(
        &self,
        swap: Swap,
        refund_address: &str,
        fee_rate_sat_per_vb: Option<f64>,
    ) -> Result<(u32, u64), SdkError> {
        let refund_address = &refund_address.to_string();
        let (refund_keypair, preimage) = match &swap {
            Swap::Chain(swap) => (
                swap.get_refund_keypair()?,
                Preimage::from_str(&swap.preimage)?,
            ),
            Swap::Send(swap) => (swap.get_refund_keypair()?, Preimage::new()),
            Swap::Receive(swap) => {
                return Err(SdkError::generic(format!(
                    "Failed to retrieve refund keypair and preimage for Receive swap {}: invalid swap type",
                    swap.id
                )));
            }
        };

        let refund_tx_size = match self.new_lbtc_refund_wrapper(&swap, refund_address).await {
            Ok(refund_tx_wrapper) => refund_tx_wrapper.size(&refund_keypair, &preimage, true)?,
            Err(_) => {
                let refund_tx_wrapper = self.new_btc_refund_wrapper(&swap, refund_address).await?;
                refund_tx_wrapper.size(&refund_keypair, &preimage)?
            }
        } as u32;

        let fee_rate_sat_per_vb = fee_rate_sat_per_vb.unwrap_or(LIQUID_FEE_RATE_SAT_PER_VBYTE);
        let refund_tx_fees_sat = (refund_tx_size as f64 * fee_rate_sat_per_vb).ceil() as u64;

        Ok((refund_tx_size, refund_tx_fees_sat))
    }

    /// Create a refund transaction for a send or chain swap
    async fn create_refund_tx(
        &self,
        swap: Swap,
        refund_address: &str,
        utxos: Vec<Utxo>,
        broadcast_fee_rate_sat_per_vb: Option<f64>,
        is_cooperative: bool,
    ) -> Result<Transaction, PaymentError> {
        let swap_id = swap.id();
        let refund_address = &refund_address.to_string();

        let tx = match &swap {
            Swap::Chain(chain_swap) => match chain_swap.direction {
                Direction::Incoming => {
                    let Some(broadcast_fee_rate_sat_per_vb) = broadcast_fee_rate_sat_per_vb else {
                        return Err(PaymentError::Generic {
                                err: format!("No broadcast fee rate provided when refunding incoming Chain Swap {swap_id}")
                            });
                    };

                    Transaction::Bitcoin(
                        self.new_btc_refund_tx(
                            chain_swap,
                            refund_address,
                            utxos,
                            broadcast_fee_rate_sat_per_vb,
                            is_cooperative,
                        )
                        .await?,
                    )
                }
                Direction::Outgoing => Transaction::Liquid(
                    self.new_lbtc_refund_tx(&swap, refund_address, utxos, is_cooperative)
                        .await?,
                ),
            },
            Swap::Send(_) => Transaction::Liquid(
                self.new_lbtc_refund_tx(&swap, refund_address, utxos, is_cooperative)
                    .await?,
            ),
            Swap::Receive(_) => {
                return Err(PaymentError::Generic {
                    err: format!(
                        "Failed to create refund tx for Receive swap {swap_id}: invalid swap type",
                    ),
                });
            }
        };

        Ok(tx)
    }

    async fn broadcast_tx(&self, chain: Chain, tx_hex: &str) -> Result<String, PaymentError> {
        let response = self
            .get_client()
            .await?
            .inner
            .broadcast_tx(chain, &tx_hex.into())?;
        let err = format!("Unexpected response from Boltz server: {response}");
        let tx_id = response
            .as_object()
            .ok_or(PaymentError::Generic { err: err.clone() })?
            .get("id")
            .ok_or(PaymentError::Generic { err: err.clone() })?
            .as_str()
            .ok_or(PaymentError::Generic { err })?
            .to_string();
        Ok(tx_id)
    }

    fn create_status_stream(&self) -> Box<dyn SwapperStatusStream> {
        Box::new(BoltzStatusStream::new(
            self.config.clone(),
            self.fetch_proxy_url.clone(),
        ))
    }

    async fn check_for_mrh(
        &self,
        invoice: &str,
    ) -> Result<Option<(String, boltz_client::bitcoin::Amount)>, PaymentError> {
        boltz_client::swaps::magic_routing::check_for_mrh(
            &self.get_client().await?.inner,
            invoice,
            self.config.network.into(),
        )
        .map_err(Into::into)
    }

    async fn get_bolt12_invoice(
        &self,
        offer: &str,
        amount_sat: u64,
    ) -> Result<String, PaymentError> {
        let invoice_res = self
            .get_client()
            .await?
            .inner
            .get_bolt12_invoice(offer, amount_sat)?;
        info!("Received BOLT12 invoice response: {invoice_res:?}");
        Ok(invoice_res.invoice)
    }
}
