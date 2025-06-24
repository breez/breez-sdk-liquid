#![cfg(feature = "regtest")]

mod bitcoin;
mod bolt11;
mod bolt12;
mod liquid;
mod utils;

use std::{fs, path::PathBuf, time::Duration};

use anyhow::Result;
use breez_sdk_liquid::model::Config;
use breez_sdk_liquid::{
    model::{
        ConnectRequest, EventListener, ListPaymentsRequest, PayOnchainRequest, Payment,
        PreparePayOnchainRequest, PreparePayOnchainResponse, PrepareReceiveRequest,
        PrepareReceiveResponse, PrepareSendRequest, PrepareSendResponse, ReceivePaymentRequest,
        ReceivePaymentResponse, SdkEvent, SendPaymentRequest, SendPaymentResponse,
    },
    prelude::Arc,
    sdk::LiquidSdk,
};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_with_wasm::alias as tokio;

pub const TIMEOUT: Duration = Duration::from_secs(40);

struct ForwardingEventListener {
    sender: Sender<SdkEvent>,
}

impl EventListener for ForwardingEventListener {
    fn on_event(&self, e: SdkEvent) {
        self.sender.try_send(e).unwrap();
    }
}

pub struct SdkNodeHandle {
    pub sdk: Arc<LiquidSdk>,
    receiver: Receiver<SdkEvent>,
}

pub enum ChainBackend {
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    Electrum,
    Esplora,
}

impl SdkNodeHandle {
    pub async fn init_node(backend: ChainBackend) -> Result<Self> {
        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        let _ = console_log::init_with_level(log::Level::Debug);

        let data_dir = PathBuf::from(format!("/tmp/{}", uuid::Uuid::new_v4()));
        if data_dir.exists() {
            fs::remove_dir_all(&data_dir)?;
        }

        let mnemonic = bip39::Mnemonic::generate_in(bip39::Language::English, 12)?;

        let mut config = match backend {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            ChainBackend::Electrum => Config::regtest(),
            ChainBackend::Esplora => Config::regtest_esplora(),
        };
        config.working_dir = data_dir.to_str().unwrap().to_string();

        #[cfg(all(target_family = "wasm", target_os = "unknown"))]
        let sdk = {
            let connect_req = ConnectRequest {
                config: config.clone(),
                mnemonic: Some(mnemonic.to_string()),
                passphrase: None,
                seed: None,
            };
            let signer: Arc<Box<dyn breez_sdk_liquid::model::Signer>> =
                Arc::new(Box::new(LiquidSdk::default_signer(&connect_req)?));
            let mut sdk_builder = breez_sdk_liquid::sdk::LiquidSdkBuilder::new(
                config.clone(),
                sdk_common::prelude::PRODUCTION_BREEZSERVER_URL.to_string(),
                signer.clone(),
            )?;
            let persister =
                std::sync::Arc::new(breez_sdk_liquid::persist::Persister::new_in_memory(
                    &config.working_dir,
                    config.network,
                    config.sync_enabled(),
                    config.asset_metadata.clone(),
                    None,
                )?);

            sdk_builder.persister(persister);

            let sdk = sdk_builder.build().await?;
            sdk.start().await?;
            sdk
        };
        #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
        let sdk = LiquidSdk::connect(ConnectRequest {
            config,
            mnemonic: Some(mnemonic.to_string()),
            passphrase: None,
            seed: None,
        })
        .await?;

        let (sender, receiver) = mpsc::channel(50);
        let listener = ForwardingEventListener { sender };
        sdk.add_event_listener(Box::new(listener)).await?;

        Ok(Self { sdk, receiver })
    }

    pub async fn get_balance_sat(&self) -> Result<u64> {
        Ok(self.sdk.get_info().await?.wallet_info.balance_sat)
    }

    pub async fn get_pending_receive_sat(&self) -> Result<u64> {
        Ok(self.sdk.get_info().await?.wallet_info.pending_receive_sat)
    }

    pub async fn get_pending_send_sat(&self) -> Result<u64> {
        Ok(self.sdk.get_info().await?.wallet_info.pending_send_sat)
    }

    pub async fn get_payments(&self) -> Result<Vec<Payment>> {
        Ok(self
            .sdk
            .list_payments(&ListPaymentsRequest::default())
            .await?)
    }

    pub async fn receive_payment(
        &self,
        prepare_request: &PrepareReceiveRequest,
    ) -> Result<(PrepareReceiveResponse, ReceivePaymentResponse)> {
        let prepare_response = self.sdk.prepare_receive_payment(prepare_request).await?;
        let receive_response = self
            .sdk
            .receive_payment(&ReceivePaymentRequest {
                prepare_response: prepare_response.clone(),
                description: None,
                use_description_hash: None,
                payer_note: None,
            })
            .await?;
        Ok((prepare_response, receive_response))
    }

    pub async fn send_payment(
        &self,
        prepare_request: &PrepareSendRequest,
    ) -> Result<(PrepareSendResponse, SendPaymentResponse)> {
        let prepare_response = self.sdk.prepare_send_payment(prepare_request).await?;
        let send_response = self
            .sdk
            .send_payment(&SendPaymentRequest {
                prepare_response: prepare_response.clone(),
                use_asset_fees: None,
                payer_note: None,
            })
            .await?;
        Ok((prepare_response, send_response))
    }

    pub async fn send_onchain_payment(
        &self,
        prepare_request: &PreparePayOnchainRequest,
        address: String,
    ) -> Result<(PreparePayOnchainResponse, SendPaymentResponse)> {
        let prepare_response = self.sdk.prepare_pay_onchain(prepare_request).await?;
        let send_response = self
            .sdk
            .pay_onchain(&PayOnchainRequest {
                address,
                prepare_response: prepare_response.clone(),
            })
            .await?;
        Ok((prepare_response, send_response))
    }

    pub async fn wait_for_event<F>(&mut self, predicate: F, timeout: Duration) -> Result<SdkEvent>
    where
        F: Fn(&SdkEvent) -> bool,
    {
        tokio::time::timeout(timeout, async {
            while let Some(event) = self.receiver.recv().await {
                if predicate(&event) {
                    return Ok(event);
                }
            }
            Err(anyhow::anyhow!("Channel closed while waiting for event"))
        })
        .await?
    }
}
