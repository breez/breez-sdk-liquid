use std::str::FromStr as _;
use std::time::Duration;

use anyhow::{anyhow, bail};
use breez_sdk_liquid::model::{
    DescriptionHash, EventListener, ListPaymentsRequest, PayAmount, Payment, PaymentDetails,
    PaymentMethod, PaymentState, PaymentType, PrepareReceiveRequest, PrepareSendRequest,
    ReceiveAmount, ReceivePaymentRequest, SdkEvent, SendPaymentRequest, SendPaymentResponse,
};
use breez_sdk_liquid::plugin::PluginSdk;
use log::info;
use nostr_sdk::nips::nip47::{
    GetBalanceResponse, GetInfoResponse, ListTransactionsRequest, LookupInvoiceResponse,
    MakeInvoiceRequest, MakeInvoiceResponse, PayInvoiceRequest, PayInvoiceResponse,
    TransactionType,
};
use nostr_sdk::Timestamp;
use sdk_common::prelude::InputType;
use tokio::sync::oneshot::{self, Sender};
use tokio::sync::Mutex;
use tokio_with_wasm::alias as tokio;

use crate::error::{NwcError, NwcResult};

pub(crate) const NWC_SUPPORTED_METHODS: [&str; 5] = [
    "pay_invoice",
    "make_invoice",
    "list_transactions",
    "get_balance",
    "get_info",
];

#[sdk_macros::async_trait]
pub trait RelayMessageHandler: Send + Sync {
    async fn make_invoice(&self, req: &MakeInvoiceRequest) -> NwcResult<MakeInvoiceResponse>;
    async fn pay_invoice(&self, req: &PayInvoiceRequest) -> NwcResult<PayInvoiceResponse>;
    async fn list_transactions(
        &self,
        req: &ListTransactionsRequest,
    ) -> NwcResult<Vec<LookupInvoiceResponse>>;
    async fn get_balance(&self) -> NwcResult<GetBalanceResponse>;
    async fn get_info(&self) -> NwcResult<GetInfoResponse>;
}

pub struct SdkRelayMessageHandler {
    sdk: PluginSdk,
}

impl SdkRelayMessageHandler {
    pub fn new(sdk: PluginSdk) -> Self {
        Self { sdk }
    }
}

struct PreimageListener {
    payment_tx_id: String,
    preimage_listener_tx: Mutex<Option<Sender<NwcResult<String>>>>,
}

impl PreimageListener {
    pub(crate) fn new(
        payment_tx_id: String,
        preimage_listener_tx: Sender<NwcResult<String>>,
    ) -> Self {
        Self {
            payment_tx_id,
            preimage_listener_tx: Mutex::new(Some(preimage_listener_tx)),
        }
    }
}

#[sdk_macros::async_trait]
impl EventListener for PreimageListener {
    async fn on_event(&self, e: SdkEvent) {
        let (payment, succeeded) = match e {
            SdkEvent::PaymentSucceeded { details } => (details, true),
            SdkEvent::PaymentFailed { details } => (details, false),
            _ => return,
        };
        if payment
            .tx_id
            .is_some_and(|tx_id| tx_id == self.payment_tx_id)
        {
            return;
        }

        let payload = match succeeded {
            true => match payment.details {
                PaymentDetails::Lightning {
                    preimage: Some(preimage),
                    ..
                } => Ok(preimage),
                _ => Err(NwcError::generic(
                    "Unexpected payment data was returned from the SDK",
                )),
            },
            false => Err(NwcError::generic(
                "Could not send payment. Check the logs for more information.",
            )),
        };
        if let Some(tx) = self.preimage_listener_tx.lock().await.take() {
            let _ = tx.send(payload);
        }
    }
}

impl SdkRelayMessageHandler {
    async fn wait_for_preimage(
        &self,
        response: SendPaymentResponse,
    ) -> anyhow::Result<PayInvoiceResponse> {
        let Some(tx_id) = response.payment.tx_id else {
            bail!("Unexpected payment data was returned from the SDK")
        };
        let (preimage_listener_tx, preimage_listener_rx) = oneshot::channel();
        let preimage_event_listener = Box::new(PreimageListener::new(tx_id, preimage_listener_tx));
        let listener_id = self.sdk.add_event_listener(preimage_event_listener).await?;
        let payment_timeout_fut =
            tokio::time::timeout(Duration::from_secs(180), preimage_listener_rx).await??;
        let result = match payment_timeout_fut {
            Ok(preimage) => {
                let fees_paid = response.payment.fees_sat * 1000; // Convert sats to msats
                Ok(PayInvoiceResponse {
                    preimage,
                    fees_paid: Some(fees_paid),
                })
            }
            Err(err) => Err(anyhow!("Could not retrieve payment preimage: {err}")),
        };
        let _ = self.sdk.remove_event_listener(listener_id).await;
        result
    }
}

#[sdk_macros::async_trait]
impl RelayMessageHandler for SdkRelayMessageHandler {
    async fn make_invoice(&self, req: &MakeInvoiceRequest) -> NwcResult<MakeInvoiceResponse> {
        info!("NWC Make invoice is called");

        let prepare_req = PrepareReceiveRequest {
            payment_method: PaymentMethod::Bolt11Invoice,
            amount: Some(ReceiveAmount::Bitcoin {
                payer_amount_sat: req.amount.div_ceil(1000),
            }),
        };

        let prepare_response = self.sdk.prepare_receive_payment(&prepare_req).await?;
        let receive_response = self
            .sdk
            .receive_payment(&ReceivePaymentRequest {
                prepare_response,
                description: req.description.clone(),
                description_hash: req
                    .description_hash
                    .clone()
                    .map(|hash| DescriptionHash::Custom { hash }),
                payer_note: None,
            })
            .await?;

        let Ok(InputType::Bolt11 { invoice }) = self.sdk.parse(&receive_response.destination).await
        else {
            return Err(NwcError::generic("Could not parse SDK invoice"));
        };

        Ok(MakeInvoiceResponse {
            invoice: invoice.bolt11,
            payment_hash: invoice.payment_hash,
        })
    }

