use super::LnUrlInfo;

#[derive(Clone, Debug, Default)]
pub(crate) struct PaymentTxDetails {
    pub(crate) destination: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) lnurl_info: Option<LnUrlInfo>,
}
