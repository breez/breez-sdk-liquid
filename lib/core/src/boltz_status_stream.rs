use std::collections::HashSet;
use std::io::ErrorKind;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use boltz_client::swaps::{
    boltz::{RevSwapStates, SubSwapStates},
    boltzv2::{Subscription, SwapUpdate},
};
use boltz_client::SwapType;
use log::{debug, error, info, warn};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

use crate::model::*;
use crate::sdk::LiquidSdk;

static SWAP_IN_IDS: OnceLock<Arc<Mutex<HashSet<String>>>> = OnceLock::new();
static SWAP_OUT_IDS: OnceLock<Arc<Mutex<HashSet<String>>>> = OnceLock::new();

fn swap_in_ids() -> &'static Arc<Mutex<HashSet<String>>> {
    let swap_in_ids = Default::default();
    SWAP_IN_IDS.get_or_init(|| swap_in_ids)
}

fn swap_out_ids() -> &'static Arc<Mutex<HashSet<String>>> {
    let swap_out_ids = Default::default();
    SWAP_OUT_IDS.get_or_init(|| swap_out_ids)
}

/// Set underlying TCP stream to nonblocking mode.
///
/// This allows us to `read()` without blocking.
pub(crate) fn set_stream_nonblocking(stream: &mut MaybeTlsStream<TcpStream>) -> Result<()> {
    match stream {
        tungstenite::stream::MaybeTlsStream::Plain(s) => s.set_nonblocking(true)?,
        tungstenite::stream::MaybeTlsStream::NativeTls(s) => s.get_mut().set_nonblocking(true)?,
        _ => Err(anyhow!("Unsupported stream type"))?,
    };
    Ok(())
}

pub(super) struct BoltzStatusStream {}
impl BoltzStatusStream {
    pub(super) fn mark_swap_as_tracked(id: &str, swap_type: SwapType) {
        match swap_type {
            SwapType::Submarine => swap_in_ids().lock().unwrap().insert(id.to_string()),
            SwapType::ReverseSubmarine => swap_out_ids().lock().unwrap().insert(id.to_string()),
        };
    }

    pub(super) fn unmark_swap_as_tracked(id: &str, swap_type: SwapType) {
        match swap_type {
            SwapType::Submarine => swap_in_ids().lock().unwrap().remove(id),
            SwapType::ReverseSubmarine => swap_out_ids().lock().unwrap().remove(id),
        };
    }

    fn connect(sdk: Arc<LiquidSdk>) -> Result<WebSocket<MaybeTlsStream<TcpStream>>> {
        let mut socket = sdk
            .boltz_client_v2()
            .connect_ws()
            .map_err(|e| anyhow!("Failed to connect to websocket: {e:?}"))?;
        set_stream_nonblocking(socket.get_mut())?;
        Ok(socket)
    }

