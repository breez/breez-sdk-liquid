use std::collections::HashMap;

use anyhow::Result;
use lwk_wollet::{
    blocking::{BlockchainBackend, EsploraClient},
    elements::{BlockHash, BlockHeader, Script, Transaction, Txid},
    ElectrumClient, ElectrumOptions, ElectrumUrl, History,
};

use crate::model::{BlockchainExplorer, LiquidNetwork};

pub(crate) enum LiquidClient {
    Electrum { inner: Box<ElectrumClient> },
    Esplora { inner: Box<EsploraClient> },
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
                let client = match *use_waterfalls {
                    true => EsploraClient::new(url, network.into()),
                    false => EsploraClient::new_waterfalls(url, network.into()),
                }?;
                Ok(LiquidClient::Esplora {
                    inner: Box::new(client),
                })
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

impl BlockchainBackend for LiquidClient {
    fn tip(&mut self) -> Result<BlockHeader, lwk_wollet::Error> {
        match self {
            LiquidClient::Electrum { inner } => inner.tip(),
            LiquidClient::Esplora { inner } => inner.tip(),
        }
    }

    fn broadcast(&self, tx: &Transaction) -> Result<Txid, lwk_wollet::Error> {
        match self {
            LiquidClient::Electrum { inner } => inner.broadcast(tx),
            LiquidClient::Esplora { inner } => inner.broadcast(tx),
        }
    }

    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>, lwk_wollet::Error> {
        match self {
            LiquidClient::Electrum { inner } => inner.get_transactions(txids),
            LiquidClient::Esplora { inner } => inner.get_transactions(txids),
        }
    }

    fn get_headers(
        &self,
        heights: &[u32],
        height_blockhash: &HashMap<u32, BlockHash>,
    ) -> Result<Vec<BlockHeader>, lwk_wollet::Error> {
        match self {
            LiquidClient::Electrum { inner } => inner.get_headers(heights, height_blockhash),
            LiquidClient::Esplora { inner } => inner.get_headers(heights, height_blockhash),
        }
    }

    fn get_scripts_history(
        &self,
        scripts: &[&Script],
    ) -> Result<Vec<Vec<History>>, lwk_wollet::Error> {
        match self {
            LiquidClient::Electrum { inner } => inner.get_scripts_history(scripts),
            LiquidClient::Esplora { inner } => inner.get_scripts_history(scripts),
        }
    }
}
