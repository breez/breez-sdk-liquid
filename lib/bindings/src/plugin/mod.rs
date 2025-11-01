mod nwc;

use std::sync::Arc;

use breez_sdk_liquid::{error::*, prelude::*};

pub use breez_sdk_liquid::plugin::PluginStorageError;

pub use nwc::*;

use crate::{rt, EventListener, EventListenerWrapper};

pub struct PluginSdk {
    plugin_sdk: breez_sdk_liquid::plugin::PluginSdk,
}

impl PluginSdk {
    pub fn get_info(&self) -> Result<GetInfoResponse, SdkError> {
        rt().block_on(self.plugin_sdk.get_info())
    }

    pub fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        rt().block_on(self.plugin_sdk.prepare_send_payment(&req))
    }

    pub fn send_payment(
        &self,
        req: SendPaymentRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        rt().block_on(self.plugin_sdk.send_payment(&req))
    }

    pub fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        rt().block_on(self.plugin_sdk.prepare_receive_payment(&req))
    }

    pub fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        rt().block_on(self.plugin_sdk.receive_payment(&req))
    }

    pub fn list_payments(&self, req: ListPaymentsRequest) -> Result<Vec<Payment>, PaymentError> {
        rt().block_on(self.plugin_sdk.list_payments(&req))
    }

    pub fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        let listener: Box<dyn breez_sdk_liquid::prelude::EventListener> =
            Box::new(EventListenerWrapper::new(listener));
        rt().block_on(self.plugin_sdk.add_event_listener(listener))
    }

    pub fn remove_event_listener(&self, id: String) -> SdkResult<()> {
        rt().block_on(self.plugin_sdk.remove_event_listener(id))
    }
}

pub struct PluginStorage {
    pub(crate) storage: breez_sdk_liquid::plugin::PluginStorage,
}

impl PluginStorage {
    pub fn set_item(&self, key: String, value: String) -> Result<(), PluginStorageError> {
        self.storage.set_item(&key, value)
    }

    pub fn get_item(&self, key: String) -> Result<Option<String>, PluginStorageError> {
        self.storage.get_item(&key)
    }

    pub fn remove_item(&self, key: String) -> Result<(), PluginStorageError> {
        self.storage.remove_item(&key)
    }
}

pub trait Plugin: Send + Sync {
    fn id(&self) -> String;
    fn on_start(&self, plugin_sdk: Arc<PluginSdk>, storage: Arc<PluginStorage>);
    fn on_stop(&self);
}

pub(crate) struct PluginWrapper {
    pub(crate) inner: Box<dyn Plugin>,
}

#[sdk_macros::async_trait]
impl breez_sdk_liquid::plugin::Plugin for PluginWrapper {
    fn id(&self) -> String {
        self.inner.id()
    }

    async fn on_start(
        &self,
        plugin_sdk: breez_sdk_liquid::plugin::PluginSdk,
        storage: breez_sdk_liquid::plugin::PluginStorage,
    ) {
        self.inner.on_start(
            PluginSdk { plugin_sdk }.into(),
            PluginStorage { storage }.into(),
        );
    }

    async fn on_stop(&self) {
        self.inner.on_stop();
    }
}
