mod boltz_status_stream;

use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use boltz_client::error::Error;
use boltz_client::network::electrum::ElectrumConfig;
use boltz_client::network::Chain;
use boltz_client::swaps::boltzv2::{
    self, BoltzApiClientV2, ClaimTxResponse, CreateReverseRequest, CreateReverseResponse,
    CreateSubmarineRequest, CreateSubmarineResponse, ReversePair, SubmarinePair,
};
use boltz_client::util::secrets::Preimage;
use boltz_client::{Amount, Bolt11Invoice, LBtcSwapTxV2};
use boltz_status_stream::BoltzStatusStream;
use log::{debug, info};
use lwk_wollet::elements::LockTime;
use serde_json::Value;
use tokio::sync::{broadcast, watch};

use crate::error::PaymentError;
use crate::model::{Config, Network, ReceiveSwap, SendSwap};
use crate::utils;

#[async_trait]
pub trait ReconnectHandler: Send + Sync {
    async fn on_stream_reconnect(&self);
}

#[async_trait]
pub trait SwapperStatusStream: Send + Sync {
    async fn start(
        self: Arc<Self>,
        callback: Box<dyn ReconnectHandler>,
        shutdown: watch::Receiver<()>,
    );
    fn track_swap_id(&self, swap_id: &str) -> Result<()>;
    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltzv2::Update>;
}

pub trait Swapper: Send + Sync {
    /// Create a new send swap
    fn create_send_swap(
        &self,
        req: CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError>;

    /// Get a submarine pair information
    fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError>;

    /// Refund a cooperatively send swap  
    fn refund_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        output_address: &str,
        broadcast_fees_sat: Amount,
    ) -> Result<String, PaymentError>;

    /// Refund non-cooperatively send swap
    fn refund_send_swap_non_cooperative(
        &self,
        swap: &SendSwap,
        broadcast_fees_sat: Amount,
        output_address: &str,
        current_height: u32,
    ) -> Result<String, PaymentError>;

    /// Get claim tx details which includes the preimage as a proof of payment.
    /// It is used to validate the preimage before claiming which is the reason why we need to separate
    /// the claim into two steps.
    fn get_claim_tx_details(&self, swap: &SendSwap) -> Result<ClaimTxResponse, PaymentError>;

    /// Claim send swap cooperatively. Here the remote swapper is the one that claims.
    /// We are helping to use key spend path for cheaper fees.
    fn claim_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        claim_tx_response: ClaimTxResponse,
        output_address: &str,
    ) -> Result<(), PaymentError>;

    /// Create a new receive swap
    fn create_receive_swap(
        &self,
        req: CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError>;

    /// Get a reverse pair information
    fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError>;

    /// Claim receive swap. Here the local swapper is the one that claims.
    fn claim_receive_swap(
        &self,
        swap: &ReceiveSwap,
        claim_address: String,
    ) -> Result<String, PaymentError>;

    /// Chain broadcast
    fn broadcast_tx(&self, chain: Chain, tx_hex: &str) -> Result<Value, PaymentError>;

    fn create_status_stream(&self) -> Box<dyn SwapperStatusStream>;

    /// Look for a valid Magic Routing Hint. If found, validate it and extract the BIP21 info (amount, address).
    fn check_for_mrh(&self, invoice: &str) -> Result<Option<(String, f64)>, PaymentError>;
}

pub struct BoltzSwapper {
    client: BoltzApiClientV2,
    config: Config,
    electrum_config: ElectrumConfig,
}

impl BoltzSwapper {
    pub fn new(config: Config) -> BoltzSwapper {
        BoltzSwapper {
            client: BoltzApiClientV2::new(&config.boltz_url),
            config: config.clone(),
            electrum_config: ElectrumConfig::new(
                config.network.into(),
                &config.electrum_url,
                true,
                true,
                100,
            ),
        }
    }

