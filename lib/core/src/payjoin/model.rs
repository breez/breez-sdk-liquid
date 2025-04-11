use lwk_wollet::{
    elements::{
        confidential::{AssetBlindingFactor, ValueBlindingFactor},
        script::Script,
        Address, AssetId, Txid,
    },
    WalletTxOut,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Request {
    AcceptedAssets(AcceptedAssetsRequest),
    Start(StartRequest),
    Sign(SignRequest),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Response {
    AcceptedAssets(AcceptedAssetsResponse),
    Start(StartResponse),
    Sign(SignResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AcceptedAssetsRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AcceptedAssetsResponse {
    pub accepted_asset: Vec<AcceptedAsset>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AcceptedAsset {
    pub asset_id: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Recipient {
    pub address: Address,
    pub asset_id: AssetId,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Utxo {
    pub txid: Txid,
    pub vout: u32,
    pub script_pub_key: Script,
    pub asset_id: AssetId,
    pub value: u64,
    pub asset_bf: AssetBlindingFactor,
    pub value_bf: ValueBlindingFactor,
}

impl From<&WalletTxOut> for Utxo {
    fn from(tx_out: &WalletTxOut) -> Self {
        Self {
            txid: tx_out.outpoint.txid,
            vout: tx_out.outpoint.vout,
            script_pub_key: tx_out.script_pubkey.clone(),
            asset_id: tx_out.unblinded.asset,
            value: tx_out.unblinded.value,
            asset_bf: tx_out.unblinded.asset_bf,
            value_bf: tx_out.unblinded.value_bf,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct StartRequest {
    pub asset_id: String,
    pub user_agent: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct StartResponse {
    pub order_id: String,
    pub expires_at: u64,
    pub price: f64,
    pub fixed_fee: u64,
    pub fee_address: Address,
    pub change_address: Address,
    pub utxos: Vec<Utxo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SignRequest {
    pub order_id: String,
    pub pset: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SignResponse {
    pub pset: String,
}
