use std::net::TcpStream;
use std::str::FromStr;

use anyhow::{anyhow, ensure, Result};
use boltz_client::swaps::{
    boltz::RevSwapStates,
    boltzv2::{BoltzApiClientV2, Subscription, SwapUpdate},
};
use log::{error, info};
use tungstenite::{stream::MaybeTlsStream, WebSocket};

/// Fetch the swap status using the websocket endpoint
pub(crate) fn get_swap_status_v2(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    swap_id: &str,
) -> Result<String> {
    loop {
        let response: SwapUpdate = serde_json::from_str(&socket.read()?.to_string())
            .map_err(|e| anyhow!("WS response is invalid SwapUpdate: {e:?}"))?;

        match response {
            SwapUpdate::Subscription {
                event,
                channel,
                args,
            } => {
                ensure!(event == "subscribe", "Wrong WS reply event {event}");
                ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                let first_arg = args.first();
                let is_ok = matches!(first_arg.as_ref(), Some(&x) if x == &swap_id);
                ensure!(is_ok, "Wrong WS reply subscription ID {first_arg:?}");

                info!("Subscription successful for swap : {swap_id}");
            }

            SwapUpdate::Update {
                event,
                channel,
                args,
            } => {
                ensure!(event == "update", "Wrong WS reply event {event}");
                ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                return match args.first() {
                    Some(update) if update.id == swap_id => {
                        info!("Got new reverse swap status: {}", update.status);

                        Ok(update.status.clone())
                    }
                    Some(update) => Err(anyhow!("WS reply has wrong swap ID {update:?}")),
                    None => Err(anyhow!("WS reply contains no update")),
                };
            }

            SwapUpdate::Error {
                event,
                channel,
                args,
            } => {
                ensure!(event == "update", "Wrong WS reply event {event}");
                ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                for e in &args {
                    error!("Got error: {} for swap: {}", e.error, e.id);
                }
                return Err(anyhow!("Got SwapUpdate errors: {args:?}"));
            }
        }
    }
}

/// Fetch the reverse swap status using the websocket endpoint
pub(crate) fn get_rev_swap_status_v2(
    client_v2: BoltzApiClientV2,
    swap_id: &str,
) -> Result<RevSwapStates> {
    let mut socket = client_v2
        .connect_ws()
        .map_err(|e| anyhow!("Failed to connect to websocket: {e:?}"))?;

    let sub_id = swap_id.to_string();
    let subscription = Subscription::new(&sub_id);
    let subscribe_json = serde_json::to_string(&subscription)
        .map_err(|e| anyhow!("Failed to serialize subscription msg: {e:?}"))?;
    socket
        .send(tungstenite::Message::Text(subscribe_json))
        .map_err(|e| anyhow!("Failed to subscribe to websocket updates: {e:?}"))?;

    loop {
        let response: SwapUpdate = serde_json::from_str(&socket.read()?.to_string())
            .map_err(|e| anyhow!("WS response is invalid SwapUpdate: {e:?}"))?;

        match response {
            SwapUpdate::Subscription {
                event,
                channel,
                args,
            } => {
                ensure!(event == "subscribe", "Wrong WS reply event {event}");
                ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                let first_arg = args.first();
                let is_ok = matches!(first_arg.as_ref(), Some(&x) if x == &sub_id);
                ensure!(is_ok, "Wrong WS reply subscription ID {first_arg:?}");

                info!("Subscription successful for swap : {sub_id}");
            }

            SwapUpdate::Update {
                event,
                channel,
                args,
            } => {
                ensure!(event == "update", "Wrong WS reply event {event}");
                ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                return match args.first() {
                    Some(update) if update.id == sub_id => {
                        info!("Got new reverse swap status: {}", update.status);

                        RevSwapStates::from_str(&update.status).map_err(|_| {
                            anyhow!("Invalid state for rev swap {swap_id}: {}", update.status)
                        })
                    }
                    Some(update) => Err(anyhow!("WS reply has wrong swap ID {update:?}")),
                    None => Err(anyhow!("WS reply contains no update")),
                };
            }

            SwapUpdate::Error {
                event,
                channel,
                args,
            } => {
                ensure!(event == "update", "Wrong WS reply event {event}");
                ensure!(channel == "swap.update", "Wrong WS reply channel {channel}");

                for e in &args {
                    error!("Got error: {} for swap: {}", e.error, e.id);
                }
                return Err(anyhow!("Got SwapUpdate errors: {args:?}"));
            }
        }
    }
}
