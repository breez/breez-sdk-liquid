use std::sync::OnceLock;

use crate::PRODUCTION_BREEZSERVER_URL;
use anyhow::Result;
use log::warn;
use sdk_common::prelude::{BoltzSwapperUrls, BreezServer};

use crate::{persist::Persister, swapper::ProxyUrlFetcher};

#[allow(dead_code)]
pub(crate) struct BoltzProxyFetcher {
    url: OnceLock<Option<BoltzSwapperUrls>>,
    persister: std::sync::Arc<Persister>,
}

impl BoltzProxyFetcher {
    pub(crate) fn new(persister: std::sync::Arc<Persister>) -> Self {
        Self {
            url: OnceLock::new(),
            persister,
        }
    }
}

#[sdk_macros::async_trait]
impl ProxyUrlFetcher for BoltzProxyFetcher {
    async fn fetch(&self) -> Result<&Option<BoltzSwapperUrls>> {
        if let Some(boltz_swapper_urls) = self.url.get() {
            return Ok(boltz_swapper_urls);
        }

        let maybe_boltz_swapper_urls = match BreezServer::new(
            PRODUCTION_BREEZSERVER_URL.into(),
            None,
        ) {
            Ok(breez_server) => match breez_server.fetch_boltz_swapper_urls().await {
                Ok(boltz_swapper_urls) => {
                    self.persister
                        .set_swapper_proxy_url(serde_json::to_string(&boltz_swapper_urls)?)?;
                    Some(boltz_swapper_urls)
                }
                Err(e) => {
                    warn!("Failed to fetch boltz swapper url: {e}. Trying to use urls cached in db...");
                    self.persister
                        .get_swapper_proxy_url()
                        .unwrap_or(None)
                        .and_then(|s| serde_json::from_str(&s).ok())
                }
            },
            Err(e) => {
                warn!("Failed to create BreezServer: {e}. Trying to use urls cached in db...");
                self.persister
                    .get_swapper_proxy_url()
                    .unwrap_or(None)
                    .and_then(|s| serde_json::from_str(&s).ok())
            }
        };

        let boltz_swapper_urls = self.url.get_or_init(|| maybe_boltz_swapper_urls);
        Ok(boltz_swapper_urls)
    }
}
