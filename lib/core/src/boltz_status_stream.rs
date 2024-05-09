use std::collections::HashSet;
use std::mem::swap;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{anyhow, ensure, Result};
use boltz_client::swaps::{
    boltz::{RevSwapStates, SubSwapStates},
    boltzv2::{Subscription, SwapUpdate},
};
use log::{error, info, warn};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

use crate::model::*;
use crate::sdk::LiquidSdk;

pub(super) struct BoltzStatusStream {
    // socket: WebSocket<MaybeTlsStream<TcpStream>>,
}
impl BoltzStatusStream {
    pub(super) fn track_pending_swaps(sdk: Arc<LiquidSdk>) -> Result<()> {
        // Track subscribed swap IDs
        let swap_in_ids = Arc::new(Mutex::new(HashSet::new()));
        let swap_out_ids = Arc::new(Mutex::new(HashSet::new()));

        let mut socket = sdk
            .boltz_client_v2()
            .connect_ws()
            .map_err(|e| anyhow!("Failed to connect to websocket: {e:?}"))?;

        thread::spawn(move || loop {
            let maybe_subscribe_fn =
                |ongoing_swap: &OngoingSwap, socket: &mut WebSocket<MaybeTlsStream<TcpStream>>| {
                    let id = ongoing_swap.id();

                    let is_ongoing_swap_already_tracked = match ongoing_swap {
                        OngoingSwap::Send(_) => swap_in_ids.lock().unwrap().contains(&id),
                        OngoingSwap::Receive(_) => swap_out_ids.lock().unwrap().contains(&id),
                    };

                    if !is_ongoing_swap_already_tracked {
                        info!("Subscribing to status for ongoing swap ID {id}");

                        let subscription = Subscription::new(&id);
                        let subscribe_json = serde_json::to_string(&subscription)
                            .map_err(|e| anyhow!("Invalid subscription msg: {e:?}"))
                            .unwrap();
                        socket
                            .send(tungstenite::Message::Text(subscribe_json))
                            .map_err(|e| anyhow!("Failed to subscribe to {id}: {e:?}"))
                            .unwrap();

                        match ongoing_swap {
                            OngoingSwap::Send(_) => swap_in_ids.lock().unwrap().insert(id),
                            OngoingSwap::Receive(_) => swap_out_ids.lock().unwrap().insert(id),
                        };
                    }
                };

            // Initially subscribe to all ongoing swaps
            match sdk.list_ongoing_swaps() {
                Ok(initial_ongoing_swaps) => {
                    info!("Got {} initial ongoing swaps", initial_ongoing_swaps.len());
                    for ongoing_swap in &initial_ongoing_swaps {
                        maybe_subscribe_fn(ongoing_swap, &mut socket);
                    }
                }
                Err(e) => error!("Failed to list initial ongoing swaps: {e:?}"),
            }

            loop {
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
                                    maybe_subscribe_fn(ongoing_swap, &mut socket);
                                }
                            }
                            Err(e) => error!("Failed to list new ongoing swaps: {e:?}"),
                        }

                        // We parse and handle any Text websocket messages, which are likely status updates
                        if msg.is_text() {
                            let response: SwapUpdate = serde_json::from_str(&msg.to_string())
                                .map_err(|e| anyhow!("WS response is invalid SwapUpdate: {e:?}"))
                                .unwrap();
                            info!("Received update : {response:?}");

                            match response {
                                // Subscription confirmation
                                boltz_client::swaps::boltzv2::SwapUpdate::Subscription {
                                    ..
                                } => {}

                                // Status update
                                boltz_client::swaps::boltzv2::SwapUpdate::Update {
                                    event,
                                    channel,
                                    args,
                                } => {
                                    let update = args.first().unwrap().clone(); // TODO
                                    let update_swap_id = update.id.clone();
                                    let update_state_str = update.status.clone();

                                    if swap_in_ids.lock().unwrap().contains(&update_swap_id) {
                                        // Known OngoingSwapIn / Send swap

                                        let new_state = SubSwapStates::from_str(&update_state_str).map_err(|_| {
                                            anyhow!("Invalid state for submarine swap {update_swap_id}: {update_state_str}")
                                        }).unwrap();
                                        let res = sdk.try_handle_submarine_swap_status(
                                            new_state,
                                            &update_swap_id,
                                        );
                                        info!("OngoingSwapIn / Send try_handle_submarine_swap_status res: {res:?}");
                                    } else if swap_out_ids.lock().unwrap().contains(&update_swap_id)
                                    {
                                        // Known OngoingSwapOut / receive swap

                                        let new_state = RevSwapStates::from_str(&update_state_str).map_err(|_| {
                                            anyhow!("Invalid state for reverse swap {update_swap_id}: {update_state_str}")
                                        }).unwrap();
                                        let res = sdk.try_handle_reverse_swap_status(
                                            new_state,
                                            &update_swap_id,
                                        );
                                        info!("OngoingSwapOut / receive try_handle_reverse_swap_status res: {res:?}");
                                    } else {
                                        // We got an update for a swap we did not track as ongoing
                                        todo!()
                                    }
                                }

                                // Error related to subscription, like "Unknown swap ID"
                                boltz_client::swaps::boltzv2::SwapUpdate::Error { .. } => todo!(),
                            }
                        }
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
}
