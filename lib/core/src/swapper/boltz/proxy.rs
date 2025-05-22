use std::sync::OnceLock;

use crate::PRODUCTION_BREEZSERVER_URL;
use anyhow::Result;
use sdk_common::prelude::BreezServer;
use url::Url;

use crate::{persist::Persister, swapper::ProxyUrlFetcher};

pub(crate) struct BoltzProxyFetcher {
    url: OnceLock<Option<String>>,
    persister: std::sync::Arc<Persister>,
}

pub(crate) fn split_proxy_url(url: &str) -> (Option<String>, Option<String>) {
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
    async fn fetch(&self) -> Result<&Option<String>> {
        if let Some(swapper_proxy_url) = self.url.get() {
            return Ok(swapper_proxy_url);
        }

        let maybe_swapper_proxy_url =
            match BreezServer::new(PRODUCTION_BREEZSERVER_URL.into(), None) {
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
