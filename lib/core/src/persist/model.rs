use super::LnUrlInfo;

#[derive(Clone, Debug, Default)]
pub(crate) struct PaymentTxDetails {
    pub(crate) tx_id: String,
    pub(crate) destination: String,
    pub(crate) description: Option<String>,
    pub(crate) lnurl_info: Option<LnUrlInfo>,
}