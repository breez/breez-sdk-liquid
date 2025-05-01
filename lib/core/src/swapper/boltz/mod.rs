use std::{sync::OnceLock, time::Duration};

use crate::{
    error::{PaymentError, SdkError},
    model::LIQUID_FEE_RATE_SAT_PER_VBYTE,
    prelude::{ChainSwap, Config, Direction, LiquidNetwork, SendSwap, Swap, Transaction, Utxo},
};
use anyhow::{anyhow, Result};
use boltz_client::{
    boltz::{
        self, BoltzApiClientV2, ChainPair, Cooperative, CreateBolt12OfferRequest,
        CreateChainRequest, CreateChainResponse, CreateReverseRequest, CreateReverseResponse,
        CreateSubmarineRequest, CreateSubmarineResponse, GetBolt12FetchResponse,
        GetBolt12ParamsResponse, GetNodesResponse, ReversePair, SubmarineClaimTxResponse,
        SubmarinePair, UpdateBolt12OfferRequest, WsRequest,
    },
    elements::secp256k1_zkp::{MusigPartialSignature, MusigPubNonce},
    network::Chain,
    Amount,
};
use client::{BitcoinClient, LiquidClient};
use log::info;
use proxy::split_proxy_url;
use sdk_common::utils::Arc;
use tokio::sync::broadcast;

use super::{ProxyUrlFetcher, Swapper};

pub(crate) mod bitcoin;
mod client;
pub(crate) mod liquid;
pub(crate) mod proxy;
pub mod status_stream;

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) struct BoltzClient {
    referral_id: Option<String>,
    inner: BoltzApiClientV2,
}

pub struct BoltzSwapper<P: ProxyUrlFetcher> {
    config: Config,
    boltz_client: OnceLock<BoltzClient>,
    liquid_client: OnceLock<LiquidClient>,
    bitcoin_client: OnceLock<BitcoinClient>,
    proxy_url: Arc<P>,
    request_notifier: broadcast::Sender<WsRequest>,
    update_notifier: broadcast::Sender<boltz::SwapStatus>,
    invoice_request_notifier: broadcast::Sender<boltz::InvoiceRequest>,
}

impl<P: ProxyUrlFetcher> BoltzSwapper<P> {
    pub fn new(config: Config, proxy_url: Arc<P>) -> Result<Self, SdkError> {
        let (request_notifier, _) = broadcast::channel::<WsRequest>(30);
        let (update_notifier, _) = broadcast::channel::<boltz::SwapStatus>(30);
        let (invoice_request_notifier, _) = broadcast::channel::<boltz::InvoiceRequest>(30);

        Ok(Self {
            proxy_url,
            config: config.clone(),
            boltz_client: OnceLock::new(),
            liquid_client: OnceLock::new(),
            bitcoin_client: OnceLock::new(),
            request_notifier,
            update_notifier,
            invoice_request_notifier,
        })
    }

    async fn get_boltz_client(&self) -> Result<&BoltzClient> {
        if let Some(client) = self.boltz_client.get() {
            return Ok(client);
        }

        let (boltz_api_base_url, referral_id) = match &self.config.network {
            LiquidNetwork::Testnet | LiquidNetwork::Regtest => (None, None),
            LiquidNetwork::Mainnet => match self.proxy_url.fetch().await {
                Ok(Some(swapper_proxy_url)) => split_proxy_url(swapper_proxy_url),
                _ => (None, None),
            },
        };

        let boltz_url = boltz_api_base_url.unwrap_or(self.config.default_boltz_url().to_string());

        let boltz_client = self.boltz_client.get_or_init(|| BoltzClient {
            inner: BoltzApiClientV2::new(boltz_url, Some(CONNECTION_TIMEOUT)),
            referral_id,
        });
        Ok(boltz_client)
    }

    fn get_liquid_client(&self) -> Result<&LiquidClient> {
        if let Some(client) = self.liquid_client.get() {
            return Ok(client);
        }
        let liquid_client = LiquidClient::new(&self.config)
            .map_err(|err| anyhow!("Could not create Boltz Liquid client: {err:?}"))?;
        let liquid_client = self.liquid_client.get_or_init(|| liquid_client);
        Ok(liquid_client)
    }

