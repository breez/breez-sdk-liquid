mod error;
mod event;
pub mod model;
mod signer;

use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use breez_sdk_liquid::sdk::LiquidSdk;
use log::Level;
use signer::{Signer, WasmSigner};
use wasm_bindgen::prelude::*;

use crate::event::{EventListener, WasmEventListener};
use crate::model::*;

#[wasm_bindgen]
pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

#[wasm_bindgen(js_name = "connect")]
pub async fn connect(req: ConnectRequest) -> WasmResult<BindingLiquidSdk> {
    let sdk = LiquidSdk::connect(req.into()).await?;
    Ok(BindingLiquidSdk { sdk })
}

#[wasm_bindgen(js_name = "connectWithSigner")]
pub async fn connect_with_signer(
    req: ConnectWithSignerRequest,
    signer: Signer,
) -> WasmResult<BindingLiquidSdk> {
    let wasm_signer = Box::new(WasmSigner { signer });
    let sdk = LiquidSdk::connect_with_signer(req.into(), wasm_signer).await?;
    Ok(BindingLiquidSdk { sdk })
}

#[wasm_bindgen(js_name = "defaultConfig")]
pub fn default_config(network: LiquidNetwork, breez_api_key: Option<String>) -> WasmResult<Config> {
    Ok(LiquidSdk::default_config(network.into(), breez_api_key)?.into())
}

#[wasm_bindgen(js_name = "parseInvoice")]
pub fn parse_invoice(input: String) -> WasmResult<LNInvoice> {
    Ok(LiquidSdk::parse_invoice(&input)?.into())
}

#[wasm_bindgen(js_name = "initLogger")]
pub fn init_logger(level: String) -> WasmResult<()> {
    Ok(console_log::init_with_level(Level::from_str(&level)?)
        .map_err(|_| anyhow!("Logger already created"))?)
}

#[wasm_bindgen]
impl BindingLiquidSdk {
    #[wasm_bindgen(js_name = "getInfo")]
    pub async fn get_info(&self) -> WasmResult<GetInfoResponse> {
        Ok(self.sdk.get_info().await?.into())
    }

    #[wasm_bindgen(js_name = "signMessage")]
    pub fn sign_message(&self, req: SignMessageRequest) -> WasmResult<SignMessageResponse> {
        Ok(self.sdk.sign_message(&req.into())?.into())
    }

    #[wasm_bindgen(js_name = "checkMessage")]
    pub fn check_message(&self, req: CheckMessageRequest) -> WasmResult<CheckMessageResponse> {
        Ok(self.sdk.check_message(&req.into())?.into())
    }

    #[wasm_bindgen(js_name = "parse")]
    pub async fn parse(&self, input: String) -> WasmResult<InputType> {
        Ok(self.sdk.parse(&input).await?.into())
    }

    #[wasm_bindgen(js_name = "addEventListener")]
    pub async fn add_event_listener(&self, listener: EventListener) -> WasmResult<String> {
        Ok(self
            .sdk
            .add_event_listener(Box::new(WasmEventListener { listener }))
            .await?)
    }