    pub(super) fn track_pending_swaps(sdk: Arc<LiquidSdk>) -> Result<()> {
        let mut socket = Self::connect(sdk.clone())?;

        let reconnect_delay = Duration::from_secs(15);
        let keep_alive_ping_interval = Duration::from_secs(15);
        let mut keep_alive_last_ping_ts = Instant::now();

        // Outer loop: reconnects in case the connection is lost
        thread::spawn(move || loop {
            // Initially subscribe to all ongoing swaps
            match sdk.list_ongoing_swaps() {
                Ok(initial_ongoing_swaps) => {
                    info!("Got {} initial ongoing swaps", initial_ongoing_swaps.len());
                    for ongoing_swap in &initial_ongoing_swaps {
                        Self::maybe_subscribe_fn(ongoing_swap, &mut socket);
                    }
                }
                Err(e) => error!("Failed to list initial ongoing swaps: {e:?}"),
            }

            // Inner loop: iterates over incoming messages and handles them
            loop {
                // Decide if we send a keep-alive ping or not
                if Instant::now()
                    .duration_since(keep_alive_last_ping_ts)
                    .gt(&keep_alive_ping_interval)
                {
                    match socket.send(Message::Ping(vec![])) {
                        Ok(_) => debug!("Sent keep-alive ping"),
                        Err(e) => warn!("Failed to send keep-alive ping: {e:?}"),
                    }
                    keep_alive_last_ping_ts = Instant::now();
                }

                match &socket.read() {
                    Ok(Message::Close(_)) => {
                        warn!("Received close msg, exiting socket loop");
                        break;
                    }
                    Ok(msg) => {
                        info!("Received msg : {msg:?}");

                        // Each time socket.read() returns, we have the opportunity to socket.send().
                        // We use this window to subscribe to any new ongoing swaps.
                        // This happens on any non-close socket messages, in particular:
                        // Ping (periodic keep-alive), Text (status update)
                        match sdk.list_ongoing_swaps() {
                            Ok(ongoing_swaps) => {
                                for ongoing_swap in &ongoing_swaps {
                                    Self::maybe_subscribe_fn(ongoing_swap, &mut socket);
                                }
                            }
                            Err(e) => error!("Failed to list new ongoing swaps: {e:?}"),
                        }

                        // We parse and handle any Text websocket messages, which are likely status updates
                        if msg.is_text() {
                            info!("Received text msg (status update) : {msg:?}");

                            match serde_json::from_str::<SwapUpdate>(&msg.to_string()) {
                                // Subscription confirmation
                                Ok(SwapUpdate::Subscription { .. }) => {}

                                // Status update(s)
                                Ok(SwapUpdate::Update {
                                    event: _,
                                    channel: _,
                                    args,
                                }) => {
                                    for boltz_client::swaps::boltzv2::Update { id, status } in args
                                    {
                                        if Self::is_tracked_swap_in(&id) {
                                            // Known OngoingSwapIn / Send swap

                                            match SubSwapStates::from_str(&status) {
                                                Ok(new_state) => {
                                                    let res = sdk.try_handle_submarine_swap_status(
                                                        new_state,
                                                        &id,
                                                    );
                                                    info!("OngoingSwapIn / send try_handle_submarine_swap_status res: {res:?}");
                                                }
                                                Err(_) => error!("Received invalid SubSwapState for swap {id}: {status}")
                                            }
                                        } else if Self::is_tracked_swap_out(&id) {
                                            // Known OngoingSwapOut / receive swap

                                            match RevSwapStates::from_str(&status) {
                                                Ok(new_state) => {
                                                    let res = sdk.try_handle_reverse_swap_status(
                                                        new_state, &id,
                                                    );
                                                    info!("OngoingSwapOut / receive try_handle_reverse_swap_status res: {res:?}");
                                                }
                                                Err(_) => error!("Received invalid RevSwapState for swap {id}: {status}"),
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
                    }
                    Err(tungstenite::Error::Io(io_err)) => {
                        match io_err.kind() {
                            // Calling socket.read() on a non-blocking stream when there is nothing
                            // to read results in an WouldBlock error. In this case, we do nothing
                            // and continue the loop.
                            ErrorKind::WouldBlock => {}
                            _ => {
                                error!("Received stream IO error : {io_err:?}");
                                break;
                            }
                        }
                    }
                    Err(tungstenite::Error::AlreadyClosed) => {
                        thread::sleep(reconnect_delay);
                        info!("Re-connecting...");
                        match Self::connect(sdk.clone()) {
                            Ok(new_socket) => {
                                socket = new_socket;
                                info!("Re-connected to WS stream");

                                // Clear monitored swaps, so on re-connect we re-subscribe to them
                                swap_in_ids().lock().unwrap().clear();
                                swap_out_ids().lock().unwrap().clear();
                            }
                            Err(e) => warn!("Failed to re-connected to WS stream: {e:}"),
                        };
                        break;
                    }
                    Err(e) => {
                        error!("Received stream error : {e:?}");
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    fn is_tracked_swap_in(id: &str) -> bool {
        swap_in_ids().lock().unwrap().contains(id)
    }

    fn is_tracked_swap_out(id: &str) -> bool {
        swap_out_ids().lock().unwrap().contains(id)
    }

    fn maybe_subscribe_fn(swap: &Swap, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
        let id = swap.id();
        let is_ongoing_swap_already_tracked = match swap {
            Swap::Send(_) => Self::is_tracked_swap_in(&id),
            Swap::Receive(_) => Self::is_tracked_swap_out(&id),
        };

        if !is_ongoing_swap_already_tracked {
            info!("Subscribing to status updates for ongoing swap ID {id}");

            let subscription = Subscription::new(&id);
            match serde_json::to_string(&subscription) {
                Ok(subscribe_json) => match socket.send(Message::Text(subscribe_json)) {
                    Ok(_) => Self::mark_swap_as_tracked(&id, swap.swap_type()),
                    Err(e) => error!("Failed to subscribe to {id}: {e:?}"),
                },
                Err(e) => error!("Invalid subscription msg: {e:?}"),
            }
        }
    }
}
