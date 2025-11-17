use std::sync::Arc;

mod plugin;

use anyhow::Result;
pub use breez_sdk_liquid::NostrWalletConnectUri as NostrConnectionUri;
use breez_sdk_liquid::{error::*, logger::Logger, model::*, prelude::*};
use log::{Metadata, Record, SetLoggerError};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use uniffi::deps::log::{Level, LevelFilter};

pub use plugin::*;

static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

pub(crate) fn rt() -> &'static Runtime {
    &RT
}

struct UniffiBindingLogger {
    logger: Box<dyn Logger>,
}

impl UniffiBindingLogger {
    fn init(logger: Box<dyn Logger>) -> Result<(), SetLoggerError> {
        let binding_logger: UniffiBindingLogger = UniffiBindingLogger { logger };
        log::set_boxed_logger(Box::new(binding_logger))
            .map(|_| log::set_max_level(LevelFilter::Trace))
    }
}

impl log::Log for UniffiBindingLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        // ignore the internal uniffi log to prevent infinite loop.
        m.level() <= Level::Trace && *m.target() != *"breez_sdk_liquid_bindings"
    }

    fn log(&self, record: &Record) {
        self.logger.log(LogEntry {
            line: record.args().to_string(),
            level: record.level().as_str().to_string(),
        });
    }
    fn flush(&self) {}
}

pub trait EventListener: Send + Sync {
    fn on_event(&self, e: SdkEvent);
}

struct EventListenerWrapper {
    inner: Box<dyn EventListener>,
}

impl EventListenerWrapper {
    pub(crate) fn new(inner: Box<dyn EventListener>) -> Self {
        Self { inner }
    }
}

#[sdk_macros::async_trait]
impl breez_sdk_liquid::prelude::EventListener for EventListenerWrapper {
    async fn on_event(&self, e: SdkEvent) {
        self.inner.on_event(e);
    }
}

/// If used, this must be called before `connect`
pub fn set_logger(logger: Box<dyn Logger>) -> Result<(), SdkError> {
    UniffiBindingLogger::init(logger).map_err(|_| SdkError::generic("Logger already created"))
}

pub fn connect(
    req: ConnectRequest,
    plugins: Option<Vec<Box<dyn Plugin>>>,
) -> Result<Arc<BindingLiquidSdk>, SdkError> {
    rt().block_on(async {
        let plugins = plugins.map(|plugins| {
            plugins
                .into_iter()
                .map(|p| {
                    Arc::new(PluginWrapper { inner: p })
                        as Arc<dyn breez_sdk_liquid::plugin::Plugin>
                })
                .collect()
        });
        let sdk = LiquidSdk::connect(req, plugins).await?;
        Ok(Arc::from(BindingLiquidSdk { sdk }))
    })
}

pub fn connect_with_signer(
    req: ConnectWithSignerRequest,
    signer: Box<dyn Signer>,
    plugins: Option<Vec<Box<dyn Plugin>>>,
) -> Result<Arc<BindingLiquidSdk>, SdkError> {
    rt().block_on(async {
        let plugins = plugins.map(|plugins| {
            plugins
                .into_iter()
                .map(|p| {
                    Arc::new(PluginWrapper { inner: p })
                        as Arc<dyn breez_sdk_liquid::plugin::Plugin>
                })
                .collect()
        });
        let sdk = LiquidSdk::connect_with_signer(req, signer, plugins).await?;
        Ok(Arc::from(BindingLiquidSdk { sdk }))
    })
}

pub fn default_config(
    network: LiquidNetwork,
    breez_api_key: Option<String>,
) -> Result<Config, SdkError> {
    LiquidSdk::default_config(network, breez_api_key)
}

