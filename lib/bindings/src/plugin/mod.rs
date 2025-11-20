mod nwc;

use std::sync::Arc;

use breez_sdk_liquid::{error::*, prelude::*};

pub use breez_sdk_liquid::plugin::{PluginStorage, PluginStorageError};

pub use nwc::*;

use crate::{rt, EventListener, EventListenerWrapper};

pub trait PluginSdk: Send + Sync {
    fn get_inner(&self) -> Arc<dyn breez_sdk_liquid::plugin::PluginSdk>;
    fn get_info(&self) -> Result<GetInfoResponse, SdkError>;
    fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError>;
    fn send_payment(&self, req: SendPaymentRequest) -> Result<SendPaymentResponse, PaymentError>;
    fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError>;
    fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError>;
    fn parse(&self, input: String) -> Result<InputType, PaymentError>;
    fn list_payments(&self, req: ListPaymentsRequest) -> Result<Vec<Payment>, PaymentError>;
    fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String>;
    fn remove_event_listener(&self, id: String) -> SdkResult<()>;
}

pub(crate) struct BindingPluginSdk {
    plugin_sdk: Arc<dyn breez_sdk_liquid::plugin::PluginSdk>,
}

impl PluginSdk for BindingPluginSdk {
    fn get_inner(&self) -> Arc<dyn breez_sdk_liquid::plugin::PluginSdk> {
        self.plugin_sdk.clone()
    }

    fn get_info(&self) -> Result<GetInfoResponse, SdkError> {
        rt().block_on(self.plugin_sdk.get_info())
    }

    fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        rt().block_on(self.plugin_sdk.prepare_send_payment(&req))
    }

    fn send_payment(&self, req: SendPaymentRequest) -> Result<SendPaymentResponse, PaymentError> {
        rt().block_on(self.plugin_sdk.send_payment(&req))
    }

    fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        rt().block_on(self.plugin_sdk.prepare_receive_payment(&req))
    }

    fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        rt().block_on(self.plugin_sdk.receive_payment(&req))
    }

    fn parse(&self, input: String) -> Result<InputType, PaymentError> {
        rt().block_on(self.plugin_sdk.parse(&input))
    }

    fn list_payments(&self, req: ListPaymentsRequest) -> Result<Vec<Payment>, PaymentError> {
        rt().block_on(self.plugin_sdk.list_payments(&req))
    }

    fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        let listener: Box<dyn breez_sdk_liquid::prelude::EventListener> =
            Box::new(EventListenerWrapper::new(listener));
        rt().block_on(self.plugin_sdk.add_event_listener(listener))
    }

    fn remove_event_listener(&self, id: String) -> SdkResult<()> {
        rt().block_on(self.plugin_sdk.remove_event_listener(id))
    }
}

pub trait Plugin: Send + Sync {
    fn id(&self) -> String;
    fn on_start(&self, plugin_sdk: Arc<dyn PluginSdk>, storage: Arc<dyn PluginStorage>);
    fn on_stop(&self);
}

pub(crate) struct PluginWrapper {
    pub(crate) inner: Arc<dyn Plugin>,
}

#[sdk_macros::async_trait]
impl breez_sdk_liquid::plugin::Plugin for PluginWrapper {
    fn id(&self) -> String {
        self.inner.id()
    }

    async fn on_start(
        &self,
        plugin_sdk: Arc<dyn breez_sdk_liquid::plugin::PluginSdk>,
        plugin_storage: Arc<dyn PluginStorage>,
    ) {
        self.inner
            .on_start(Arc::new(BindingPluginSdk { plugin_sdk }), plugin_storage);
    }

    async fn on_stop(&self) {
        self.inner.on_stop();
    }
}