    fn new_refund_tx(
        &self,
        swap: &SendSwap,
        output_address: &String,
    ) -> Result<LBtcSwapTxV2, PaymentError> {
        let swap_script = swap.get_swap_script()?;

        Ok(LBtcSwapTxV2::new_refund(
            swap_script.clone(),
            output_address,
            &self.electrum_config,
            self.config.boltz_url.clone(),
            swap.id.to_string(),
        )?)
    }

    fn validate_send_swap_preimage(
        &self,
        swap_id: &str,
        invoice: &str,
        preimage: &str,
    ) -> Result<(), PaymentError> {
        Self::verify_payment_hash(preimage, invoice)?;
        info!("Preimage is valid for Send Swap {swap_id}");
        Ok(())
    }

    fn verify_payment_hash(preimage: &str, invoice: &str) -> Result<(), PaymentError> {
        let preimage = Preimage::from_str(preimage)?;
        let preimage_hash = preimage.sha256.to_string();
        let invoice =
            Bolt11Invoice::from_str(invoice).map_err(|e| Error::Generic(e.to_string()))?;
        let invoice_payment_hash = invoice.payment_hash();

        (invoice_payment_hash.to_string() == preimage_hash)
            .then_some(())
            .ok_or(PaymentError::InvalidPreimage)
    }
}

impl Swapper for BoltzSwapper {
    /// Create a new send swap
    fn create_send_swap(
        &self,
        req: CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError> {
        Ok(self.client.post_swap_req(&req)?)
    }

    /// Get a submarine pair information
    fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError> {
        Ok(self.client.get_submarine_pairs()?.get_lbtc_to_btc_pair())
    }

    /// Refund a cooperatively send swap  
    fn refund_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        output_address: &str,
        broadcast_fees_sat: Amount,
    ) -> Result<String, PaymentError> {
        info!("Initiating cooperative refund for Send Swap {}", &swap.id);
        let refund_tx = self.new_refund_tx(swap, &output_address.into())?;

        let cooperative = Some((&self.client, &swap.id));
        let tx = refund_tx.sign_refund(
            &swap
                .get_refund_keypair()
                .map_err(|e| Error::Generic(e.to_string()))?,
            broadcast_fees_sat,
            cooperative,
        )?;
        let is_lowball = match self.config.network {
            Network::Mainnet => None,
            Network::Testnet => Some((&self.client, boltz_client::network::Chain::LiquidTestnet)),
        };
        let refund_tx_id = refund_tx.broadcast(&tx, &self.electrum_config, is_lowball)?;
        info!(
            "Successfully broadcast cooperative refund for Send Swap {}",
            &swap.id
        );
        Ok(refund_tx_id.clone())
    }

    /// Refund non-cooperatively send swap
    fn refund_send_swap_non_cooperative(
        &self,
        swap: &SendSwap,
        broadcast_fees_sat: Amount,
        output_address: &str,
        current_height: u32,
    ) -> Result<String, PaymentError> {
        let swap_script = swap.get_swap_script()?;
        let locktime_from_height =
            LockTime::from_height(current_height).map_err(|e| PaymentError::Generic {
                err: format!("Cannot convert current block height to lock time: {e:?}"),
            })?;

        info!("locktime info: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}",  swap_script.locktime);
        if !utils::is_locktime_expired(locktime_from_height, swap_script.locktime) {
            return Err(PaymentError::Generic {
                err: format!(
                    "Cannot refund non-cooperatively. Lock time not elapsed yet. Current tip: {:?}. Script lock time: {:?}",
                    locktime_from_height, swap_script.locktime
                )
            });
        }

        let refund_tx = self.new_refund_tx(swap, &output_address.into())?;
        let tx = refund_tx.sign_refund(
            &swap
                .get_refund_keypair()
                .map_err(|e| Error::Generic(e.to_string()))?,
            broadcast_fees_sat,
            None,
        )?;
        let is_lowball = match self.config.network {
            Network::Mainnet => None,
            Network::Testnet => Some((&self.client, boltz_client::network::Chain::LiquidTestnet)),
        };
        let refund_tx_id = refund_tx.broadcast(&tx, &self.electrum_config, is_lowball)?;
        info!(
            "Successfully broadcast non-cooperative refund for swap-in {}",
            swap.id
        );
        Ok(refund_tx_id)
    }

