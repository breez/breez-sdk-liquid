use std::sync::Arc;

use async_trait::async_trait;
use boltz_client::{
    boltz::{
        ChainPair, CreateChainRequest, CreateChainResponse, CreateReverseRequest,
        CreateReverseResponse, CreateSubmarineRequest, CreateSubmarineResponse, ReversePair,
        SubmarineClaimTxResponse, SubmarinePair,
    },
    network::Chain,
};
use tokio::sync::{broadcast, watch};

use crate::{
    error::{PaymentError, SdkError},
    prelude::{Direction, SendSwap, Swap, Utxo},
};

pub(crate) use reconnect_handler::*;

pub(crate) mod boltz;
pub(crate) mod reconnect_handler;

pub trait Swapper: Send + Sync {
    /// Create a new chain swap
    fn create_chain_swap(
        &self,
        req: CreateChainRequest,
    ) -> Result<CreateChainResponse, PaymentError>;

    /// Create a new send swap
    fn create_send_swap(
        &self,
        req: CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError>;

    /// Get the current rate, limits and fees for a given swap direction
    fn get_chain_pair(&self, direction: Direction) -> Result<Option<ChainPair>, PaymentError>;

    /// Get the current rate, limits and fees for both swap directions
    fn get_chain_pairs(&self) -> Result<(Option<ChainPair>, Option<ChainPair>), PaymentError>;

    /// Get a submarine pair information
    fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError>;

    /// Get send swap claim tx details which includes the preimage as a proof of payment.
    /// It is used to validate the preimage before claiming which is the reason why we need to separate
    /// the claim into two steps.
    fn get_send_claim_tx_details(
        &self,
        swap: &SendSwap,
    ) -> Result<SubmarineClaimTxResponse, PaymentError>;

    /// Claim send swap cooperatively. Here the remote swapper is the one that claims.
    /// We are helping to use key spend path for cheaper fees.
    fn claim_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        claim_tx_response: SubmarineClaimTxResponse,
        refund_address: &str,
    ) -> Result<(), PaymentError>;

    /// Create a new receive swap
    fn create_receive_swap(
        &self,
        req: CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError>;

    /// Get a reverse pair information
    fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError>;

    /// Create a claim transaction for a receive or chain swap
    fn create_claim_tx(
        &self,
        swap: Swap,
        claim_address: Option<String>,
    ) -> Result<crate::prelude::Transaction, PaymentError>;

    /// Estimate the refund broadcast transaction size and fees in sats for a send or chain swap
    fn estimate_refund_broadcast(
        &self,
        swap: Swap,
        refund_address: &str,
        fee_rate_sat_per_vb: Option<f64>,
    ) -> Result<(u32, u64), SdkError>;

    /// Create a refund transaction for a send or chain swap
    fn create_refund_tx(
        &self,
        swap: Swap,
        refund_address: &str,
        utxos: Vec<Utxo>,
        broadcast_fee_rate_sat_per_vb: Option<f64>,
        is_cooperative: bool,
    ) -> Result<crate::prelude::Transaction, PaymentError>;

    /// Broadcasts a transaction and returns its id
    fn broadcast_tx(&self, chain: Chain, tx_hex: &str) -> Result<String, PaymentError>;

    fn create_status_stream(&self) -> Box<dyn SwapperStatusStream>;

    /// Look for a valid Magic Routing Hint. If found, validate it and extract the BIP21 info (amount, address).
    fn check_for_mrh(
        &self,
        invoice: &str,
    ) -> Result<Option<(String, boltz_client::bitcoin::Amount)>, PaymentError>;

    fn get_bolt12_invoice(&self, offer: &str, amount_sat: u64) -> Result<String, PaymentError>;
}

#[async_trait]
pub trait SwapperStatusStream: Send + Sync {
    async fn start(
        self: Arc<Self>,
        callback: Box<dyn ReconnectHandler>,
        shutdown: watch::Receiver<()>,
    );
    fn track_swap_id(&self, swap_id: &str) -> anyhow::Result<()>;
    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltz_client::boltz::Update>;
}
