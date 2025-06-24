use super::LnUrlInfo;

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
