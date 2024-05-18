use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, ensure, Result};
use boltz_client::swaps::boltzv2::SwapUpdate;
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
                let is_ok = matches!(first_arg.as_ref(), Some(&x) if x == swap_id);
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
                        info!("Got new swap status: {}", update.status);

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

pub(crate) fn now() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}