    /// Processes a Lightning invoice payment request.
    ///
    /// This method handles the complete payment flow:
    /// 1. Prepares the payment using the SDK
    /// 2. Executes the payment
    /// 3. Extracts the preimage and fees from the completed payment
    ///
    /// # Arguments
    /// * `req` - Payment request containing invoice and optional amount override
    ///
    /// # Returns
    /// * `Ok(PayInvoiceResponse)` - Contains payment preimage and fees paid
    /// * `Err(NIP47Error)` - Payment preparation or execution error
    async fn pay_invoice(&self, req: &PayInvoiceRequest) -> NwcResult<PayInvoiceResponse> {
        // Create prepare request
        info!("NWC Pay invoice is called");
        let prepare_req = PrepareSendRequest {
            destination: req.invoice.clone(),
            amount: req.amount.map(|a| PayAmount::Bitcoin {
                receiver_amount_sat: a / 1000,
            }),
            disable_mrh: Some(true),
            payment_timeout_sec: Some(180), // 3 minutes timeout
        };

        // Prepare the payment
        let prepare_resp = self.sdk.prepare_send_payment(&prepare_req).await?;

        // Create send request
        let send_req = SendPaymentRequest {
            prepare_response: prepare_resp,
            use_asset_fees: None,
            payer_note: None,
        };

        // Send the payment
        let response = self.sdk.send_payment(&send_req).await?;
        Ok(self.wait_for_preimage(response).await?)
    }

    /// Retrieves a filtered list of wallet transactions.
    ///
    /// This method converts NIP-47 transaction filters to Breez payment filters
    /// and returns transactions in the expected NIP-47 format.
    ///
    /// # Arguments
    /// * `req` - Filter criteria including transaction type, unpaid status, time range, and pagination
    ///
    /// # Returns
    /// * `Ok(Vec<LookupInvoiceResponse>)` - List of transactions matching the filters
    /// * `Err(NIP47Error)` - Error retrieving payments from the SDK
    async fn list_transactions(
        &self,
        req: &ListTransactionsRequest,
    ) -> NwcResult<Vec<LookupInvoiceResponse>> {
        let filters = req.transaction_type.map(|p| {
            vec![match p {
                TransactionType::Incoming => PaymentType::Receive,
                TransactionType::Outgoing => PaymentType::Send,
            }]
        });
        info!("NWC List transactions is called");
        let states = req.unpaid.and_then(|unpaid| {
            if unpaid {
                None
            } else {
                Some(vec![PaymentState::Complete])
            }
        });

        // Get payments from SDK
        let payments: Vec<Payment> = self
            .sdk
            .list_payments(&ListPaymentsRequest {
                filters,
                states,
                from_timestamp: req.from.map(|t| t.as_u64() as i64),
                to_timestamp: req.until.map(|t| t.as_u64() as i64),
                limit: req.limit.map(|l| l as u32),
                offset: req.offset.map(|o| o as u32),
                details: None,
                sort_ascending: Some(false),
            })
            .await?;

        // Convert payments to NIP-47 transactions
        let txs: Vec<LookupInvoiceResponse> = payments
            .into_iter()
            .map(|payment| {
                let (description, preimage, invoice, payment_hash) = match payment.details {
                    PaymentDetails::Lightning {
                        description,
                        preimage,
                        invoice,
                        payment_hash,
                        ..
                    } => (Some(description), preimage, invoice, payment_hash),
                    _ => (None, None, None, None),
                };

                LookupInvoiceResponse {
                    payment_hash: payment_hash.unwrap_or("null".to_string()),
                    transaction_type: Some(match payment.payment_type {
                        PaymentType::Receive => TransactionType::Incoming,
                        PaymentType::Send => TransactionType::Outgoing,
                    }),
                    invoice,
                    description,
                    preimage,
                    amount: payment.amount_sat * 1000,
                    fees_paid: payment.fees_sat * 1000,
                    created_at: Timestamp::from_secs(payment.timestamp as u64),
                    description_hash: None,
                    expires_at: None,
                    settled_at: None,
                    metadata: None,
                }
            })
            .collect();

        Ok(txs)
    }

    /// Retrieves the current wallet balance.
    ///
    /// # Returns
    /// * `Ok(GetBalanceResponse)` - Balance in millisatoshis
    /// * `Err(NIP47Error)` - Error getting wallet info from the SDK
    async fn get_balance(&self) -> NwcResult<GetBalanceResponse> {
        info!("NWC Get balance is called");
        let info = self.sdk.get_info().await?;

        let balance_msats = info.wallet_info.balance_sat * 1000;

        Ok(GetBalanceResponse {
            balance: balance_msats,
        })
    }

    async fn get_info(&self) -> NwcResult<GetInfoResponse> {
        info!("NWC Get info is called");
        let info = self.sdk.get_info().await?;
        Ok(GetInfoResponse {
            alias: None,
            color: None,
            network: None,
            block_hash: None,
            block_height: None,
            pubkey: nostr_sdk::secp256k1::PublicKey::from_str(&info.wallet_info.pubkey).ok(),
            methods: NWC_SUPPORTED_METHODS
                .into_iter()
                .map(String::from)
                .collect(),
            notifications: vec![],
        })
    }
}