    fn get_bitcoin_client(&self) -> Result<&BitcoinClient> {
        if let Some(client) = self.bitcoin_client.get() {
            return Ok(client);
        }
        let bitcoin_client = BitcoinClient::new(&self.config)
            .map_err(|err| anyhow!("Could not create Boltz Bitcoin client: {err:?}"))?;
        let bitcoin_client = self.bitcoin_client.get_or_init(|| bitcoin_client);
        Ok(bitcoin_client)
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
            .get_boltz_client()
            .await?
            .inner
            .get_chain_claim_tx_details(&swap.id)
            .await?;
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
            boltz_api: &self.get_boltz_client().await?.inner,
            swap_id,
            pub_nonce,
            partial_sig,
        }))
    }
}

#[sdk_macros::async_trait]
impl<P: ProxyUrlFetcher> Swapper for BoltzSwapper<P> {
    /// Create a new chain swap
    async fn create_chain_swap(
        &self,
        req: CreateChainRequest,
    ) -> Result<CreateChainResponse, PaymentError> {
        let client = self.get_boltz_client().await?;
        let modified_req = CreateChainRequest {
            referral_id: client.referral_id.clone(),
            ..req.clone()
        };
        Ok(client.inner.post_chain_req(modified_req).await?)
    }

    /// Create a new send swap
    async fn create_send_swap(
        &self,
        req: CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError> {
        let client = self.get_boltz_client().await?;
        let modified_req = CreateSubmarineRequest {
            referral_id: client.referral_id.clone(),
            ..req.clone()
        };
        Ok(client.inner.post_swap_req(&modified_req).await?)
    }

    async fn get_chain_pair(
        &self,
        direction: Direction,
    ) -> Result<Option<ChainPair>, PaymentError> {
        let pairs = self
            .get_boltz_client()
            .await?
            .inner
            .get_chain_pairs()
            .await?;
        let pair = match direction {
            Direction::Incoming => pairs.get_btc_to_lbtc_pair(),
            Direction::Outgoing => pairs.get_lbtc_to_btc_pair(),
        };
        Ok(pair)
    }

    async fn get_chain_pairs(
        &self,
    ) -> Result<(Option<ChainPair>, Option<ChainPair>), PaymentError> {
        let pairs = self
            .get_boltz_client()
            .await?
            .inner
            .get_chain_pairs()
            .await?;
        let pair_outgoing = pairs.get_lbtc_to_btc_pair();
        let pair_incoming = pairs.get_btc_to_lbtc_pair();
        Ok((pair_outgoing, pair_incoming))
    }

    async fn get_zero_amount_chain_swap_quote(&self, swap_id: &str) -> Result<Amount, SdkError> {
        self.get_boltz_client()
            .await?
            .inner
            .get_quote(swap_id)
            .await
            .map(|r| Amount::from_sat(r.amount))
            .map_err(Into::into)
    }

    async fn accept_zero_amount_chain_swap_quote(
        &self,
        swap_id: &str,
        server_lockup_sat: u64,
    ) -> Result<(), PaymentError> {
        self.get_boltz_client()
            .await?
            .inner
            .accept_quote(swap_id, server_lockup_sat)
            .await
            .map_err(Into::into)
    }

    /// Get a submarine pair information
    async fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError> {
        Ok(self
            .get_boltz_client()
            .await?
            .inner
            .get_submarine_pairs()
            .await?
            .get_lbtc_to_btc_pair())
    }

    /// Get a submarine swap's preimage
    async fn get_submarine_preimage(&self, swap_id: &str) -> Result<String, PaymentError> {
        Ok(self
            .get_boltz_client()
            .await?
            .inner
            .get_submarine_preimage(swap_id)
            .await?
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
            .get_boltz_client()
            .await?
            .inner
            .get_submarine_claim_tx_details(&swap.id)
            .await?;
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

        let (partial_sig, pub_nonce) = refund_tx_wrapper.partial_sign(
            &keypair,
            &claim_tx_response.pub_nonce,
            &claim_tx_response.transaction_hash,
        )?;

        self.get_boltz_client()
            .await?
            .inner
            .post_submarine_claim_tx_details(&swap_id.to_string(), pub_nonce, partial_sig)
            .await?;
        info!("Successfully cooperatively claimed Send Swap {swap_id}");
        Ok(())
    }

    // Create a new receive swap
    async fn create_receive_swap(
        &self,
        req: CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError> {
        let client = self.get_boltz_client().await?;
        let modified_req = CreateReverseRequest {
            referral_id: client.referral_id.clone(),
            ..req.clone()
        };
        Ok(client.inner.post_reverse_req(modified_req).await?)
    }

    // Get a reverse pair information
    async fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError> {
        Ok(self
            .get_boltz_client()
            .await?
            .inner
            .get_reverse_pairs()
            .await?
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
        is_cooperative: bool,
    ) -> Result<(u32, u64), SdkError> {
        let refund_address = &refund_address.to_string();
        let refund_keypair = match &swap {
            Swap::Chain(swap) => swap.get_refund_keypair()?,
            Swap::Send(swap) => swap.get_refund_keypair()?,
            Swap::Receive(swap) => {
                return Err(SdkError::generic(format!(
                    "Cannot create refund tx for Receive swap {}: invalid swap type",
                    swap.id
                )));
            }
        };

        let refund_tx_size = match self.new_lbtc_refund_wrapper(&swap, refund_address).await {
            Ok(refund_tx_wrapper) => {
                refund_tx_wrapper.size(&refund_keypair, is_cooperative, true)?
            }
            Err(_) => {
                let refund_tx_wrapper = self.new_btc_refund_wrapper(&swap, refund_address).await?;
                refund_tx_wrapper.size(&refund_keypair, is_cooperative)?
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
                        return Err(PaymentError::generic(format!("No broadcast fee rate provided when refunding incoming Chain Swap {swap_id}")));
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
            .get_boltz_client()
            .await?
            .inner
            .broadcast_tx(chain, &tx_hex.into())
            .await?;
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

    async fn check_for_mrh(&self, invoice: &str) -> Result<Option<(String, Amount)>, PaymentError> {
        boltz_client::swaps::magic_routing::check_for_mrh(
            &self.get_boltz_client().await?.inner,
            invoice,
            self.config.network.into(),
        )
        .await
        .map_err(Into::into)
    }

    async fn get_bolt12_invoice(
        &self,
        offer: &str,
        amount_sat: u64,
    ) -> Result<GetBolt12FetchResponse, PaymentError> {
        let invoice_res = self
            .get_boltz_client()
            .await?
            .inner
            .get_bolt12_invoice(offer, amount_sat)
            .await?;
        info!("Received BOLT12 invoice response: {invoice_res:?}");
        Ok(invoice_res)
    }

    async fn create_bolt12_offer(&self, req: CreateBolt12OfferRequest) -> Result<(), SdkError> {
        self.get_boltz_client()
            .await?
            .inner
            .post_bolt12_offer(req)
            .await?;
        Ok(())
    }

    async fn update_bolt12_offer(&self, req: UpdateBolt12OfferRequest) -> Result<(), SdkError> {
        self.get_boltz_client()
            .await?
            .inner
            .patch_bolt12_offer(req)
            .await?;
        Ok(())
    }

    async fn delete_bolt12_offer(&self, offer: &str, signature: &str) -> Result<(), SdkError> {
        self.get_boltz_client()
            .await?
            .inner
            .delete_bolt12_offer(offer, signature)
            .await?;
        Ok(())
    }

    async fn get_bolt12_params(&self) -> Result<GetBolt12ParamsResponse, SdkError> {
        let res = self
            .get_boltz_client()
            .await?
            .inner
            .get_bolt12_params()
            .await?;
        Ok(res)
    }

    async fn get_nodes(&self) -> Result<GetNodesResponse, SdkError> {
        let res = self.get_boltz_client().await?.inner.get_nodes().await?;
        Ok(res)
    }
}