    /// Get claim tx details which includes the preimage as a proof of payment.
    /// It is used to validate the preimage before claiming which is the reason why we need to separate
    /// the claim into two steps.
    fn get_claim_tx_details(&self, swap: &SendSwap) -> Result<ClaimTxResponse, PaymentError> {
        let claim_tx_response = self.client.get_claim_tx_details(&swap.id)?;
        info!("Received claim tx details: {:?}", &claim_tx_response);

        self.validate_send_swap_preimage(&swap.id, &swap.invoice, &claim_tx_response.preimage)?;
        Ok(claim_tx_response)
    }
    /// Claim send swap cooperatively. Here the remote swapper is the one that claims.
    /// We are helping to use key spend path for cheaper fees.
    fn claim_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        claim_tx_response: ClaimTxResponse,
        output_address: &str,
    ) -> Result<(), PaymentError> {
        let swap_id = &swap.id;
        let keypair = swap.get_refund_keypair()?;
        let refund_tx = self.new_refund_tx(swap, &output_address.into())?;

        self.validate_send_swap_preimage(swap_id, &swap.invoice, &claim_tx_response.preimage)?;

        let (partial_sig, pub_nonce) =
            refund_tx.submarine_partial_sig(&keypair, &claim_tx_response)?;

        self.client
            .post_claim_tx_details(&swap_id.to_string(), pub_nonce, partial_sig)?;
        info!("Successfully sent claim details for swap-in {swap_id}");
        Ok(())
    }

    // Create a new receive swap
    fn create_receive_swap(
        &self,
        req: CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError> {
        Ok(self.client.post_reverse_req(req)?)
    }

    // Get a reverse pair information
    fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError> {
        Ok(self.client.get_reverse_pairs()?.get_btc_to_lbtc_pair())
    }

    /// Claim receive swap. Here the local swapper is the one that claims.
    fn claim_receive_swap(
        &self,
        swap: &ReceiveSwap,
        claim_address: String,
    ) -> Result<String, PaymentError> {
        let swap_script = swap.get_swap_script()?;
        let swap_id = &swap.id;
        let claim_tx_wrapper = LBtcSwapTxV2::new_claim(
            swap_script,
            claim_address,
            &self.electrum_config,
            self.config.boltz_url.clone(),
            swap.id.clone(),
        )?;

        let cooperative = Some((&self.client, swap.id.clone()));
        let claim_tx = claim_tx_wrapper.sign_claim(
            &swap.get_claim_keypair()?,
            &Preimage::from_str(&swap.preimage)?,
            Amount::from_sat(swap.claim_fees_sat),
            // Enable cooperative claim (Some) or not (None)
            cooperative,
            // None
        )?;

        let claim_tx_id = claim_tx_wrapper.broadcast(
            &claim_tx,
            &self.electrum_config,
            Some((&self.client, self.config.network.into())),
        )?;
        info!("Successfully broadcast claim tx {claim_tx_id} for Receive Swap {swap_id}");
        debug!("Claim Tx {:?}", claim_tx);
        Ok(claim_tx_id)
    }

    fn broadcast_tx(&self, chain: Chain, tx_hex: &str) -> Result<Value, PaymentError> {
        Ok(self.client.broadcast_tx(chain, &tx_hex.into())?)
    }

    fn create_status_stream(&self) -> Box<dyn SwapperStatusStream> {
        Box::new(BoltzStatusStream::new(&self.config.boltz_url))
    }

    fn check_for_mrh(&self, invoice: &str) -> Result<Option<(String, f64)>, PaymentError> {
        boltz_client::swaps::magic_routing::check_for_mrh(
            &self.client,
            invoice,
            self.config.network.into(),
        )
        .map_err(Into::into)
    }
}
