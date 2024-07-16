use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use moonpay::MoonpayProvider;
use sdk_common::prelude::BreezServer;

use crate::{
    model::{BuyBitcoinProvider, ChainSwap, Config},
    prelude::LiquidNetwork,
};

pub(crate) mod moonpay;

#[async_trait]
pub(crate) trait FiatOnRampProvider: Send + Sync {
    /// Configure buying Bitcoin and return a URL to continue
    async fn buy_bitcoin_onchain(
        &self,
        chain_swap: &ChainSwap,
        redirect_url: Option<String>,
    ) -> Result<String>;
}

#[async_trait]
pub(crate) trait FiatOnRampService: Send + Sync {
    /// Initiate buying Bitcoin and return a URL to the selected third party provider
    async fn buy_bitcoin_onchain(
        &self,
        provider: BuyBitcoinProvider,
        chain_swap: &ChainSwap,
        redirect_url: Option<String>,
    ) -> Result<String>;
}

pub(crate) struct BuyBitcoinService {
    config: Config,
    moonpay_provider: Arc<dyn FiatOnRampProvider>,
}

impl BuyBitcoinService {
    pub fn new(config: Config, breez_server: Arc<BreezServer>) -> Self {
        let moonpay_provider = Arc::new(MoonpayProvider::new(breez_server));
        Self {
            config,
            moonpay_provider,
        }
    }
}

#[async_trait]
impl FiatOnRampService for BuyBitcoinService {
    async fn buy_bitcoin_onchain(
        &self,
        provider: BuyBitcoinProvider,
        chain_swap: &ChainSwap,
        redirect_url: Option<String>,
    ) -> Result<String> {
        if self.config.network != LiquidNetwork::Mainnet {
            return Err(anyhow!("Can only buy Bitcoin on Mainnet"));
        }

        match provider {
            BuyBitcoinProvider::Moonpay => {
                self.moonpay_provider
                    .buy_bitcoin_onchain(chain_swap, redirect_url)
                    .await
            }
        }
    }
}
