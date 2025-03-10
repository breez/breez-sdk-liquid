use anyhow::{bail, Result};
use electrum_client::ElectrumApi as _;
use lwk_wollet::{
    asyncr::EsploraClient, blocking::BlockchainBackend, ElectrumOptions, ElectrumUrl,
};

use crate::model::{BlockchainExplorer, LiquidNetwork};

#[macro_export]
macro_rules! get_client {
    ($chain_service:ident,$client:ident) => {
        $chain_service.set_client()?;
        let lock = $chain_service
            .client
            .read()
            .map_err(|err| anyhow!("Could not read client lock: {err:?}"))
            .await?;
        let Some($client) = lock.as_ref() else {
            bail!("Client not set"); // unreachable
        };
    };
}

pub(crate) enum BlockchainClient<ElectrumClient, EsploraClient> {
    Electrum { inner: Box<ElectrumClient> },
    Esplora { inner: Box<EsploraClient> },
}

/// The impl below is used for all Liquid-related instances, using [lwk_wollet::ElectrumClient] and
/// [lwk_wollet::blocking::EsploraClient]
impl BlockchainClient<lwk_wollet::ElectrumClient, lwk_wollet::blocking::EsploraClient> {
    pub(crate) fn is_available(&self) -> bool {
        match self {
            BlockchainClient::Electrum { inner } => inner.ping().is_ok(),
            BlockchainClient::Esplora { .. } => true,
        }
    }

    pub(crate) fn try_from_explorer(
        exp: &BlockchainExplorer,
        network: LiquidNetwork,
    ) -> Result<Self> {
        match exp {
            BlockchainExplorer::Electrum { url } => {
                let (tls, validate_domain) = match network {
                    LiquidNetwork::Mainnet | LiquidNetwork::Testnet => (true, true),
                    LiquidNetwork::Regtest => (false, false),
                };
                let url = ElectrumUrl::new(url, tls, validate_domain)?;
                let client = lwk_wollet::ElectrumClient::with_options(
                    &url,
                    ElectrumOptions { timeout: Some(3) },
                )?;
                // Ensure we ping so we know the client is working
                client.ping()?;
                Ok(BlockchainClient::Electrum {
                    inner: Box::new(client),
                })
            }
            BlockchainExplorer::Esplora {
                url,
                use_waterfalls,
            } => {
                let client = match *use_waterfalls {
                    true => lwk_wollet::blocking::EsploraClient::new(url, network.into()),
                    false => {
                        lwk_wollet::blocking::EsploraClient::new_waterfalls(url, network.into())
                    }
                }?;
                Ok(BlockchainClient::Esplora {
                    inner: Box::new(client),
                })
            }
            BlockchainExplorer::MempoolSpace { .. } => {
                bail!("Cannot create client from MempoolSpace url")
            }
        }
    }

    pub(crate) fn update_wallet(
        &mut self,
        wollet: &mut lwk_wollet::Wollet,
        index: u32,
    ) -> Result<(), lwk_wollet::Error> {
        let state = wollet.state();
        let update = match self {
            Self::Electrum { inner, .. } => inner.full_scan_to_index(&state, index)?,
            Self::Esplora { inner, .. } => inner.full_scan_to_index(&state, index)?,
        };
        if let Some(update) = update {
            wollet.apply_update(update)?;
        }
        Ok(())
    }
}

impl<C: BlockchainBackend> BlockchainBackend for BlockchainClient<C> {
    fn tip(&mut self) -> std::result::Result<lwk_wollet::elements::BlockHeader, lwk_wollet::Error> {
        match self {
            BlockchainClient::Electrum { inner } => inner.tip(),
            BlockchainClient::Esplora { inner } => inner.tip(),
        }
    }

    fn broadcast(
        &self,
        tx: &lwk_wollet::elements::Transaction,
    ) -> Result<lwk_wollet::elements::Txid, lwk_wollet::Error> {
        match self {
            BlockchainClient::Electrum { inner } => inner.broadcast(tx),
            BlockchainClient::Esplora { inner } => inner.broadcast(tx),
        }
    }

