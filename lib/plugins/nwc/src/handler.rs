use std::sync::{Arc, Weak};

use crate::model::{
    ListPaymentsRequest, PayAmount, Payment, PaymentDetails, PaymentState, PaymentType,
    PrepareSendRequest, SendPaymentRequest,
};
use crate::sdk::LiquidSdk;
use log::info;
use nostr_sdk::nips::nip47::{
    ErrorCode, GetBalanceResponse, ListTransactionsRequest, LookupInvoiceResponse, NIP47Error,
    PayInvoiceRequest, PayInvoiceResponse, TransactionType,
};
use nostr_sdk::Timestamp;

type Result<T> = std::result::Result<T, NIP47Error>;

#[sdk_macros::async_trait]
pub trait RelayMessageHandler: Send + Sync {
    async fn pay_invoice(&self, req: PayInvoiceRequest) -> Result<PayInvoiceResponse>;
    async fn list_transactions(
        &self,
        req: ListTransactionsRequest,
    ) -> Result<Vec<LookupInvoiceResponse>>;
    async fn get_balance(&self) -> Result<GetBalanceResponse>;
}

pub struct SdkRelayMessageHandler {
    sdk: Weak<LiquidSdk>,
}

impl SdkRelayMessageHandler {
    pub fn new(sdk: Weak<LiquidSdk>) -> Self {
        Self { sdk }
    }

    fn get_sdk(&self) -> Result<Arc<LiquidSdk>> {
        let Some(sdk) = self.sdk.upgrade() else {
            return Err(NIP47Error {
                code: ErrorCode::Internal,
                message: "Could not handle message: SDK is not running.".to_string(),
            });
        };
        Ok(sdk)
    }
}

#[sdk_macros::async_trait]
impl RelayMessageHandler for SdkRelayMessageHandler {
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
    async fn pay_invoice(&self, req: PayInvoiceRequest) -> Result<PayInvoiceResponse> {
        // Create prepare request
        info!("NWC Pay invoice is called");
        let sdk = self.get_sdk()?;
        let prepare_req = PrepareSendRequest {
            destination: req.invoice,
            amount: req.amount.map(|a| PayAmount::Bitcoin {
                receiver_amount_sat: a / 1000,
            }),
            disable_mrh: Some(true),
            payment_timeout_sec: Some(180), // 3 minutes timeout
        };

        // Prepare the payment
        let prepare_resp =
            sdk.prepare_send_payment(&prepare_req)
                .await
                .map_err(|e| NIP47Error {
                    code: ErrorCode::PaymentFailed,
                    message: format!("Failed to prepare payment: {e}"),
                })?;

        // Create send request
        let send_req = SendPaymentRequest {
            prepare_response: prepare_resp,
            use_asset_fees: None,
            payer_note: None,
        };

        // Send the payment
        let response = sdk.send_payment(&send_req).await.map_err(|e| NIP47Error {
            code: ErrorCode::PaymentFailed,
            message: format!("Failed to send payment: {e}"),
        })?;

        // Extract preimage and fees from payment
        let PaymentDetails::Lightning {
            preimage: Some(preimage),
            ..
        } = response.payment.details
        else {
            return Err(NIP47Error {
                code: ErrorCode::PaymentFailed,
                message: "Payment did not return any preimage".to_string(),
            });
        };

        let fees_paid = response.payment.fees_sat * 1000; // Convert sats to msats

        Ok(PayInvoiceResponse {
            preimage,
            fees_paid: Some(fees_paid),
        })
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
        req: ListTransactionsRequest,
    ) -> Result<Vec<LookupInvoiceResponse>> {
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
            .get_sdk()?
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
            .await
            .map_err(|e| NIP47Error {
                code: ErrorCode::Internal,
                message: format!("Failed to list payments: {e}"),
            })?;

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
    async fn get_balance(&self) -> Result<GetBalanceResponse> {
        info!("NWC Get balance is called");
        let info = self.get_sdk()?.get_info().await.map_err(|e| NIP47Error {
            code: ErrorCode::Internal,
            message: format!("Failed to get wallet info: {e}"),
        })?;

        let balance_msats = info.wallet_info.balance_sat * 1000;

        Ok(GetBalanceResponse {
            balance: balance_msats,
        })
    }
}