pub fn parse_invoice(input: String) -> Result<LNInvoice, PaymentError> {
    LiquidSdk::parse_invoice(&input)
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        let listener: Box<dyn breez_sdk_liquid::prelude::EventListener> =
            Box::new(EventListenerWrapper::new(listener));
        rt().block_on(self.sdk.add_event_listener(listener))
    }

    pub fn remove_event_listener(&self, id: String) -> SdkResult<()> {
        rt().block_on(self.sdk.remove_event_listener(id))
    }

    pub fn get_info(&self) -> Result<GetInfoResponse, SdkError> {
        rt().block_on(self.sdk.get_info())
    }

    pub fn sign_message(&self, req: SignMessageRequest) -> SdkResult<SignMessageResponse> {
        self.sdk.sign_message(&req)
    }

    pub fn check_message(&self, req: CheckMessageRequest) -> SdkResult<CheckMessageResponse> {
        self.sdk.check_message(&req)
    }

    pub fn parse(&self, input: String) -> Result<InputType, PaymentError> {
        rt().block_on(async { self.sdk.parse(&input).await })
    }

    pub fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_send_payment(&req))
    }

    pub fn send_payment(
        &self,
        req: SendPaymentRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        rt().block_on(self.sdk.send_payment(&req))
    }

    pub fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_receive_payment(&req))
    }

    pub fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        rt().block_on(self.sdk.receive_payment(&req))
    }

    pub fn create_bolt12_invoice(
        &self,
        req: CreateBolt12InvoiceRequest,
    ) -> Result<CreateBolt12InvoiceResponse, PaymentError> {
        rt().block_on(self.sdk.create_bolt12_invoice(&req))
    }

    pub fn fetch_lightning_limits(&self) -> Result<LightningPaymentLimitsResponse, PaymentError> {
        rt().block_on(self.sdk.fetch_lightning_limits())
    }

    pub fn fetch_onchain_limits(&self) -> Result<OnchainPaymentLimitsResponse, PaymentError> {
        rt().block_on(self.sdk.fetch_onchain_limits())
    }

    pub fn prepare_pay_onchain(
        &self,
        req: PreparePayOnchainRequest,
    ) -> Result<PreparePayOnchainResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_pay_onchain(&req))
    }

    pub fn pay_onchain(&self, req: PayOnchainRequest) -> Result<SendPaymentResponse, PaymentError> {
        rt().block_on(self.sdk.pay_onchain(&req))
    }

    pub fn prepare_buy_bitcoin(
        &self,
        req: PrepareBuyBitcoinRequest,
    ) -> Result<PrepareBuyBitcoinResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_buy_bitcoin(&req))
    }

    pub fn buy_bitcoin(&self, req: BuyBitcoinRequest) -> Result<String, PaymentError> {
        rt().block_on(self.sdk.buy_bitcoin(&req))
    }

    pub fn list_payments(&self, req: ListPaymentsRequest) -> Result<Vec<Payment>, PaymentError> {
        rt().block_on(self.sdk.list_payments(&req))
    }

    pub fn get_payment(&self, req: GetPaymentRequest) -> Result<Option<Payment>, PaymentError> {
        rt().block_on(self.sdk.get_payment(&req))
    }

    pub fn fetch_payment_proposed_fees(
        &self,
        req: FetchPaymentProposedFeesRequest,
    ) -> SdkResult<FetchPaymentProposedFeesResponse> {
        rt().block_on(self.sdk.fetch_payment_proposed_fees(&req))
    }

    pub fn accept_payment_proposed_fees(
        &self,
        req: AcceptPaymentProposedFeesRequest,
    ) -> Result<(), PaymentError> {
        rt().block_on(self.sdk.accept_payment_proposed_fees(&req))
    }

    pub fn prepare_lnurl_pay(
        &self,
        req: PrepareLnUrlPayRequest,
    ) -> Result<PrepareLnUrlPayResponse, LnUrlPayError> {
        rt().block_on(self.sdk.prepare_lnurl_pay(req))
    }

    pub fn lnurl_pay(&self, req: model::LnUrlPayRequest) -> Result<LnUrlPayResult, LnUrlPayError> {
        rt().block_on(self.sdk.lnurl_pay(req))
    }

    pub fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<LnUrlWithdrawResult, LnUrlWithdrawError> {
        rt().block_on(self.sdk.lnurl_withdraw(req))
    }

    pub fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<LnUrlCallbackStatus, LnUrlAuthError> {
        rt().block_on(self.sdk.lnurl_auth(req_data))
    }

    pub fn register_webhook(&self, webhook_url: String) -> Result<(), SdkError> {
        rt().block_on(self.sdk.register_webhook(webhook_url))
    }

    pub fn unregister_webhook(&self) -> Result<(), SdkError> {
        rt().block_on(self.sdk.unregister_webhook())
    }

    pub fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, SdkError> {
        rt().block_on(self.sdk.fetch_fiat_rates())
    }

    pub fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, SdkError> {
        rt().block_on(self.sdk.list_fiat_currencies())
    }

    pub fn list_refundables(&self) -> SdkResult<Vec<RefundableSwap>> {
        rt().block_on(self.sdk.list_refundables())
    }

    pub fn prepare_refund(&self, req: PrepareRefundRequest) -> SdkResult<PrepareRefundResponse> {
        rt().block_on(self.sdk.prepare_refund(&req))
    }

    pub fn refund(&self, req: RefundRequest) -> Result<RefundResponse, PaymentError> {
        rt().block_on(self.sdk.refund(&req))
    }

    pub fn rescan_onchain_swaps(&self) -> SdkResult<()> {
        rt().block_on(self.sdk.rescan_onchain_swaps())
    }

    pub fn sync(&self) -> SdkResult<()> {
        rt().block_on(self.sdk.sync(false))
    }

    pub fn recommended_fees(&self) -> SdkResult<RecommendedFees> {
        rt().block_on(self.sdk.recommended_fees())
    }

    pub fn empty_wallet_cache(&self) -> SdkResult<()> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    pub fn backup(&self, req: BackupRequest) -> SdkResult<()> {
        self.sdk.backup(req).map_err(Into::into)
    }

    pub fn restore(&self, req: RestoreRequest) -> SdkResult<()> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub fn disconnect(&self) -> SdkResult<()> {
        rt().block_on(self.sdk.disconnect())
    }
}

uniffi::include_scaffolding!("breez_sdk_liquid");