    fn get_transactions(
        &self,
        txids: &[lwk_wollet::elements::Txid],
    ) -> std::result::Result<Vec<lwk_wollet::elements::Transaction>, lwk_wollet::Error> {
        match self {
            BlockchainClient::Electrum { inner } => inner.get_transactions(txids),
            BlockchainClient::Esplora { inner } => inner.get_transactions(txids),
        }
    }

    fn get_headers(
        &self,
        heights: &[u32],
        height_blockhash: &std::collections::HashMap<u32, lwk_wollet::elements::BlockHash>,
    ) -> std::result::Result<Vec<lwk_wollet::elements::BlockHeader>, lwk_wollet::Error> {
        match self {
            BlockchainClient::Electrum { inner } => inner.get_headers(heights, height_blockhash),
            BlockchainClient::Esplora { inner } => inner.get_headers(heights, height_blockhash),
        }
    }

    fn get_scripts_history(
        &self,
        scripts: &[&lwk_wollet::elements::Script],
    ) -> std::result::Result<Vec<Vec<lwk_wollet::History>>, lwk_wollet::Error> {
        match self {
            BlockchainClient::Electrum { inner } => inner.get_scripts_history(scripts),
            BlockchainClient::Esplora { inner } => inner.get_scripts_history(scripts),
        }
    }
}

/// The impl below is used for all Bitcoin-related instances, using [electrum_client::Client]
impl BlockchainClient<electrum_client::Client> {
    pub(crate) fn is_available(&self) -> bool {
        match self {
            BlockchainClient::Electrum { inner } => inner.ping().is_ok(),
            BlockchainClient::Esplora { .. } => true,
        }
    }

    pub(crate) fn try_from_explorer(
        exp: &BlockchainExplorer,
        network: LiquidNetwork,
    ) -> Result<Self> {
        match exp {
            BlockchainExplorer::Electrum { url } => {
                let (tls, validate_domain) = match network {
                    LiquidNetwork::Mainnet | LiquidNetwork::Testnet => (true, true),
                    LiquidNetwork::Regtest => (false, false),
                };
                let url = ElectrumUrl::new(url, tls, validate_domain)?;
                let client = url.build_client(&ElectrumOptions { timeout: Some(3) })?;
                // Ensure we ping so we know the client is working
                client.ping()?;
                Ok(BlockchainClient::Electrum {
                    inner: Box::new(client),
                })
            }
            BlockchainExplorer::Esplora {
                url,
                use_waterfalls,
            } => {
                let client = match *use_waterfalls {
                    true => EsploraClient::new(url, network.into()),
                    false => EsploraClient::new_waterfalls(url, network.into()),
                }?;
                Ok(BlockchainClient::Esplora {
                    inner: Box::new(client),
                })
            }
            BlockchainExplorer::MempoolSpace { .. } => {
                bail!("Cannot create client from MempoolSpace url")
            }
        }
    }

    // fn tip(&mut self) -> Result<lwk_wollet::elements::BlockHeader, lwk_wollet::Error> {
    //     todo!()
    // }
    //
    fn broadcast(
        &self,
        tx: &electrum_client::bitcoin::blockdata::transaction::Transaction,
    ) -> Result<lwk_wollet::elements::Txid, lwk_wollet::Error> {
        match self {
            BlockchainClient::Electrum { inner } => todo!(),
            BlockchainClient::Esplora { inner } => inner.broadcast(tx),
        }
    }
    //
    // fn get_transactions(
    //     &self,
    //     txids: &[lwk_wollet::elements::Txid],
    // ) -> Result<Vec<lwk_wollet::elements::Transaction>, lwk_wollet::Error> {
    //     todo!()
    // }
    //
    // fn get_headers(
    //     &self,
    //     heights: &[u32],
    //     height_blockhash: &std::collections::HashMap<Height, lwk_wollet::elements::BlockHash>,
    // ) -> Result<Vec<lwk_wollet::elements::BlockHeader>, lwk_wollet::Error> {
    //     todo!()
    // }

    // fn get_scripts_history(
    //     &self,
    //     scripts: &[&lwk_wollet::elements::Script],
    // ) -> Result<Vec<Vec<lwk_wollet::History>>, lwk_wollet::Error> {
    //     todo!()
    // }
}
