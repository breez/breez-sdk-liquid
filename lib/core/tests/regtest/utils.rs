use base64::Engine;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::time::Duration;
use tokio_with_wasm::alias as tokio;

const BITCOIND_URL: &str = "http://localhost:18443/wallet/client";
const ELEMENTSD_URL: &str = "http://localhost:18884/wallet/client";
const LND_URL: &str = "https://localhost:8081";

const PROXY_URL: &str = "http://localhost:51234/proxy";

/// Esplora HTTP endpoints for checking indexer tip heights.
/// BTC: esplora container → electrs (same process as Electrum at 19001).
/// L-BTC: nginx → waterfalls → esplora → electrs-liquid (same process as
///        Electrum at 19002).
const BTC_ESPLORA_URL: &str = "http://localhost:4002/api";
const LBTC_ESPLORA_URL: &str = "http://localhost:3120/api";

const BITCOIND_COOKIE: Option<&str> = option_env!("BITCOIND_COOKIE");
const ELEMENTSD_COOKIE: &str = "regtest:regtest";
const LND_MACAROON_HEX: Option<&str> = option_env!("LND_MACAROON_HEX");

/// Which blockchain(s) to mine on.
#[derive(Clone, Copy)]
pub enum Chain {
    Bitcoin,
    Liquid,
    Both,
}

async fn json_rpc_request(
    url: &str,
    cookie: &str,
    method: &str,
    params: Value,
) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();

    let req_body = json!({
        "jsonrpc": "1.0",
        "id": "curltest",
        "method": method,
        "params": params
    });

    let res = client
        .post(PROXY_URL)
        .header(
            "Authorization",
            format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD.encode(cookie)
            ),
        )
        .header("X-Proxy-URL", url)
        .json(&req_body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    res.get("result")
        .cloned()
        .ok_or_else(|| "Invalid response".into())
}

async fn lnd_request(method: &str, params: Value) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/{}", LND_URL, method);

    let res = client
        .post(PROXY_URL)
        .header("Grpc-Metadata-macaroon", LND_MACAROON_HEX.unwrap())
        .header("X-Proxy-URL", url)
        .json(&params)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

pub async fn generate_address_bitcoind() -> Result<String, Box<dyn Error>> {
    let response = json_rpc_request(
        BITCOIND_URL,
        BITCOIND_COOKIE.unwrap(),
        "getnewaddress",
        json!([]),
    )
    .await?;

    response
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid response".into())
}

pub async fn send_to_address_bitcoind(
    address: &str,
    sat_amount: u64,
) -> Result<String, Box<dyn Error>> {
    let btc_amount = (sat_amount as f64) / 100_000_000.0;
    json_rpc_request(
        BITCOIND_URL,
        BITCOIND_COOKIE.unwrap(),
        "sendtoaddress",
        json!([address, format!("{:.8}", btc_amount)]),
    )
    .await?
    .as_str()
    .map(|s| s.to_string())
    .ok_or_else(|| "Invalid response".into())
}

pub async fn generate_address_elementsd() -> Result<String, Box<dyn Error>> {
    json_rpc_request(ELEMENTSD_URL, ELEMENTSD_COOKIE, "getnewaddress", json!([]))
        .await?
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid response".into())
}

pub async fn send_to_address_elementsd(
    address: &str,
    sat_amount: u64,
) -> Result<String, Box<dyn Error>> {
    let btc_amount = (sat_amount as f64) / 100_000_000.0;
    json_rpc_request(
        ELEMENTSD_URL,
        ELEMENTSD_COOKIE,
        "sendtoaddress",
        json!([address, format!("{:.8}", btc_amount)]),
    )
    .await?
    .as_str()
    .map(|s| s.to_string())
    .ok_or_else(|| "Invalid response".into())
}

