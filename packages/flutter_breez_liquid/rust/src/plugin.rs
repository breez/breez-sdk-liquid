use std::sync::Arc;

use crate::{errors::*, events::BreezEventListener, frb_generated::StreamSink, BreezSdkLiquid};
use breez_sdk_liquid::prelude::*;

#[derive(Clone)]
pub struct PluginSdk {
    plugin_sdk: breez_sdk_liquid::plugin::PluginSdk,
}

impl PluginSdk {
    pub async fn get_info(&self) -> Result<GetInfoResponse, SdkError> {
        self.plugin_sdk.get_info().await
    }

    pub async fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.plugin_sdk.prepare_send_payment(&req).await
    }

    pub async fn send_payment(
        &self,
        req: SendPaymentRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.plugin_sdk.send_payment(&req).await
    }

    pub async fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.plugin_sdk.prepare_receive_payment(&req).await
    }

    pub async fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.plugin_sdk.receive_payment(&req).await
    }

    pub async fn list_payments(
        &self,
        req: ListPaymentsRequest,
    ) -> Result<Vec<Payment>, PaymentError> {
        self.plugin_sdk.list_payments(&req).await
    }

    pub async fn add_event_listener(&self, listener: StreamSink<SdkEvent>) -> SdkResult<String> {
        self.plugin_sdk
            .add_event_listener(Box::new(BreezEventListener { stream: listener }))
            .await
    }
}

pub struct PluginStorage {
    storage: breez_sdk_liquid::plugin::PluginStorage,
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
    fn on_start(&self, plugin_sdk: PluginSdk, storage: PluginStorage);
    fn on_stop(&self);
}

pub(crate) struct PluginWrapper {
    pub(crate) plugin: Arc<dyn Plugin>,
}

#[async_trait::async_trait]
impl breez_sdk_liquid::plugin::Plugin for PluginWrapper {
    fn id(&self) -> String {
        self.plugin.id()
    }

    async fn on_start(
        &self,
        plugin_sdk: breez_sdk_liquid::plugin::PluginSdk,
        storage: breez_sdk_liquid::plugin::PluginStorage,
    ) {
        self.plugin
            .on_start(PluginSdk { plugin_sdk }, PluginStorage { storage })
    }

    async fn on_stop(&self) {
        self.plugin.on_stop()
    }
}
