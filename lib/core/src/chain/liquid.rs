use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::ToHex;
use log::info;
use lwk_wollet::{
    elements::{pset::serialize::Serialize, BlockHash, Script, Transaction, Txid},
    hashes::{sha256, Hash},
    BlockchainBackend, ElectrumClient, ElectrumUrl, History,
};
use reqwest::Response;
use serde::Deserialize;
use std::str::FromStr;

use crate::model::Config;

const LIQUID_ESPLORA_URL: &str = "https://lq1.breez.technology/liquid/api";

#[async_trait]
pub trait LiquidChainService: Send + Sync {
    /// Get the blockchain latest block
    async fn tip(&mut self) -> Result<u32>;

    /// Broadcast a transaction
    async fn broadcast(&self, tx: &Transaction, swap_id: Option<&str>) -> Result<Txid>;

    /// Get a list of transactions
    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>>;

    /// Get the transactions involved in a list of scripts including lowball
    async fn get_script_history(&self, scripts: &Script) -> Result<Vec<History>>;
}

#[derive(Deserialize)]
struct EsploraTx {
    txid: Txid,
    status: Status,
}

// TODO some of this fields may be Option in unconfirmed

#[derive(Deserialize)]
struct Status {
    block_height: Option<i32>,
    block_hash: Option<BlockHash>,
}

pub(crate) struct HybridLiquidChainService {
    electrum_client: ElectrumClient,
}

impl HybridLiquidChainService {
    pub(crate) fn new(config: Config) -> Result<Self> {
        let electrum_client =
            ElectrumClient::new(&ElectrumUrl::new(&config.liquid_electrum_url, true, true))?;
        Ok(Self { electrum_client })
    }
}

#[async_trait]
impl LiquidChainService for HybridLiquidChainService {
    async fn tip(&mut self) -> Result<u32> {
        Ok(self.electrum_client.tip()?.height)
    }

    async fn broadcast(&self, tx: &Transaction, swap_id: Option<&str>) -> Result<Txid> {
        let tx_bytes = tx.serialize();
        info!("tx: {}", tx_bytes.to_hex());
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{LIQUID_ESPLORA_URL}/tx"))
            .header("Swap-ID", swap_id.unwrap_or_default())
            .body(tx_bytes.to_hex())
            .send()
            .await?;
        let txid = Txid::from_str(&response.text().await?)?;
        Ok(txid)
    }

    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        Ok(self.electrum_client.get_transactions(txids)?)
    }

    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        let script = lwk_wollet::elements::bitcoin::Script::from_bytes(script.as_bytes());
        let script_hash = sha256::Hash::hash(script.as_bytes())
            .to_byte_array()
            .to_hex();
        let url = format!("{}/scripthash/{}/txs", LIQUID_ESPLORA_URL, script_hash);
        // TODO must handle paging -> https://github.com/blockstream/esplora/blob/master/API.md#addresses
        let response = get_with_retry(&url, 2).await?;
        let json: Vec<EsploraTx> = response.json().await?;

        let history: Vec<History> = json.into_iter().map(Into::into).collect();
        Ok(history)
    }
}

async fn get_with_retry(url: &str, retries: usize) -> Result<Response> {
    let mut attempt = 0;
    loop {
        info!("liquid chain service get_with_retry for url {url}");
        let response = reqwest::get(url).await?;
        attempt += 1;
        // 429 Too many requests
        // 503 Service Temporarily Unavailable
        if response.status() == 429 || response.status() == 503 {
            if attempt >= retries {
                return Err(anyhow!("Too many retry".to_string()));
            }
            let secs = 1 << attempt;

            std::thread::sleep(std::time::Duration::from_secs(secs));
        } else {
            return Ok(response);
        }
    }
}

impl From<EsploraTx> for History {
    fn from(value: EsploraTx) -> Self {
        let status = value.status;
        History {
            txid: value.txid,
            height: status.block_height.unwrap_or_default(),
            block_hash: status.block_hash,
        }
    }
}