    #[wasm_bindgen(js_name = "removeEventListener")]
    pub async fn remove_event_listener(&self, id: String) -> WasmResult<()> {
        self.sdk.remove_event_listener(id).await?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "prepareSendPayment")]
    pub async fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> WasmResult<PrepareSendResponse> {
        Ok(self.sdk.prepare_send_payment(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "sendPayment")]
    pub async fn send_payment(&self, req: SendPaymentRequest) -> WasmResult<SendPaymentResponse> {
        Ok(self.sdk.send_payment(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "preparePeceivePayment")]
    pub async fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> WasmResult<PrepareReceiveResponse> {
        Ok(self.sdk.prepare_receive_payment(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "receivePayment")]
    pub async fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> WasmResult<ReceivePaymentResponse> {
        Ok(self.sdk.receive_payment(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "fetchLightningLimits")]
    pub async fn fetch_lightning_limits(&self) -> WasmResult<LightningPaymentLimitsResponse> {
        Ok(self.sdk.fetch_lightning_limits().await?.into())
    }

    #[wasm_bindgen(js_name = "fetchOnchainLimits")]
    pub async fn fetch_onchain_limits(&self) -> WasmResult<OnchainPaymentLimitsResponse> {
        Ok(self.sdk.fetch_onchain_limits().await?.into())
    }

    #[wasm_bindgen(js_name = "preparePayOnchain")]
    pub async fn prepare_pay_onchain(
        &self,
        req: PreparePayOnchainRequest,
    ) -> WasmResult<PreparePayOnchainResponse> {
        Ok(self.sdk.prepare_pay_onchain(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "payOnchain")]
    pub async fn pay_onchain(&self, req: PayOnchainRequest) -> WasmResult<SendPaymentResponse> {
        Ok(self.sdk.pay_onchain(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "prepareBuyBitcoin")]
    pub async fn prepare_buy_bitcoin(
        &self,
        req: PrepareBuyBitcoinRequest,
    ) -> WasmResult<PrepareBuyBitcoinResponse> {
        Ok(self.sdk.prepare_buy_bitcoin(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "buyBitcoin")]
    pub async fn buy_bitcoin(&self, req: BuyBitcoinRequest) -> WasmResult<String> {
        Ok(self.sdk.buy_bitcoin(&req.into()).await?)
    }

    #[wasm_bindgen(js_name = "listPayments")]
    pub async fn list_payments(&self, req: ListPaymentsRequest) -> WasmResult<Vec<Payment>> {
        Ok(self
            .sdk
            .list_payments(&req.into())
            .await?
            .into_iter()
            .map(|r| r.into())
            .collect())
    }

    #[wasm_bindgen(js_name = "getPayment")]
    pub async fn get_payment(&self, req: GetPaymentRequest) -> WasmResult<Option<Payment>> {
        Ok(self.sdk.get_payment(&req.into()).await?.map(|r| r.into()))
    }

    #[wasm_bindgen(js_name = "fetchPaymentProposedFees")]
    pub async fn fetch_payment_proposed_fees(
        &self,
        req: FetchPaymentProposedFeesRequest,
    ) -> WasmResult<FetchPaymentProposedFeesResponse> {
        Ok(self
            .sdk
            .fetch_payment_proposed_fees(&req.into())
            .await?
            .into())
    }

    #[wasm_bindgen(js_name = "acceptPaymentProposedFees")]
    pub async fn accept_payment_proposed_fees(
        &self,
        req: AcceptPaymentProposedFeesRequest,
    ) -> WasmResult<()> {
        self.sdk.accept_payment_proposed_fees(&req.into()).await?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "prepareLnurlPay")]
    pub async fn prepare_lnurl_pay(
        &self,
        req: PrepareLnUrlPayRequest,
    ) -> WasmResult<PrepareLnUrlPayResponse> {
        Ok(self.sdk.prepare_lnurl_pay(req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "lnurlPay")]
    pub async fn lnurl_pay(&self, req: LnUrlPayRequest) -> WasmResult<LnUrlPayResult> {
        Ok(self.sdk.lnurl_pay(req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "lnurlWithdraw")]
    pub async fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> WasmResult<LnUrlWithdrawResult> {
        Ok(self.sdk.lnurl_withdraw(req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "lnurlAuth")]
    pub async fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> WasmResult<LnUrlCallbackStatus> {
        Ok(self.sdk.lnurl_auth(req_data.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "registerWebhook")]
    pub async fn register_webhook(&self, webhook_url: String) -> WasmResult<()> {
        self.sdk.register_webhook(webhook_url).await?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "unregisterWebhook")]
    pub async fn unregister_webhook(&self) -> WasmResult<()> {
        self.sdk.unregister_webhook().await?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "fetchFiatRates")]
    pub async fn fetch_fiat_rates(&self) -> WasmResult<Vec<Rate>> {
        Ok(self
            .sdk
            .fetch_fiat_rates()
            .await?
            .into_iter()
            .map(|r| r.into())
            .collect())
    }

    #[wasm_bindgen(js_name = "listFiatCurrencies")]
    pub async fn list_fiat_currencies(&self) -> WasmResult<Vec<FiatCurrency>> {
        Ok(self
            .sdk
            .list_fiat_currencies()
            .await?
            .into_iter()
            .map(|r| r.into())
            .collect())
    }

    #[wasm_bindgen(js_name = "listRefundables")]
    pub async fn list_refundables(&self) -> WasmResult<Vec<RefundableSwap>> {
        Ok(self
            .sdk
            .list_refundables()
            .await?
            .into_iter()
            .map(|r| r.into())
            .collect())
    }

    #[wasm_bindgen(js_name = "prepareRefund")]
    pub async fn prepare_refund(
        &self,
        req: PrepareRefundRequest,
    ) -> WasmResult<PrepareRefundResponse> {
        Ok(self.sdk.prepare_refund(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "refund")]
    pub async fn refund(&self, req: RefundRequest) -> WasmResult<RefundResponse> {
        Ok(self.sdk.refund(&req.into()).await?.into())
    }

    #[wasm_bindgen(js_name = "rescanOnchainSwaps")]
    pub async fn rescan_onchain_swaps(&self) -> WasmResult<()> {
        self.sdk.rescan_onchain_swaps().await?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "sync")]
    pub async fn sync(&self) -> WasmResult<()> {
        self.sdk.sync(false).await?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "recommendedFees")]
    pub async fn recommended_fees(&self) -> WasmResult<RecommendedFees> {
        Ok(self.sdk.recommended_fees().await?.into())
    }

    #[wasm_bindgen(js_name = "emptyWalletCache")]
    pub fn empty_wallet_cache(&self) -> WasmResult<()> {
        self.sdk.empty_wallet_cache()?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "backup")]
    pub fn backup(&self, req: BackupRequest) -> WasmResult<()> {
        self.sdk.backup(req.into())?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "restore")]
    pub fn restore(&self, req: RestoreRequest) -> WasmResult<()> {
        self.sdk.restore(req.into())?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "disconnect")]
    pub async fn disconnect(&self) -> WasmResult<()> {
        self.sdk.disconnect().await?;
        Ok(())
    }
}
