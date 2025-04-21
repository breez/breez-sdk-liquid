use crate::swapper::boltz::CONNECTION_TIMEOUT;
use crate::{
    bitcoin, elements,
    model::{BlockchainExplorer, Config, BREEZ_LIQUID_ESPLORA_URL},
};
use boltz_client::{
    error::Error,
    network::{
        esplora::{EsploraBitcoinClient, EsploraLiquidClient},
        BitcoinChain, BitcoinClient as BoltzBitcoinClient, LiquidChain,
        LiquidClient as BoltzLiquidClient,
    },
    reqwest,
};
use log::error;
use sdk_macros::async_trait;

pub(crate) enum LiquidClient {
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    Electrum(Box<boltz_client::network::electrum::ElectrumLiquidClient>),
    Esplora(Box<EsploraLiquidClient>),
}

impl LiquidClient {
    pub(crate) fn new(config: &Config) -> Result<Self, Error> {
        Ok(match &config.liquid_explorer {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            BlockchainExplorer::Electrum { url } => {
                let (tls, validate_domain) = config.electrum_tls_options();
                Self::Electrum(Box::new(
                    boltz_client::network::electrum::ElectrumLiquidClient::new(
                        config.network.into(),
                        url,
                        tls,
                        validate_domain,
                        CONNECTION_TIMEOUT.as_secs() as u8,
                    )?,
                ))
            }
            BlockchainExplorer::Esplora { url, .. } => {
                let mut builder = reqwest::Client::builder();
                if url == BREEZ_LIQUID_ESPLORA_URL {
                    match &config.breez_api_key {
                        Some(api_key) => {
                            let mut headers = reqwest::header::HeaderMap::new();
                            let api_key = format!("Bearer {api_key}").parse().map_err(|err| {
                                Error::Generic(format!("Could not set api key header: {err}"))
                            })?;
                            headers.insert(reqwest::header::AUTHORIZATION, api_key);
                            builder = builder.default_headers(headers)
                        }
                        None => {
                            let err = "Cannot start Breez Esplora client: Breez API key is not set";
                            error!("{err}");
                            return Err(Error::Generic(err.to_string()));
                        }
                    };
                }
                let client = builder.build().map_err(|err| {
                    Error::Generic(format!("Could not initialize HTTP client: {err}"))
                })?;

                Self::Esplora(Box::new(EsploraLiquidClient::with_client(
                    client,
                    config.network.into(),
                    url,
                    CONNECTION_TIMEOUT.as_secs(),
                )))
            }
        })
    }
}

#[async_trait]
impl BoltzLiquidClient for LiquidClient {
    async fn get_address_utxo(
        &self,
        address: &elements::Address,
    ) -> Result<Option<(elements::OutPoint, elements::TxOut)>, Error> {
        match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            Self::Electrum(c) => c.get_address_utxo(address).await,
            Self::Esplora(c) => c.get_address_utxo(address).await,
        }
    }

    async fn get_genesis_hash(&self) -> Result<elements::BlockHash, Error> {
        match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            Self::Electrum(c) => c.get_genesis_hash().await,
            Self::Esplora(c) => c.get_genesis_hash().await,
        }
    }

    async fn broadcast_tx(&self, signed_tx: &elements::Transaction) -> Result<String, Error> {
        match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            Self::Electrum(c) => c.broadcast_tx(signed_tx).await,
            Self::Esplora(c) => c.broadcast_tx(signed_tx).await,
        }
    }

    fn network(&self) -> LiquidChain {
        match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            Self::Electrum(c) => c.network(),
            Self::Esplora(c) => c.network(),
        }
    }
}

pub(crate) enum BitcoinClient {
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    Electrum(Box<boltz_client::network::electrum::ElectrumBitcoinClient>),
    Esplora(Box<EsploraBitcoinClient>),
}

impl BitcoinClient {
    pub(crate) fn new(config: &Config) -> Result<Self, Error> {
        Ok(match &config.bitcoin_explorer {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            BlockchainExplorer::Electrum { url } => {
                let (tls, validate_domain) = config.electrum_tls_options();
                Self::Electrum(Box::new(
                    boltz_client::network::electrum::ElectrumBitcoinClient::new(
                        config.network.as_bitcoin_chain(),
                        url,
                        tls,
                        validate_domain,
                        CONNECTION_TIMEOUT.as_secs() as u8,
                    )?,
                ))
            }
            BlockchainExplorer::Esplora { url, .. } => {
                Self::Esplora(Box::new(EsploraBitcoinClient::new(
                    config.network.as_bitcoin_chain(),
                    url,
                    CONNECTION_TIMEOUT.as_secs(),
                )))
            }
        })
    }
}

#[async_trait]
impl BoltzBitcoinClient for BitcoinClient {
    async fn get_address_balance(&self, address: &bitcoin::Address) -> Result<(u64, i64), Error> {
        match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            Self::Electrum(c) => c.get_address_balance(address).await,
            Self::Esplora(c) => c.get_address_balance(address).await,
        }
    }

    async fn get_address_utxos(
        &self,
        address: &bitcoin::Address,
    ) -> Result<Vec<(bitcoin::OutPoint, bitcoin::TxOut)>, Error> {
        match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            Self::Electrum(c) => c.get_address_utxos(address).await,
            Self::Esplora(c) => c.get_address_utxos(address).await,
        }
    }

    async fn broadcast_tx(&self, signed_tx: &bitcoin::Transaction) -> Result<bitcoin::Txid, Error> {
        match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            Self::Electrum(c) => c.broadcast_tx(signed_tx).await,
            Self::Esplora(c) => c.broadcast_tx(signed_tx).await,
        }
    }

    fn network(&self) -> BitcoinChain {
        match self {
            #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
            Self::Electrum(c) => c.network(),
            Self::Esplora(c) => c.network(),
        }
    }
}