pub async fn generate_invoice_lnd(amount_sat: u64) -> Result<String, Box<dyn Error>> {
    let response = lnd_request("v1/invoices", json!({ "value": amount_sat })).await?;
    response["payment_request"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Missing payment_request field".into())
}

pub async fn pay_invoice_lnd(invoice: &str) -> Result<(), Box<dyn Error>> {
    lnd_request(
        "v2/router/send",
        json!({ "payment_request": invoice, "timeout_seconds": 1 }),
    )
    .await?;
    Ok(())
}

pub async fn mine_blocks(n_blocks: u64) -> Result<(), Box<dyn Error>> {
    mine_and_index_blocks(n_blocks, Chain::Both, false).await
}

/// Mine one block on Bitcoin first, then one on Liquid, waiting for each
/// indexer before proceeding.  The ordering matters: by the time the
/// Liquid block arrives and triggers the SDK's background poller, the BTC
/// electrs has already indexed its block, so `on_bitcoin_block` will see
/// the new tip.  Without this ordering the gating optimisation in
/// `get_sync_context` (PR #978) can permanently skip the BTC tip fetch.
pub async fn mine_bitcoin_then_liquid(n_blocks: u64) -> Result<(), Box<dyn Error>> {
    mine_and_index_blocks(n_blocks, Chain::Bitcoin, true).await?;
    mine_and_index_blocks(n_blocks, Chain::Liquid, true).await
}

pub async fn mine_and_index_blocks(
    n_blocks: u64,
    chain: Chain,
    wait_for_indexer: bool,
) -> Result<(), Box<dyn Error>> {
    let mine_bitcoin = matches!(chain, Chain::Bitcoin | Chain::Both);
    let mine_liquid = matches!(chain, Chain::Liquid | Chain::Both);

    if mine_bitcoin {
        let address = generate_address_bitcoind().await?;
        json_rpc_request(
            BITCOIND_URL,
            BITCOIND_COOKIE.unwrap(),
            "generatetoaddress",
            json!([n_blocks, address]),
        )
        .await?;
    }

    if mine_liquid {
        let address = generate_address_elementsd().await?;
        json_rpc_request(
            ELEMENTSD_URL,
            ELEMENTSD_COOKIE,
            "generatetoaddress",
            json!([n_blocks, address]),
        )
        .await?;
    }

    if wait_for_indexer {
        let timeout = Duration::from_secs(30);
        if mine_bitcoin {
            let daemon_height = get_daemon_blockcount(BITCOIND_URL, BITCOIND_COOKIE.unwrap()).await?;
            wait_for_indexer_tip(BTC_ESPLORA_URL, daemon_height, timeout).await?;
        }
        if mine_liquid {
            let daemon_height = get_daemon_blockcount(ELEMENTSD_URL, ELEMENTSD_COOKIE).await?;
            wait_for_indexer_tip(LBTC_ESPLORA_URL, daemon_height, timeout).await?;
        }
    }

    Ok(())
}

/// Query a daemon's current block height via JSON-RPC `getblockcount`.
async fn get_daemon_blockcount(url: &str, cookie: &str) -> Result<u64, Box<dyn Error>> {
    let result = json_rpc_request(url, cookie, "getblockcount", json!([])).await?;
    result
        .as_u64()
        .ok_or_else(|| "getblockcount did not return a number".into())
}

/// Poll elementsd's `getrawmempool` until it contains at least one
/// transaction.  Returns the txid of the first entry found.
/// Times out with an error if the mempool stays empty.
pub async fn wait_for_liquid_mempool_tx(timeout: Duration) -> Result<String, Box<dyn Error>> {
    let poll_interval = Duration::from_millis(100);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        let result =
            json_rpc_request(ELEMENTSD_URL, ELEMENTSD_COOKIE, "getrawmempool", json!([])).await?;
        if let Some(arr) = result.as_array() {
            if let Some(first) = arr.first() {
                if let Some(txid) = first.as_str() {
                    return Ok(txid.to_string());
                }
            }
        }
        tokio::time::sleep(poll_interval).await;
    }

    Err(format!(
        "Liquid mempool did not contain any transaction within {:?}",
        timeout
    )
    .into())
}

/// Poll elementsd's `getrawmempool` until `txid` appears.
/// Times out with an error if the transaction is not seen.
pub async fn wait_for_tx_in_liquid_mempool(
    txid: &str,
    timeout: Duration,
) -> Result<(), Box<dyn Error>> {
    let poll_interval = Duration::from_millis(100);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        let result =
            json_rpc_request(ELEMENTSD_URL, ELEMENTSD_COOKIE, "getrawmempool", json!([])).await?;
        if let Some(arr) = result.as_array() {
            if arr.iter().any(|v| v.as_str() == Some(txid)) {
                return Ok(());
            }
        }
        tokio::time::sleep(poll_interval).await;
    }

    Err(format!(
        "Transaction {} did not appear in Liquid mempool within {:?}",
        txid, timeout
    )
    .into())
}

/// Poll bitcoind's `getrawmempool` until it contains at least one
/// transaction.  Returns the txid of the first entry found.
/// Times out with an error if the mempool stays empty.
pub async fn wait_for_bitcoin_mempool_tx(timeout: Duration) -> Result<String, Box<dyn Error>> {
    let poll_interval = Duration::from_millis(100);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        let result = json_rpc_request(
            BITCOIND_URL,
            BITCOIND_COOKIE.unwrap(),
            "getrawmempool",
            json!([]),
        )
        .await?;
        if let Some(arr) = result.as_array() {
            if let Some(first) = arr.first() {
                if let Some(txid) = first.as_str() {
                    return Ok(txid.to_string());
                }
            }
        }
        tokio::time::sleep(poll_interval).await;
    }

    Err(format!(
        "Bitcoin mempool did not contain any transaction within {:?}",
        timeout
    )
    .into())
}

/// Poll bitcoind's `getrawmempool` until `txid` appears.
/// Times out with an error if the transaction is not seen.
pub async fn wait_for_tx_in_bitcoin_mempool(
    txid: &str,
    timeout: Duration,
) -> Result<(), Box<dyn Error>> {
    let poll_interval = Duration::from_millis(100);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        let result = json_rpc_request(
            BITCOIND_URL,
            BITCOIND_COOKIE.unwrap(),
            "getrawmempool",
            json!([]),
        )
        .await?;
        if let Some(arr) = result.as_array() {
            if arr.iter().any(|v| v.as_str() == Some(txid)) {
                return Ok(());
            }
        }
        tokio::time::sleep(poll_interval).await;
    }

    Err(format!(
        "Transaction {} did not appear in Bitcoin mempool within {:?}",
        txid, timeout
    )
    .into())
}

/// Poll an esplora HTTP endpoint until the reported tip height is at least
/// `expected_height`.  Times out after `timeout`.
async fn wait_for_indexer_tip(
    esplora_url: &str,
    expected_height: u64,
    timeout: Duration,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/blocks/tip/height", esplora_url);
    let client = Client::new();
    let poll_interval = Duration::from_millis(100);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        if let Ok(resp) = client.get(&url).send().await {
            if let Ok(text) = resp.text().await {
                if let Ok(tip) = text.trim().parse::<u64>() {
                    if tip >= expected_height {
                        return Ok(());
                    }
                }
            }
        }
        tokio::time::sleep(poll_interval).await;
    }

    Err(format!(
        "Indexer at {} did not reach height {} within {:?}",
        esplora_url, expected_height, timeout
    )
    .into())
}
