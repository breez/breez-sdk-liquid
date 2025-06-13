use super::LnUrlInfo;
use crate::model::PaymentType;

#[derive(Clone, Debug, Default)]
pub(crate) struct PaymentTxDetails {
    pub(crate) tx_id: String,
    pub(crate) destination: String,
    pub(crate) description: Option<String>,
    pub(crate) lnurl_info: Option<LnUrlInfo>,
    pub(crate) bip353_address: Option<String>,
    pub(crate) payer_note: Option<String>,
    pub(crate) asset_fees: Option<u64>,
}

#[derive(Debug, Clone)]
pub(crate) struct PaymentTxBalance {
    /// The asset id
    pub(crate) asset_id: String,

    /// The relative amount of funds sent
    /// In case of an outbound payment (Send), this is the payer amount. Otherwise it's the receiver amount.
    /// The amount precision w.r.t satoshis is based on the [PaymentBalance::asset_id] used. Refer to
    /// [crate::prelude::AssetMetadata::precision] for more information
    pub(crate) amount: u64,

    pub(crate) payment_type: PaymentType,
}
