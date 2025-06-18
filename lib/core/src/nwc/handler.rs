use crate::model::{
    ListPaymentsRequest, PayAmount, Payment, PaymentDetails, PaymentState, PaymentType,
    PrepareSendRequest, SendPaymentRequest,
};
use crate::sdk::LiquidSdk;
use nostr_sdk::nips::nip47::{
    ErrorCode, GetBalanceResponse, ListTransactionsRequest, LookupInvoiceResponse, NIP47Error,
    PayInvoiceRequest, PayInvoiceResponse, TransactionType,
};
use nostr_sdk::Timestamp;
use std::sync::Arc;

type Result<T> = std::result::Result<T, NIP47Error>;

pub trait RelayMessageHandler {
    async fn pay_invoice(&self, req: PayInvoiceRequest) -> Result<PayInvoiceResponse>;
    async fn list_transactions(
        &self,
        req: ListTransactionsRequest,
    ) -> Result<Vec<LookupInvoiceResponse>>;
    async fn get_balance(&self) -> Result<GetBalanceResponse>;
}

pub struct BreezRelayMessageHandler {
    sdk: Arc<LiquidSdk>,
}

impl BreezRelayMessageHandler {
    pub fn new(sdk: Arc<LiquidSdk>) -> Self {
        Self { sdk }
    }
}

impl From<TransactionType> for PaymentType {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Incoming => Self::Receive,
            TransactionType::Outgoing => Self::Send,
        }
    }
}

impl Into<TransactionType> for PaymentType {
    fn into(self) -> TransactionType {
        match self {
            Self::Receive => TransactionType::Incoming,
            Self::Send => TransactionType::Outgoing,
        }
    }
}

impl RelayMessageHandler for BreezRelayMessageHandler {
    async fn pay_invoice(&self, req: PayInvoiceRequest) -> Result<PayInvoiceResponse> {
        // Create prepare request
        let prepare_req = PrepareSendRequest {
            comment: None,
            destination: req.invoice,
            amount: req.amount.map(|a| PayAmount::Bitcoin {
                receiver_amount_sat: a / 1000,
            }),
        };

        // Prepare the payment
        let prepare_resp = self
            .sdk
            .prepare_send_payment(&prepare_req)
            .await
            .map_err(|e| NIP47Error {
                code: ErrorCode::PaymentFailed,
                message: format!("Failed to prepare payment: {}", e),
            })?;

        // Create send request
        let send_req = SendPaymentRequest {
            prepare_response: prepare_resp,
            use_asset_fees: None,
        };

        // Send the payment
        let response = self
            .sdk
            .send_payment(&send_req)
            .await
            .map_err(|e| NIP47Error {
                code: ErrorCode::PaymentFailed,
                message: format!("Failed to send payment: {}", e),
            })?;

        // Extract preimage and fees from payment
        let PaymentDetails::Lightning {
            preimage: Some(preimage),
            ..
        } = response.payment.details
        else {
            return Err(NIP47Error {
                code: ErrorCode::Internal,
                message: "Payment did not return any preimage".to_string(),
            });
        };
        // TODO: Switch to custom fork
        // let fees_paid = response.payment.fees_sat * 1000; // Convert sats to msats

        Ok(PayInvoiceResponse { preimage })
    }

    async fn list_transactions(
        &self,
        req: ListTransactionsRequest,
    ) -> Result<Vec<LookupInvoiceResponse>> {
        let filters = req.transaction_type.map(|p| vec![p.into()]);

        let states = req
            .unpaid
            .map(|unpaid| {
                if unpaid {
                    return Some(vec![PaymentState::Pending]);
                }
                None
            })
            .flatten();

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
            .await
            .map_err(|e| NIP47Error {
                code: ErrorCode::Internal,
                message: format!("Failed to list payments: {}", e),
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
                    payment_hash: payment_hash.unwrap_or("null".to_string()), // TODO: Set this as optional?
                    transaction_type: Some(Into::<TransactionType>::into(payment.payment_type)),
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

    async fn get_balance(&self) -> Result<GetBalanceResponse> {
        let info = self.sdk.get_info().await.map_err(|e| NIP47Error {
            code: ErrorCode::Internal,
            message: format!("Failed to get wallet info: {}", e),
        })?;

        let balance_msats = info.wallet_info.balance_sat * 1000;

        Ok(GetBalanceResponse {
            balance: balance_msats,
        })
    }
}
