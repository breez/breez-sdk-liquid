use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, ensure, Result};
use boltz_client::swaps::boltzv2::SwapUpdate;
use futures_util::StreamExt;
use log::{debug, error, info};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::error::PaymentError;

/// Fetch the swap status using the websocket endpoint
pub(crate) async fn get_swap_status_v2(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    swap_id: &str,
) -> Result<String> {
    loop {
        match ws_stream
            .next()
            .await
            .ok_or(anyhow!("Received nothing from the stream"))?
        {
            Ok(msg) if msg.is_text() => {
                info!("Received msg: {msg:?}");
                match serde_json::from_str::<SwapUpdate>(&msg.to_string()) {
                    Ok(SwapUpdate::Subscription {
                        event,
                        channel,
                        args,
                    }) => {
                        ensure!(event == "subscribe", "Wrong WS reply event {event}");
                        ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                        let first_arg = args.first();
                        let is_ok = matches!(first_arg.as_ref(), Some(&x) if x == swap_id);
                        ensure!(is_ok, "Wrong WS reply subscription ID {first_arg:?}");

                        info!("Subscription successful for swap : {swap_id}");
                    }
                    Ok(SwapUpdate::Update {
                        event,
                        channel,
                        args,
                    }) => {
                        ensure!(event == "update", "Wrong WS reply event {event}");
                        ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                        return match args.first() {
                            Some(update) if update.id == swap_id => {
                                info!("Got new swap status for {swap_id}: {}", update.status);

                                Ok(update.status.clone())
                            }
                            Some(update) => Err(anyhow!(
                                "WS reply has wrong swap ID {update:?}. Should be {swap_id}"
                            )),
                            None => Err(anyhow!("WS reply contains no update")),
                        };
                    }
                    Ok(SwapUpdate::Error {
                        event,
                        channel,
                        args,
                    }) => {
                        ensure!(event == "update", "Wrong WS reply event {event}");
                        ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                        for e in &args {
                            error!("Got error: {} for swap: {}", e.error, e.id);
                        }
                        return Err(anyhow!(
                            "Got SwapUpdate errors for swap {swap_id}: {args:?}"
                        ));
                    }
                    Err(e) => return Err(anyhow!("WS response is invalid SwapUpdate: {e:?}")),
                }
            }
            Ok(msg) => debug!("Unhandled msg: {msg:?}"),
            Err(e) => return Err(anyhow!("Received error from stream: {e:?}")),
        }
    }
}

pub(crate) fn now() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

pub(crate) fn json_to_pubkey(json: &str) -> Result<boltz_client::PublicKey, PaymentError> {
    boltz_client::PublicKey::from_str(json).map_err(|e| PaymentError::Generic {
        err: format!("Failed to deserialize PublicKey: {e:?}"),
    })
}

pub(crate) fn generate_keypair() -> boltz_client::Keypair {
    let secp = boltz_client::Secp256k1::new();
    let mut rng = bip39::rand::rngs::OsRng;
    let secret_key = lwk_wollet::secp256k1::SecretKey::new(&mut rng);
    boltz_client::Keypair::from_secret_key(&secp, &secret_key)
}

pub(crate) fn decode_keypair(secret_key: &str) -> Result<boltz_client::Keypair, lwk_wollet::Error> {
    let secp = boltz_client::Secp256k1::new();
    let secret_key = lwk_wollet::secp256k1::SecretKey::from_str(secret_key)?;
    Ok(boltz_client::Keypair::from_secret_key(&secp, &secret_key))
}
