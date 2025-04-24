use base64::Engine;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

const BITCOIND_URL: &str = "http://localhost:18443/wallet/client";
const ELEMENTSD_URL: &str = "http://localhost:18884/wallet/client";
const LND_URL: &str = "https://localhost:8081";

const PROXY_URL: &str = "http://localhost:51234/proxy";

const BITCOIND_COOKIE: Option<&str> = option_env!("BITCOIND_COOKIE");
const ELEMENTSD_COOKIE: &str = "regtest:regtest";
const LND_MACAROON_HEX: Option<&str> = option_env!("LND_MACAROON_HEX");

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
    let address_btc = generate_address_bitcoind().await?;
    let address_lqd = generate_address_elementsd().await?;

    json_rpc_request(
        BITCOIND_URL,
        BITCOIND_COOKIE.unwrap(),
        "generatetoaddress",
        json!([n_blocks, address_btc]),
    )
    .await?;

    json_rpc_request(
        ELEMENTSD_URL,
        ELEMENTSD_COOKIE,
        "generatetoaddress",
        json!([n_blocks, address_lqd]),
    )
    .await?;

    Ok(())
}
