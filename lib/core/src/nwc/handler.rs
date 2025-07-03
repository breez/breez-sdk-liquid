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
    sdk: sdk_common::utils::Arc<LiquidSdk>,
}

impl BreezRelayMessageHandler {
    pub fn new(sdk: sdk_common::utils::Arc<LiquidSdk>) -> Self {
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
                code: ErrorCode::PaymentFailed,
                message: "Payment did not return any preimage".to_string(),
            });
        };
        // TODO: Switch to custom fork, done
        let fees_paid = response.payment.fees_sat * 1000; // Convert sats to msats

        Ok(PayInvoiceResponse { preimage, fees_paid: Some(fees_paid) })
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

    /// Retrieves the current wallet balance.
    /// 
    /// # Returns
    /// * `Ok(GetBalanceResponse)` - Balance in millisatoshis
    /// * `Err(NIP47Error)` - Error getting wallet info from the SDK
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_utils::{persist::create_persister,sdk::new_liquid_sdk, status_stream::MockStatusStream, swapper::MockSwapper};
  use std::sync::Arc;

  //TODO: return sdk and BreezRelayMessageHandler, write a macro, done.
  #[macro_export]
  macro_rules! setup_test_handler {
    ($sdk:ident, $handler:ident) => {
      create_persister!(persister);
      let swapper = Arc::new(MockSwapper::default());
      let status_stream = Arc::new(MockStatusStream::new());

      let $sdk =
        new_liquid_sdk(persister.clone(), swapper.clone(), status_stream.clone())
          .await
          .unwrap();
      
      // Start the SDK
      $sdk.start().await.unwrap();

      let $handler = BreezRelayMessageHandler::new($sdk.clone());
    };
  }


  #[sdk_macros::async_test_all] //TODO: use sdk_macros, done.
  async fn test_pay_invoice() -> anyhow::Result<()> {
    setup_test_handler!(sdk, handler);

    let invoice = "lntb1u1p5ymklqsp57u297u963mzxj4e5q0d3p424pjqr7uzd6jtps9mgejgaf6vnau9spp5svzg0d36d04kmsaz9x674r9gg0a8xr6sla5atmj2u2kumlsemppsdq6w3jhxapqv3jhxcmjd9c8g6t0dcxqyjw5qcqp29qxpqysgqlyuk2zmjwq0a9285atyukjql3qxwd8lpy8aj9fypwxst3lzlw4cxamtnxtwvz7m82gwspprsr6dzlu4n2dpgtckfjlkyfg0qyh5mc8spu0chpn";

    // Create a test invoice request
    let request = PayInvoiceRequest {
        id: None,
        invoice: invoice.to_string(),
        amount: Some(100000),
    };

    // Test the pay_invoice function
    let result = handler.pay_invoice(request).await;
    if let Err(e) = &result {
      println!("pay_invoice failed with error: {:?}", e);
    }
    assert!(result.is_ok());
    
    let response = result.unwrap();
    //assert!(Some(response.preimage)); //TODO: check if the preimage is valid, HOWW???
    assert!(response.fees_paid.is_some());
    Ok(())
  }

  #[sdk_macros::async_test_all]
  async fn test_list_transactions() -> anyhow::Result<()> {
    setup_test_handler!(sdk, handler);
    // TODO: exectue a send payment, done.
    test_send_payment(sdk.clone()).await;

    // Create a test list transactions request
    let request = ListTransactionsRequest {
      limit: Some(10),
      offset: Some(0),
      transaction_type: Some(TransactionType::Outgoing),
      unpaid: Some(false),
      from: None,
      until: None,
    };

    let result = handler.list_transactions(request).await;
    match result {
      Ok(response) => {
        // Print transactions
        println!("\n=== Transactions ===");
        for (i, transaction) in response.iter().enumerate() {
          println!("\nTransaction {}:", i + 1);
          println!("Invoice: {}", transaction.invoice.as_deref().unwrap_or("N/A"));
          println!("Amount: {} sats", transaction.amount);
          println!("Type: {:?}", transaction.transaction_type);
          println!("Created at: {}", transaction.created_at);
          if let Some(settled_at) = transaction.settled_at {
            println!("Settled at: {}", settled_at);
          }
        }

      assert!(!response.is_empty());
      let transaction = &response[0];
      assert!(!transaction.invoice.as_ref().map_or(false, |s| s.is_empty()));
      assert!(transaction.amount > 0);
    }
    Err(e) => panic!("list_transactions failed: {:?}", e),
}
    Ok(())
  }

  #[sdk_macros::async_test_all]
  async fn test_get_balance() -> anyhow::Result<()> {
    setup_test_handler!(sdk, handler);
    // TODO: exectue a send payment, done.
    test_send_payment(sdk.clone()).await;
    let result = handler.get_balance().await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(response.balance >= 0);
    Ok(())
  }

  pub async fn test_send_payment(sdk: Arc<LiquidSdk>) {
    let invoice = "lntb1u1p5ymklqsp57u297u963mzxj4e5q0d3p424pjqr7uzd6jtps9mgejgaf6vnau9spp5svzg0d36d04kmsaz9x674r9gg0a8xr6sla5atmj2u2kumlsemppsdq6w3jhxapqv3jhxcmjd9c8g6t0dcxqyjw5qcqp29qxpqysgqlyuk2zmjwq0a9285atyukjql3qxwd8lpy8aj9fypwxst3lzlw4cxamtnxtwvz7m82gwspprsr6dzlu4n2dpgtckfjlkyfg0qyh5mc8spu0chpn";

    let prepare_request = PrepareSendRequest {
      comment: None,
      destination: invoice.to_string(),
      amount: None,
    };

    match sdk.prepare_send_payment(&prepare_request).await {
      Ok(prepare_response) => {
        let send_request = SendPaymentRequest {
          prepare_response,
          use_asset_fees: None,
        };

        match sdk.send_payment(&send_request).await {
          Ok(response) => {
            println!("Payment successful! Payment details: {:?}", response.payment);
          }
          Err(e) => {
            println!("Failed to send payment: {}", e);
          }
        }
      }
      Err(e) => {
        println!("Failed to prepare payment: {}", e);
      }
    }
  }
}