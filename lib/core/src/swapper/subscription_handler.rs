use log::{error, info};
use maybe_sync::{MaybeSend, MaybeSync};
use sdk_common::bitcoin::hashes::hex::ToHex;
use sdk_common::utils::Arc;

use crate::{persist::Persister, utils};

use super::SwapperStatusStream;

#[sdk_macros::async_trait]
pub trait SubscriptionHandler: MaybeSend + MaybeSync {
    async fn track_subscriptions(&self);
}

#[derive(Clone)]
pub(crate) struct SwapperSubscriptionHandler {
    persister: std::sync::Arc<Persister>,
    status_stream: Arc<dyn SwapperStatusStream>,
}

impl SwapperSubscriptionHandler {
    pub(crate) fn new(
        persister: std::sync::Arc<Persister>,
        status_stream: Arc<dyn SwapperStatusStream>,
    ) -> Self {
        Self {
            persister,
            status_stream,
        }
    }
}

#[sdk_macros::async_trait]
impl SubscriptionHandler for SwapperSubscriptionHandler {
    async fn track_subscriptions(&self) {
        match self.persister.list_ongoing_swaps() {
            Ok(initial_ongoing_swaps) => {
                info!(
                    "On stream reconnection, got {} initial ongoing swaps",
                    initial_ongoing_swaps.len()
                );
                for ongoing_swap in initial_ongoing_swaps {
                    match self.status_stream.track_swap_id(&ongoing_swap.id()) {
                        Ok(_) => info!("Tracking ongoing swap: {}", ongoing_swap.id()),
                        Err(e) => error!("Failed to track ongoing swap: {e:?}"),
                    }
                }
            }
            Err(e) => error!("Failed to list initial ongoing swaps: {e:?}"),
        }

        match self.persister.list_bolt12_offers() {
            Ok(initial_bolt12_offers) => {
                info!(
                    "On stream reconnection, got {} initial BOLT12 offers",
                    initial_bolt12_offers.len()
                );
                for bolt12_offer in initial_bolt12_offers {
                    let offer = &bolt12_offer.id;
                    let Ok(keypair) = bolt12_offer.get_keypair() else {
                        error!("Failed to get keypair for BOLT12 offer: {offer}");
                        continue;
                    };
                    let Ok(subscribe_hash_sig) = utils::sign_message_hash("SUBSCRIBE", &keypair)
                    else {
                        error!("Failed to sign hash for BOLT12 offer: {offer}");
                        continue;
                    };

                    match self
                        .status_stream
                        .track_offer(offer, &subscribe_hash_sig.to_hex())
                    {
                        Ok(_) => info!("Tracking bolt12 offer: {offer}"),
                        Err(e) => error!("Failed to track bolt12 offer: {e:?}"),
                    }
                }
            }
            Err(e) => error!("Failed to list initial bolt12 offers: {e:?}"),
        }
    }
}
