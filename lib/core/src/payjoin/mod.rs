pub(crate) mod error;
pub(crate) mod model;
mod pset;
pub(crate) mod side_swap;
mod utxo_select;

use error::PayjoinResult;
use lwk_wollet::elements::Transaction;
use maybe_sync::{MaybeSend, MaybeSync};
use model::AcceptedAsset;

#[sdk_macros::async_trait]
pub trait PayjoinService: MaybeSend + MaybeSync {
    /// Get a list of accepted assets
    async fn fetch_accepted_assets(&self) -> PayjoinResult<Vec<AcceptedAsset>>;

    /// Estimate the fee for a payjoin transaction
    async fn estimate_payjoin_tx_fee(&self, asset_id: &str, amount_sat: u64) -> PayjoinResult<f64>;

    /// Build a payjoin transaction to send funds to a recipient using the asset to pay fees.
    /// Returns the transaction and the service fee paid in satoshi units.
    async fn build_payjoin_tx(
        &self,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> PayjoinResult<(Transaction, u64)>;
}
