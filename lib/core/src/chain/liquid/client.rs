use anyhow::Result;
use lwk_wollet::{
    asyncr::{EsploraClient, EsploraClientBuilder},
    elements::{BlockHeader, Script, Transaction, Txid},
    History,
};
use lwk_wollet::{blocking::BlockchainBackend, ElectrumClient, ElectrumOptions, ElectrumUrl};

use crate::model::{BlockchainExplorer, LiquidNetwork};

pub(crate) enum LiquidClient {
    Electrum {
        inner: Box<ElectrumClient>,
    },
    Esplora {
        inner: Box<EsploraClient>,
    },
}

impl LiquidClient {
    pub(crate) fn is_available(&self) -> bool {
        match self {
            LiquidClient::Electrum { inner } => inner.ping().is_ok(),
            LiquidClient::Esplora { .. } => true,
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
                let client =
                    ElectrumClient::with_options(&url, ElectrumOptions { timeout: Some(3) })?;
                // Ensure we ping so we know the client is working
                client.ping()?;
                Ok(LiquidClient::Electrum {
                    inner: Box::new(client),
                })
            }
            BlockchainExplorer::Esplora {
                url,
                use_waterfalls,
            } => {
                let client = EsploraClientBuilder::new(url, network.into())
                    .waterfalls(*use_waterfalls)
                    .build();
                Ok(LiquidClient::Esplora {
                    inner: Box::new(client),
                })
            }
        }
    }

    pub(crate) async fn update_wallet(
        &mut self,
        wollet: &mut lwk_wollet::Wollet,
        index: u32,
    ) -> Result<(), lwk_wollet::Error> {
        let update = match self {
            Self::Electrum { inner, .. } => {
                let state = wollet.state();
                inner.full_scan_to_index(&state, index)?
            }
            Self::Esplora { inner, .. } => inner.full_scan_to_index(wollet, index).await?,
        };
        if let Some(update) = update {
            wollet.apply_update(update)?;
        }
        Ok(())
    }
}

impl LiquidClient {
    pub(crate) async fn tip(&mut self) -> Result<BlockHeader> {
        Ok(match self {
            LiquidClient::Electrum { inner } => inner.tip()?,
            LiquidClient::Esplora { inner } => inner.tip().await?,
        })
    }

    pub(crate) async fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        Ok(match self {
            LiquidClient::Electrum { inner } => inner.broadcast(tx)?,
            LiquidClient::Esplora { inner } => inner.broadcast(tx).await?,
        })
    }

    pub(crate) async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        Ok(match self {
            LiquidClient::Electrum { inner } => inner.get_transactions(txids)?,
            LiquidClient::Esplora { inner } => inner.get_transactions(txids).await?,
        })
    }

    pub(crate) async fn get_scripts_history(
        &self,
        scripts: &[&Script],
    ) -> Result<Vec<Vec<History>>> {
        Ok(match self {
            LiquidClient::Electrum { inner } => inner.get_scripts_history(scripts)?,
            LiquidClient::Esplora { inner } => inner.get_scripts_history(scripts).await?,
        })
    }
}
