use std::sync::Arc;

use crate::{
    errors::*, events::BreezEventListener, frb_generated::StreamSink, nwc::BreezNwcService,
};
use breez_sdk_liquid::prelude::*;
use breez_sdk_liquid_nwc::model::NwcConfig;
use flutter_rust_bridge::frb;

pub use breez_sdk_liquid::plugin::{
    Plugin as _Plugin, PluginSdk as _PluginSdk, PluginStorage as _PluginStorage,
};

#[derive(Clone)]
pub struct PluginSdk {
    pub(crate) plugin_sdk: _PluginSdk,
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

    pub async fn parse(&self, input: String) -> Result<InputType, PaymentError> {
        self.plugin_sdk.parse(&input).await
    }

    pub async fn list_payments(
        &self,
        req: ListPaymentsRequest,
    ) -> Result<Vec<Payment>, PaymentError> {
        self.plugin_sdk.list_payments(&req).await
    }

    pub async fn add_event_listener(
        &self,
        listener: StreamSink<SdkEvent>,
    ) -> Result<String, SdkError> {
        self.plugin_sdk
            .add_event_listener(Box::new(BreezEventListener { stream: listener }))
            .await
    }
}

pub struct PluginStorage {
    pub(crate) storage: _PluginStorage,
}

impl PluginStorage {
    #[frb(sync)]
    pub fn set_item(&self, key: String, value: String) -> Result<(), PluginStorageError> {
        self.storage.set_item(&key, value)
    }

    #[frb(sync)]
    pub fn get_item(&self, key: String) -> Result<Option<String>, PluginStorageError> {
        self.storage.get_item(&key)
    }

    #[frb(sync)]
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
    pub(crate) plugin: Box<dyn Plugin>,
}

#[async_trait::async_trait]
impl _Plugin for PluginWrapper {
    fn id(&self) -> String {
        self.plugin.id()
    }

    async fn on_start(&self, plugin_sdk: _PluginSdk, storage: _PluginStorage) {
        self.plugin
            .on_start(PluginSdk { plugin_sdk }, PluginStorage { storage });
    }

    async fn on_stop(&self) {
        self.plugin.on_stop();
    }
}

pub struct PluginConfigs {
    pub nwc: Option<NwcConfig>,
}

#[derive(Clone)]
pub struct PluginServices {
    pub nwc: Option<BreezNwcService>,
}

impl Into<PluginServices> for PluginConfigs {
    fn into(self) -> PluginServices {
        let nwc = self.nwc.map(|config| BreezNwcService::new(config));
        PluginServices { nwc }
    }
}

impl PluginServices {
    pub(crate) fn as_plugins(&self) -> Vec<Arc<dyn _Plugin>> {
        let mut plugins = vec![];
        if let Some(nwc_service) = self.nwc.clone() {
            let plugin = Box::new(nwc_service) as Box<dyn Plugin>;
            plugins.push(Arc::new(PluginWrapper { plugin }) as Arc<dyn _Plugin>);
        }
        plugins
    }
}
