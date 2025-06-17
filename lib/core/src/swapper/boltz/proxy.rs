use std::sync::OnceLock;

use crate::PRODUCTION_BREEZSERVER_URL;
use anyhow::Result;
use log::warn;
use sdk_common::prelude::{BoltzSwapperUrls, BreezServer};
use url::Url;

use crate::{persist::Persister, swapper::ProxyUrlFetcher};

pub(crate) struct BoltzProxyFetcher {
    url: OnceLock<Option<BoltzSwapperUrls>>,
    persister: std::sync::Arc<Persister>,
}

pub(crate) fn split_boltz_url(url: &str) -> (Option<String>, Option<String>) {
    Url::parse(url)
        .map(|url| {
            let api_base_url = url.domain().map(|domain| format!("https://{domain}/v2"));
            let referral_id = url.query().and_then(|q| {
                q.split('=')
                    .map(Into::into)
                    .collect::<Vec<String>>()
                    .get(1)
                    .cloned()
            });
            (api_base_url, referral_id)
        })
        .unwrap_or_default()
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
            Ok(breez_server) => {
                let maybe_boltz_swapper_urls = match breez_server.fetch_boltz_swapper_urls().await {
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
                };

                maybe_boltz_swapper_urls
            }
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
