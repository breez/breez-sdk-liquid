use std::collections::HashSet;
use std::str::FromStr;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use anyhow::Result;
use boltz_client::swaps::{
    boltz::{RevSwapStates, SubSwapStates},
    boltzv2::{Subscription, SwapUpdate},
};
use boltz_client::SwapType;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use tokio::net::TcpStream;
use tokio::sync::watch;
use tokio::time::MissedTickBehavior;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::model::*;
use crate::sdk::LiquidSdk;

static SEND_SWAP_IDS: OnceLock<Arc<Mutex<HashSet<String>>>> = OnceLock::new();
static RECEIVE_SWAP_IDS: OnceLock<Arc<Mutex<HashSet<String>>>> = OnceLock::new();

fn send_swap_ids() -> &'static Arc<Mutex<HashSet<String>>> {
    let send_swap_ids = Default::default();
    SEND_SWAP_IDS.get_or_init(|| send_swap_ids)
}

fn receive_swap_ids() -> &'static Arc<Mutex<HashSet<String>>> {
    let receive_swap_ids = Default::default();
    RECEIVE_SWAP_IDS.get_or_init(|| receive_swap_ids)
}

pub(super) struct BoltzStatusStream {}
impl BoltzStatusStream {
    pub(super) fn mark_swap_as_tracked(id: &str, swap_type: SwapType) {
        match swap_type {
            SwapType::Submarine => send_swap_ids().lock().unwrap().insert(id.to_string()),
            SwapType::ReverseSubmarine => receive_swap_ids().lock().unwrap().insert(id.to_string()),
        };
    }

    pub(super) fn unmark_swap_as_tracked(id: &str, swap_type: SwapType) {
        match swap_type {
            SwapType::Submarine => send_swap_ids().lock().unwrap().remove(id),
            SwapType::ReverseSubmarine => receive_swap_ids().lock().unwrap().remove(id),
        };
    }

