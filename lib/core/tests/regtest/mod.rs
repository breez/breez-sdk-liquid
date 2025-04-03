#![cfg(feature = "regtest")]

mod bitcoin;
mod bolt11;
mod liquid;
mod utils;

use std::{fs, path::PathBuf, sync::Arc, time::Duration};

use anyhow::Result;
use breez_sdk_liquid::{
    model::{
        ConnectRequest, EventListener, LiquidNetwork, ListPaymentsRequest, PayOnchainRequest,
        Payment, PreparePayOnchainRequest, PreparePayOnchainResponse, PrepareReceiveRequest,
        PrepareReceiveResponse, PrepareSendRequest, PrepareSendResponse, ReceivePaymentRequest,
        ReceivePaymentResponse, SdkEvent, SendPaymentRequest, SendPaymentResponse,
    },
    sdk::LiquidSdk,
};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub const TIMEOUT: Duration = Duration::from_secs(15);

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

impl SdkNodeHandle {
    pub async fn init_node() -> Result<Self> {
        let data_dir = PathBuf::from(format!("/tmp/{}", uuid::Uuid::new_v4()));
        if data_dir.exists() {
            fs::remove_dir_all(&data_dir)?;
        }

        let mnemonic = bip39::Mnemonic::generate_in(bip39::Language::English, 12)?;

        let mut config = LiquidSdk::default_config(LiquidNetwork::Regtest, None)?;
        config.working_dir = data_dir.to_str().unwrap().to_string();

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
