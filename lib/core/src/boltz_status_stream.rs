use std::collections::HashMap;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::Arc;
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
        let mut socket = sdk
            .boltz_client_v2()
            .connect_ws()
            .map_err(|e| anyhow!("Failed to connect to websocket: {e:?}"))?;

        thread::spawn(move || loop {
            // Map of (subscribed swap ID, is_swap_out)
            let mut subscribed_ids: HashMap<String, bool> = HashMap::new();

            // Initially subscribe to all ongoing swaps
            match sdk.list_ongoing_swaps() {
                Ok(initial_ongoing_swaps) => {
                    info!("Got {} initial ongoing swaps", initial_ongoing_swaps.len());

                    for ongoing_swap in &initial_ongoing_swaps {
                        let id = &ongoing_swap.id();
                        info!("Subscribing to status for initial ongoing swap ID {id}");

                        let subscription = Subscription::new(id);
                        let subscribe_json = serde_json::to_string(&subscription)
                            .map_err(|e| anyhow!("Invalid subscription msg: {e:?}"))
                            .unwrap();
                        socket
                            .send(tungstenite::Message::Text(subscribe_json))
                            .map_err(|e| anyhow!("Failed to subscribe to {id}: {e:?}"))
                            .unwrap();

                        subscribed_ids
                            .insert(id.clone(), matches!(ongoing_swap, OngoingSwap::Receive(_)));
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
                                let new_ongoing_swaps: Vec<OngoingSwap> = ongoing_swaps
                                    .into_iter()
                                    .filter(|os| !subscribed_ids.contains_key(&os.id()))
                                    .collect();
                                for ongoing_swap in &new_ongoing_swaps {
                                    let id = ongoing_swap.id();
                                    info!("Subscribing to statuses for ongoing swap ID: {id}");

                                    let subscription = Subscription::new(&id);
                                    let subscribe_json = serde_json::to_string(&subscription)
                                        .map_err(|e| anyhow!("Invalid subscription msg: {e:?}"))
                                        .unwrap();
                                    socket
                                        .send(tungstenite::Message::Text(subscribe_json))
                                        .map_err(|e| anyhow!("Failed to subscribe to {id}: {e:?}"))
                                        .unwrap();

                                    subscribed_ids.insert(
                                        id.clone(),
                                        matches!(ongoing_swap, OngoingSwap::Receive(_)),
                                    );
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

                                    match subscribed_ids.get(&update_swap_id) {
                                        Some(true) => {
                                            // Known OngoingSwapOut / receive swap

                                            let new_state = RevSwapStates::from_str(&update_state_str).map_err(|_| {
                                                anyhow!("Invalid state for reverse swap {update_swap_id}: {update_state_str}")
                                            }).unwrap();
                                            let res = sdk.try_handle_reverse_swap_status(
                                                new_state,
                                                &update_swap_id,
                                            );
                                            info!("OngoingSwapOut / receive try_handle_reverse_swap_status res: {res:?}");
                                        }
                                        Some(false) => {
                                            // Known OngoingSwapIn / Send swap

                                            let new_state = SubSwapStates::from_str(&update_state_str).map_err(|_| {
                                                anyhow!("Invalid state for submarine swap {update_swap_id}: {update_state_str}")
                                            }).unwrap();
                                            let res = sdk.try_handle_submarine_swap_status(
                                                new_state,
                                                &update_swap_id,
                                            );
                                            info!("OngoingSwapIn / Send try_handle_submarine_swap_status res: {res:?}");
                                        }
                                        None => {
                                            // We got an update for a swap we did not track as ongoing
                                            todo!()
                                        }
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
