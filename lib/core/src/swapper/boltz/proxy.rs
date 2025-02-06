use std::sync::{Arc, OnceLock};

use anyhow::Result;
use async_trait::async_trait;
use sdk_common::prelude::BreezServer;

use crate::{persist::Persister, swapper::ProxyUrlFetcher};

pub(crate) struct BoltzProxyFetcher {
    url: OnceLock<Option<String>>,
    persister: Arc<Persister>,
}

impl BoltzProxyFetcher {
    pub(crate) fn new(persister: Arc<Persister>) -> Self {
        Self {
            url: OnceLock::new(),
            persister,
        }
    }
}

#[async_trait]
impl ProxyUrlFetcher for BoltzProxyFetcher {
    async fn fetch(&self) -> Result<&Option<String>> {
        if let Some(swapper_proxy_url) = self.url.get() {
            return Ok(swapper_proxy_url);
        }

        let maybe_swapper_proxy_url =
            match BreezServer::new("https://bs1.breez.technology:443".into(), None) {
                Ok(breez_server) => {
                    let maybe_swapper_proxy_url = breez_server
                        .fetch_boltz_swapper_urls()
                        .await
                        .map(|swapper_urls| swapper_urls.first().cloned())?;

                    if let Some(swapper_proxy_url) = maybe_swapper_proxy_url.clone() {
                        self.persister.set_swapper_proxy_url(swapper_proxy_url)?;
                    }
                    maybe_swapper_proxy_url
                }
                Err(_) => self.persister.get_swapper_proxy_url().unwrap_or(None),
            };

        let swapper_proxy_url = self.url.get_or_init(|| maybe_swapper_proxy_url);
        Ok(swapper_proxy_url)
    }
}
