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
const SWAPPROXY_URL: &str = "http://localhost:8387";

const BTC_ESPLORA_URL: &str = "http://localhost:4002/api";
const LBTC_ESPLORA_URL: &str = "http://localhost:3120/api";
const WATERFALLS_URL: &str = "http://localhost:3102";

const BITCOIND_COOKIE: Option<&str> = option_env!("BITCOIND_COOKIE");
const ELEMENTSD_COOKIE: &str = "regtest:regtest";
const LND_MACAROON_HEX: Option<&str> = option_env!("LND_MACAROON_HEX");

#[derive(Clone, Copy)]
pub enum Chain {
    Bitcoin,
    Liquid,
    Both,
}

/// Describes which indexing services are present in the test environment
/// and must be waited on after mining.
#[derive(Clone, Copy)]
pub struct Indexers {
    pub btc_esplora: bool,
    pub lbtc_esplora: bool,
    pub waterfalls: bool,
}

impl Indexers {
    /// Electrum environment: only esplora/electrs, no waterfalls.
    pub fn electrum() -> Self {
        Self {
            btc_esplora: true,
            lbtc_esplora: true,
            waterfalls: false,
        }
    }

    /// Esplora environment: esplora/electrs + waterfalls.
    pub fn esplora() -> Self {
        Self {
            btc_esplora: true,
            lbtc_esplora: true,
            waterfalls: true,
        }
    }
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
    mine_and_index_blocks(n_blocks, Chain::Both, None).await
}

/// Mine one block on Bitcoin first, then one on Liquid, waiting for each
/// indexer before proceeding.  The ordering matters: by the time the
/// Liquid block arrives and triggers the SDK's background poller, the BTC
/// electrs has already indexed its block, so `on_bitcoin_block` will see
/// the new tip.  Without this ordering the gating optimisation in
/// `get_sync_context` (PR #978) can permanently skip the BTC tip fetch.
pub async fn mine_bitcoin_then_liquid(
    n_blocks: u64,
    indexers: &Indexers,
) -> Result<(), Box<dyn Error>> {
    mine_and_index_blocks(n_blocks, Chain::Bitcoin, Some(indexers)).await?;
    mine_and_index_blocks(n_blocks, Chain::Liquid, Some(indexers)).await
}

pub async fn mine_and_index_blocks(
    n_blocks: u64,
    chain: Chain,
    indexers: Option<&Indexers>,
) -> Result<(), Box<dyn Error>> {
    let mine_bitcoin = matches!(chain, Chain::Bitcoin | Chain::Both);
    let mine_liquid = matches!(chain, Chain::Liquid | Chain::Both);

    for _ in 0..n_blocks {
        if mine_bitcoin {
            let address = generate_address_bitcoind().await?;
            json_rpc_request(
                BITCOIND_URL,
                BITCOIND_COOKIE.unwrap(),
                "generatetoaddress",
                json!([1, address]),
            )
            .await?;
        }

        if mine_liquid {
            let address = generate_address_elementsd().await?;
            json_rpc_request(
                ELEMENTSD_URL,
                ELEMENTSD_COOKIE,
                "generatetoaddress",
                json!([1, address]),
            )
            .await?;
        }

        if let Some(idx) = indexers {
            wait_for_indexers(chain, idx).await?;
        }
    }

    Ok(())
}

/// Wait for all indexing services listed in `indexers` to catch up with
/// the daemon tip for the given chain(s).  Each individual wait function
/// assumes its service is running and will fail loudly if unreachable.
pub async fn wait_for_indexers(chain: Chain, indexers: &Indexers) -> Result<(), Box<dyn Error>> {
    let timeout = Duration::from_secs(30);
    let wait_bitcoin = matches!(chain, Chain::Bitcoin | Chain::Both);
    let wait_liquid = matches!(chain, Chain::Liquid | Chain::Both);

    if wait_bitcoin && indexers.btc_esplora {
        let h = get_daemon_blockcount(BITCOIND_URL, BITCOIND_COOKIE.unwrap()).await?;
        wait_for_indexer_tip(BTC_ESPLORA_URL, h, timeout).await?;
    }
    if wait_liquid {
        if indexers.lbtc_esplora {
            let h = get_daemon_blockcount(ELEMENTSD_URL, ELEMENTSD_COOKIE).await?;
            wait_for_indexer_tip(LBTC_ESPLORA_URL, h, timeout).await?;
        }
        if indexers.waterfalls {
            let h = get_daemon_blockcount(ELEMENTSD_URL, ELEMENTSD_COOKIE).await?;
            wait_for_waterfalls_tip(h, timeout).await?;
        }
    }
    Ok(())
}

async fn get_daemon_blockcount(url: &str, cookie: &str) -> Result<u64, Box<dyn Error>> {
    let result = json_rpc_request(url, cookie, "getblockcount", json!([])).await?;
    result
        .as_u64()
        .ok_or_else(|| "getblockcount did not return a number".into())
}

