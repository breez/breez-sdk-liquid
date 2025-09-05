use std::sync::Arc;

use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;

use crate::errors::*;
use crate::events::BreezEventListener;
use crate::models::*;

pub async fn connect(req: ConnectRequest) -> Result<BreezSdkLiquid, SdkError> {
    let ln_sdk = LiquidSdk::connect(req).await?;
    Ok(BreezSdkLiquid { sdk: ln_sdk })
}

#[frb(sync)]
pub fn default_config(
    network: LiquidNetwork,
    breez_api_key: Option<String>,
) -> Result<Config, SdkError> {
    LiquidSdk::default_config(network, breez_api_key)
}

#[frb(sync)]
pub fn parse_invoice(input: String) -> Result<LNInvoice, PaymentError> {
    LiquidSdk::parse_invoice(&input)
}

pub struct BreezSdkLiquid {
    pub(crate) sdk: Arc<LiquidSdk>,
}

impl BreezSdkLiquid {
    pub async fn get_info(&self) -> Result<GetInfoResponse, SdkError> {
        self.sdk.get_info().await
    }

    #[frb(sync)]
    pub fn sign_message(&self, req: SignMessageRequest) -> Result<SignMessageResponse, SdkError> {
        self.sdk.sign_message(&req)
    }

    #[frb(sync)]
    pub fn check_message(
        &self,
        req: CheckMessageRequest,
    ) -> Result<CheckMessageResponse, SdkError> {
        self.sdk.check_message(&req)
    }

    pub async fn parse(&self, input: String) -> Result<InputType, PaymentError> {
        self.sdk.parse(&input).await
    }

    pub async fn add_event_listener(
        &self,
        listener: StreamSink<SdkEvent>,
    ) -> Result<String, SdkError> {
        self.sdk
            .add_event_listener(Box::new(BreezEventListener { stream: listener }))
            .await
    }

    pub async fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.sdk.prepare_send_payment(&req).await
    }

    pub async fn send_payment(
        &self,
        req: SendPaymentRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk.send_payment(&req).await
    }

    pub async fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.sdk.prepare_receive_payment(&req).await
    }

    pub async fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.sdk.receive_payment(&req).await
    }

    pub async fn create_bolt12_invoice(
        &self,
        req: CreateBolt12InvoiceRequest,
    ) -> Result<CreateBolt12InvoiceResponse, PaymentError> {
        self.sdk.create_bolt12_invoice(&req).await
    }

    pub async fn fetch_lightning_limits(
        &self,
    ) -> Result<LightningPaymentLimitsResponse, PaymentError> {
        self.sdk.fetch_lightning_limits().await
    }

    pub async fn fetch_onchain_limits(&self) -> Result<OnchainPaymentLimitsResponse, PaymentError> {
        self.sdk.fetch_onchain_limits().await
    }

    pub async fn prepare_pay_onchain(
        &self,
        req: PreparePayOnchainRequest,
    ) -> Result<PreparePayOnchainResponse, PaymentError> {
        self.sdk.prepare_pay_onchain(&req).await
    }

    pub async fn pay_onchain(
        &self,
        req: PayOnchainRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk.pay_onchain(&req).await
    }

    pub async fn prepare_buy_bitcoin(
        &self,
        req: PrepareBuyBitcoinRequest,
    ) -> Result<PrepareBuyBitcoinResponse, PaymentError> {
        self.sdk.prepare_buy_bitcoin(&req).await
    }

    pub async fn buy_bitcoin(&self, req: BuyBitcoinRequest) -> Result<String, PaymentError> {
        self.sdk.buy_bitcoin(&req).await
    }

    pub async fn list_payments(
        &self,
        req: ListPaymentsRequest,
    ) -> Result<Vec<Payment>, PaymentError> {
        self.sdk.list_payments(&req).await
    }

    pub async fn get_payment(
        &self,
        req: GetPaymentRequest,
    ) -> Result<Option<Payment>, PaymentError> {
        self.sdk.get_payment(&req).await
    }

    pub async fn fetch_payment_proposed_fees(
        &self,
        req: FetchPaymentProposedFeesRequest,
    ) -> Result<FetchPaymentProposedFeesResponse, SdkError> {
        self.sdk.fetch_payment_proposed_fees(&req).await
    }

    pub async fn accept_payment_proposed_fees(
        &self,
        req: AcceptPaymentProposedFeesRequest,
    ) -> Result<(), PaymentError> {
        self.sdk.accept_payment_proposed_fees(&req).await
    }

    pub async fn prepare_lnurl_pay(
        &self,
        req: PrepareLnUrlPayRequest,
    ) -> Result<PrepareLnUrlPayResponse, crate::duplicates::LnUrlPayError> {
        self.sdk.prepare_lnurl_pay(req).await.map_err(Into::into)
    }

    pub async fn lnurl_pay(
        &self,
        req: breez_sdk_liquid::model::LnUrlPayRequest,
    ) -> Result<LnUrlPayResult, crate::duplicates::LnUrlPayError> {
        self.sdk.lnurl_pay(req).await.map_err(Into::into)
    }

    pub async fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<crate::duplicates::LnUrlWithdrawResult, crate::duplicates::LnUrlWithdrawError> {
        self.sdk
            .lnurl_withdraw(req)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    pub async fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<crate::duplicates::LnUrlCallbackStatus, crate::duplicates::LnUrlAuthError> {
        self.sdk
            .lnurl_auth(req_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    pub async fn register_webhook(&self, webhook_url: String) -> Result<(), SdkError> {
        self.sdk.register_webhook(webhook_url).await
    }

    pub async fn unregister_webhook(&self) -> Result<(), SdkError> {
        self.sdk.unregister_webhook().await
    }

    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, SdkError> {
        self.sdk.fetch_fiat_rates().await
    }

    pub async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, SdkError> {
        self.sdk.list_fiat_currencies().await
    }

    pub async fn list_refundables(&self) -> Result<Vec<RefundableSwap>, SdkError> {
        self.sdk.list_refundables().await
    }

    pub async fn prepare_refund(
        &self,
        req: PrepareRefundRequest,
    ) -> Result<PrepareRefundResponse, SdkError> {
        self.sdk.prepare_refund(&req).await
    }

    pub async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, PaymentError> {
        self.sdk.refund(&req).await
    }

    pub async fn rescan_onchain_swaps(&self) -> Result<(), SdkError> {
        self.sdk.rescan_onchain_swaps().await
    }

    #[frb(name = "sync")]
    pub async fn sync(&self) -> Result<(), SdkError> {
        self.sdk.sync(false).await.map_err(Into::into)
    }

    pub async fn recommended_fees(&self) -> Result<RecommendedFees, SdkError> {
        self.sdk.recommended_fees().await
    }

    #[frb(sync)]
    pub fn empty_wallet_cache(&self) -> Result<(), SdkError> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    #[frb(sync)]
    pub fn backup(&self, req: BackupRequest) -> Result<(), SdkError> {
        self.sdk.backup(req).map_err(Into::into)
    }

    #[frb(sync)]
    pub fn restore(&self, req: RestoreRequest) -> Result<(), SdkError> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub async fn disconnect(&self) -> Result<(), SdkError> {
        self.sdk.disconnect().await
    }
}
