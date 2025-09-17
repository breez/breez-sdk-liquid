use std::collections::HashMap;

use breez_sdk_liquid::model::{EventListener, PaymentDetails, PaymentType, SdkEvent};
use log::{info, warn};
use nostr_sdk::{
    nips::nip44::{encrypt, Version},
    nips::nip47::{
        NostrWalletConnectURI, Notification, NotificationResult, NotificationType,
        PaymentNotification, TransactionType,
    },
    EventBuilder, Keys, Kind, Tag, Timestamp,
};
use sdk_common::utils::Arc;

use crate::context::RuntimeContext;

pub(crate) struct SdkEventListener {
    ctx: Arc<RuntimeContext>,
    clients: HashMap<String, NostrWalletConnectURI>,
}

impl SdkEventListener {
    pub fn new(ctx: Arc<RuntimeContext>, clients: HashMap<String, NostrWalletConnectURI>) -> Self {
        Self { ctx, clients }
    }
}

#[sdk_macros::async_trait]
impl EventListener for SdkEventListener {
    async fn on_event(&self, e: SdkEvent) {
        let SdkEvent::PaymentSucceeded { details: payment } = e else {
            return;
        };

        let (invoice, description, preimage, payment_hash) = match &payment.details {
            PaymentDetails::Lightning {
                invoice,
                description,
                preimage,
                payment_hash,
                ..
            } => (
                invoice.clone().unwrap_or_default(),
                description.clone(),
                preimage.clone().unwrap_or_default(),
                payment_hash.clone().unwrap_or_default(),
            ),
            _ => {
                return;
            }
        };

        let payment_notification = PaymentNotification {
            transaction_type: Some(if payment.payment_type == PaymentType::Send {
                TransactionType::Outgoing
            } else {
                TransactionType::Incoming
            }),
            invoice,
            description: Some(description),
            description_hash: None,
            preimage,
            payment_hash,
            amount: payment.amount_sat * 1000,
            fees_paid: payment.fees_sat * 1000,
            created_at: Timestamp::from_secs(payment.timestamp as u64),
            expires_at: None,
            settled_at: Timestamp::from_secs(payment.timestamp as u64),
            metadata: None,
        };

        let notification = if payment.payment_type == PaymentType::Send {
            Notification {
                notification_type: NotificationType::PaymentSent,
                notification: NotificationResult::PaymentSent(payment_notification),
            }
        } else {
            Notification {
                notification_type: NotificationType::PaymentReceived,
                notification: NotificationResult::PaymentReceived(payment_notification),
            }
        };

        let notification_content = match serde_json::to_string(&notification) {
            Ok(content) => content,
            Err(e) => {
                warn!("Could not serialize notification: {e:?}");
                return;
            }
        };

        for uri in self.clients.values() {
            let nwc_client_keypair = Keys::new(uri.secret.clone());
            let encrypted_content = match encrypt(
                self.ctx.our_keys.secret_key(),
                &nwc_client_keypair.public_key,
                &notification_content,
                Version::V2,
            ) {
                Ok(encrypted) => encrypted,
                Err(e) => {
                    warn!("Could not encrypt notification content: {e:?}");
                    continue;
                }
            };

            let event_builder = EventBuilder::new(Kind::Custom(23196), encrypted_content)
                .tags([Tag::public_key(uri.public_key)]);

            if let Err(e) = self.ctx.send_event(event_builder).await {
                warn!("Could not send notification event to relay: {e:?}");
            } else {
                info!("Sent payment notification to relay");
            }
        }
    }
}