fn chain_rpc_params(chain: Chain) -> (&'static str, &'static str) {
    match chain {
        Chain::Bitcoin => (BITCOIND_URL, BITCOIND_COOKIE.unwrap()),
        Chain::Liquid => (ELEMENTSD_URL, ELEMENTSD_COOKIE),
        Chain::Both => panic!("chain_rpc_params requires a single chain, not Both"),
    }
}

fn chain_name(chain: Chain) -> &'static str {
    match chain {
        Chain::Bitcoin => "Bitcoin",
        Chain::Liquid => "Liquid",
        Chain::Both => "Bitcoin+Liquid",
    }
}

pub async fn wait_for_mempool_tx(
    chain: Chain,
    timeout: Duration,
) -> Result<String, Box<dyn Error>> {
    let (url, cookie) = chain_rpc_params(chain);
    let poll_interval = Duration::from_millis(100);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        let result = json_rpc_request(url, cookie, "getrawmempool", json!([])).await?;
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
        "{} mempool did not contain any transaction within {:?}",
        chain_name(chain),
        timeout
    )
    .into())
}

/// Poll a daemon's mempool until `txid` appears.
pub async fn wait_for_tx_in_mempool(
    chain: Chain,
    txid: &str,
    timeout: Duration,
) -> Result<(), Box<dyn Error>> {
    let (url, cookie) = chain_rpc_params(chain);
    let poll_interval = Duration::from_millis(100);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        let result = json_rpc_request(url, cookie, "getrawmempool", json!([])).await?;
        if let Some(arr) = result.as_array() {
            if arr.iter().any(|v| v.as_str() == Some(txid)) {
                return Ok(());
            }
        }
        tokio::time::sleep(poll_interval).await;
    }

    Err(format!(
        "Transaction {} did not appear in {} mempool within {:?}",
        txid,
        chain_name(chain),
        timeout
    )
    .into())
}

/// Check whether a single chain's mempool is empty.
async fn is_mempool_empty(chain: Chain) -> Result<bool, Box<dyn Error>> {
    let (url, cookie) = chain_rpc_params(chain);
    let result = json_rpc_request(url, cookie, "getrawmempool", json!([])).await?;
    Ok(result.as_array().map_or(true, |a| a.is_empty()))
}

/// Check whether both the Bitcoin and Liquid mempools are empty.
async fn both_mempools_empty() -> Result<bool, Box<dyn Error>> {
    Ok(is_mempool_empty(Chain::Bitcoin).await? && is_mempool_empty(Chain::Liquid).await?)
}

/// Mine blocks on both chains until both mempools are empty
/// Gives up after `max_rounds` mining iterations.
pub async fn drain_mempools(max_rounds: u32) -> Result<(), Box<dyn Error>> {
    let settle = Duration::from_millis(1000);

    for _ in 0..max_rounds {
        mine_and_index_blocks(1, Chain::Both, None).await?;

        if !both_mempools_empty().await? {
            continue; // more txs than fit in one block, mine again
        }

        // Quiescence check: wait for deferred broadcasts.
        tokio::time::sleep(settle).await;
        if both_mempools_empty().await? {
            return Ok(());
        }
        // Something appeared after the first check — loop and mine again
    }

    Err(format!("Mempools not empty after {max_rounds} rounds of mining").into())
}

pub async fn poll_boltz_server_lockup_txid(
    swap_id: &str,
    timeout: Duration,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/v2/swap/chain/{}/transactions", SWAPPROXY_URL, swap_id);
    let client = Client::new();
    let poll_interval = Duration::from_millis(200);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        if let Ok(resp) = client
            .get(PROXY_URL)
            .header("X-Proxy-URL", &url)
            .send()
            .await
        {
            if let Ok(body) = resp.json::<Value>().await {
                if let Some(txid) = body
                    .get("serverLock")
                    .and_then(|sl| sl.get("transaction"))
                    .and_then(|tx| tx.get("id"))
                    .and_then(|id| id.as_str())
                {
                    return Ok(txid.to_string());
                }
            }
        }
        tokio::time::sleep(poll_interval).await;
    }

    Err(format!(
        "Boltz did not broadcast server lockup for chain swap {} within {:?}",
        swap_id, timeout
    )
    .into())
}

async fn wait_for_waterfalls_tip(
    expected_height: u64,
    timeout: Duration,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/block-height/{}", WATERFALLS_URL, expected_height);
    let client = Client::new();
    let poll_interval = Duration::from_millis(100);
    let iterations = (timeout.as_millis() / poll_interval.as_millis()).max(1) as u64;

    for _ in 0..iterations {
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                return Ok(());
            }
        }
        tokio::time::sleep(poll_interval).await;
    }

    Err(format!(
        "Waterfalls did not index Liquid height {} within {:?}",
        expected_height, timeout
    )
    .into())
}

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
