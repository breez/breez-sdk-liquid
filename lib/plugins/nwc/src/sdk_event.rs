use std::sync::Arc;

use breez_sdk_liquid::{
    model::{EventListener, Payment, PaymentDetails, PaymentType, SdkEvent},
    InputType,
};
use log::{info, warn};
use nostr_sdk::{
    nips::nip47::{
        NostrWalletConnectURI, Notification, NotificationResult, NotificationType,
        PaymentNotification, TransactionType,
    },
    Alphabet, EventBuilder, Keys, Kind, Tag, TagKind, TagStandard, Timestamp,
};

use crate::{context::RuntimeContext, encrypt::EncryptionHandler};

enum NotificationKind {
    NIP04 = 23196,
    NIP44 = 23197,
}

pub(crate) struct SdkEventListener {
    ctx: Arc<RuntimeContext>,
    active_uris: Vec<NostrWalletConnectURI>,
}

impl SdkEventListener {
    pub fn new(ctx: Arc<RuntimeContext>, active_uris: Vec<NostrWalletConnectURI>) -> Self {
        Self { ctx, active_uris }
    }

    async fn handle_notif_to_relay(&self, payment: &Payment) {
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

        for uri in self.active_uris.iter() {
            let nwc_client_keypair = Keys::new(uri.secret.clone());
            let encryption_handler = EncryptionHandler::new(
                self.ctx.our_keys.secret_key(),
                &nwc_client_keypair.public_key,
            );

            for kind in [NotificationKind::NIP04, NotificationKind::NIP44] {
                let enc = match kind {
                    NotificationKind::NIP04 => EncryptionHandler::nip04_encrypt,
                    NotificationKind::NIP44 => EncryptionHandler::nip44_encrypt,
                };
                let encrypted_content = match enc(&encryption_handler, &notification_content) {
                    Ok(encrypted) => encrypted,
                    Err(e) => {
                        warn!("Could not encrypt notification content: {e:?}");
                        continue;
                    }
                };

                let event_builder = EventBuilder::new(Kind::Custom(kind as u16), encrypted_content)
                    .tags([Tag::public_key(uri.public_key)]);

                if let Err(e) = self.ctx.send_event(event_builder).await {
                    warn!("Could not send notification event to relay: {e:?}");
                } else {
                    info!("Sent payment notification to relay");
                }
            }
        }
    }

    async fn handle_zap_receipt(&self, payment: &Payment) {
        let PaymentDetails::Lightning {
            invoice: Some(invoice),
            preimage,
            ..
        } = &payment.details
        else {
            return;
        };

        let Some(zap_request) = self.ctx.tracked_zaps.lock().await.remove(invoice) else {
            return;
        };

        let Ok(InputType::Bolt11 { invoice }) = self.ctx.sdk.parse(invoice).await else {
            warn!("Could not parse bolt11 invoice for tracked zap");
            return;
        };

        let mut eb = EventBuilder::new(Kind::ZapReceipt, "")
            .custom_created_at(Timestamp::from_secs(invoice.timestamp));

        // Verify zap_request
        // https://github.com/nostr-protocol/nips/blob/master/57.md#appendix-e-zap-receipt-event

        // Insert `p` tag
        let Some(p_tag) = zap_request.tags.find(TagKind::p()) else {
            warn!("No `p` tag found for zap request. Aborting receipt.");
            return;
        };
        eb = eb.tag(p_tag.clone());

        // Insert e, a, and P tag if present
        for tag_kind in [
            TagKind::a(),
            TagKind::e(),
            TagKind::single_letter(Alphabet::P, true),
        ] {
            if let Some(tag) = zap_request.tags.find(tag_kind) {
                eb = eb.tag(tag.clone());
            }
        }
        // Insert bolt11 tag
        eb = eb.tag(Tag::from_standardized(TagStandard::Bolt11(
            invoice.description_hash.unwrap_or("".to_string()),
        )));
        // Insert description tag
        let Ok(zap_request_json) = serde_json::to_string(&zap_request) else {
            warn!("Could not encode zap request in JSON");
            return;
        };
        eb = eb.tag(Tag::from_standardized(TagStandard::Description(
            zap_request_json,
        )));
        // Insert preimage tag
        if let Some(preimage) = preimage {
            eb = eb.tag(Tag::from_standardized(TagStandard::Preimage(
                preimage.clone(),
            )));
        }

        // Sign and send
        let Ok(zap_receipt) = eb.sign_with_keys(&self.ctx.our_keys) else {
            warn!("Could not sign zap receipt.");
            return;
        };
        if let Err(err) = self.ctx.client.send_event(&zap_receipt).await {
            warn!("Coult not broadcast zap receipt: {err}");
        }
    }
}

#[sdk_macros::async_trait]
impl EventListener for SdkEventListener {
    async fn on_event(&self, e: SdkEvent) {
        let SdkEvent::PaymentSucceeded { details: payment } = e else {
            return;
        };
        self.handle_notif_to_relay(&payment).await;
        self.handle_zap_receipt(&payment).await;
    }
}