    pub(super) async fn track_pending_swaps(
        sdk: Arc<LiquidSdk>,
        mut shutdown: watch::Receiver<()>,
    ) -> Result<()> {
        let keep_alive_ping_interval = Duration::from_secs(15);
        let reconnect_delay = Duration::from_secs(2);

        tokio::spawn(async move {
            loop {
                debug!("Start of ws stream loop");
                match sdk.get_boltz_ws_stream().await {
                    Ok(mut ws_stream) => {
                        // Initially subscribe to all ongoing swaps
                        match sdk.list_ongoing_swaps() {
                            Ok(initial_ongoing_swaps) => {
                                info!("Got {} initial ongoing swaps", initial_ongoing_swaps.len());
                                for ongoing_swap in &initial_ongoing_swaps {
                                    Self::maybe_subscribe_fn(ongoing_swap, &mut ws_stream).await;
                                }
                            }
                            Err(e) => error!("Failed to list initial ongoing swaps: {e:?}"),
                        }

                        let mut interval = tokio::time::interval(keep_alive_ping_interval);
                        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

                        loop {
                            tokio::select! {
                                _ = shutdown.changed() => {
                                    info!("Received shutdown signal, exiting Status Stream loop");

                                    // Clear monitored swaps, so on re-connect we re-subscribe to them
                                    send_swap_ids().lock().unwrap().clear();
                                    receive_swap_ids().lock().unwrap().clear();

                                    return;
                                }

                                _ = interval.tick() => {
                                    match ws_stream.send(Message::Ping(vec![])).await {
                                        Ok(_) => debug!("Sent keep-alive ping"),
                                        Err(e) => warn!("Failed to send keep-alive ping: {e:?}"),
                                    }
                                },

                                maybe_next = ws_stream.next() => match maybe_next {
                                    Some(msg) => match msg {
                                        Ok(Message::Close(_)) => {
                                            warn!("Received close msg, exiting socket loop");
                                            // Clear monitored swaps, so on re-connect we re-subscribe to them
                                            send_swap_ids().lock().unwrap().clear();
                                            receive_swap_ids().lock().unwrap().clear();

                                            tokio::time::sleep(reconnect_delay).await;
                                            break;
                                        },
                                        Ok(msg) => {
                                            info!("Received msg: {msg:?}");

                                            // Each time socket.read() returns, we have the opportunity to socket.send().
                                            // We use this window to subscribe to any new ongoing swaps.
                                            // This happens on any non-close socket messages, in particular:
                                            // Ping (periodic keep-alive), Text (status update)
                                            match sdk.list_ongoing_swaps() {
                                                Ok(ongoing_swaps) => {
                                                    for ongoing_swap in &ongoing_swaps {
                                                        Self::maybe_subscribe_fn(ongoing_swap, &mut ws_stream).await;
                                                    }
                                                }
                                                Err(e) => error!("Failed to list new ongoing swaps: {e:?}"),
                                            }

                                            if msg.is_text() {
                                                match serde_json::from_str::<SwapUpdate>(&msg.to_string()) {
                                                    // Subscription confirmation
                                                    Ok(SwapUpdate::Subscription { .. }) => {}

                                                    // Status update(s)
                                                    Ok(SwapUpdate::Update {
                                                        event: _,
                                                        channel: _,
                                                        args,
                                                    }) => {
                                                        for boltz_client::swaps::boltzv2::Update { id, status } in
                                                            args
                                                        {
                                                            if Self::is_tracked_send_swap(&id) {
                                                                match SubSwapStates::from_str(&status) {
                                                                    Ok(new_state) => {
                                                                        let res = sdk.try_handle_send_swap_boltz_status(
                                                                            new_state,
                                                                            &id,
                                                                        ).await;
                                                                        info!("Handled new Send Swap status from Boltz, result: {res:?}");
                                                                    }
                                                                    Err(_) => error!("Received invalid SubSwapState for Send Swap {id}: {status}")
                                                                }
                                                            } else if Self::is_tracked_receive_swap(&id) {
                                                                match RevSwapStates::from_str(&status) {
                                                                    Ok(new_state) => {
                                                                        let res = sdk.try_handle_receive_swap_boltz_status(
                                                                            new_state, &id,
                                                                        ).await;
                                                                        info!("Handled new Receive Swap status from Boltz, result: {res:?}");
                                                                    }
                                                                    Err(_) => error!("Received invalid RevSwapState for Receive Swap {id}: {status}"),
                                                                }
                                                            } else {
                                                                warn!("Received a status update for swap {id}, which is not tracked as ongoing")
                                                            }
                                                        }
                                                    }

                                                    // Error related to subscription, like "Unknown swap ID"
                                                    Ok(SwapUpdate::Error {
                                                        event: _,
                                                        channel: _,
                                                        args,
                                                    }) => error!("Received a status update error: {args:?}"),

                                                    Err(e) => warn!("WS response is invalid SwapUpdate: {e:?}"),
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            error!("Received stream error: {e:?}");
                                            let _ = ws_stream.close(None).await;
                                            break;
                                        }
                                    },
                                    None => {
                                        warn!("Received nothing from the stream");
                                        let _ = ws_stream.close(None).await;
                                        tokio::time::sleep(reconnect_delay).await;
                                        break;
                                    },
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error connecting to stream: {e}");
                        tokio::time::sleep(reconnect_delay).await;
                    }
                }
            }
        });

        Ok(())
    }

    fn is_tracked_send_swap(id: &str) -> bool {
        send_swap_ids().lock().unwrap().contains(id)
    }

    fn is_tracked_receive_swap(id: &str) -> bool {
        receive_swap_ids().lock().unwrap().contains(id)
    }

    async fn maybe_subscribe_fn(
        swap: &Swap,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) {
        let id = swap.id();
        let is_ongoing_swap_already_tracked = match swap {
            Swap::Send(_) => Self::is_tracked_send_swap(&id),
            Swap::Receive(_) => Self::is_tracked_receive_swap(&id),
        };

        if !is_ongoing_swap_already_tracked {
            info!("Subscribing to status updates for ongoing swap ID {id}");

            let subscription = Subscription::new(&id);
            match serde_json::to_string(&subscription) {
                Ok(subscribe_json) => match ws_stream.send(Message::Text(subscribe_json)).await {
                    Ok(_) => Self::mark_swap_as_tracked(&id, swap.swap_type()),
                    Err(e) => error!("Failed to subscribe to {id}: {e:?}"),
                },
                Err(e) => error!("Invalid subscription msg: {e:?}"),
            }
        }
    }
}
