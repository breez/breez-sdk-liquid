use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sdk_common::prelude::{BreezServer, BuyBitcoinProviderApi, MoonpayProvider};

use crate::{
    model::{BuyBitcoinProvider, ChainSwap, Config},
    prelude::LiquidNetwork,
};

#[async_trait]
pub(crate) trait BuyBitcoinApi: Send + Sync {
    /// Initiate buying Bitcoin and return a URL to the selected third party provider
    async fn buy_bitcoin(
        &self,
        provider: BuyBitcoinProvider,
        chain_swap: &ChainSwap,
        redirect_url: Option<String>,
    ) -> Result<String>;
}

pub(crate) struct BuyBitcoinService {
    config: Config,
    moonpay_provider: Arc<dyn BuyBitcoinProviderApi>,
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
impl BuyBitcoinApi for BuyBitcoinService {
    async fn buy_bitcoin(
        &self,
        provider: BuyBitcoinProvider,
        chain_swap: &ChainSwap,
        redirect_url: Option<String>,
    ) -> Result<String> {
        if self.config.network != LiquidNetwork::Mainnet {
            return Err(anyhow!("Can only buy bitcoin on Mainnet"));
        }

        let create_response = chain_swap.get_boltz_create_response()?;

        match provider {
            BuyBitcoinProvider::Moonpay => {
                self.moonpay_provider
                    .buy_bitcoin(
                        create_response.lockup_details.lockup_address,
                        Some(create_response.lockup_details.amount as u64),
                        None,
                        redirect_url,
                    )
                    .await
            }
        }
    }
}
