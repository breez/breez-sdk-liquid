use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::ElementsAddress;
use lwk_common::Signer;
use lwk_common::{singlesig_desc, Singlesig};
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::{
    elements::{Address, Transaction},
    ElectrumClient, ElectrumUrl, ElementsNetwork, FsPersister, Tip, WalletTx, Wollet,
    WolletDescriptor,
};
use sdk_common::bitcoin::secp256k1::Secp256k1;
use sdk_common::bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};
use tokio::sync::Mutex;

use crate::{
    error::PaymentError,
    model::{Config, LiquidNetwork},
};

#[async_trait]
pub trait OnchainWallet: Send + Sync {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError>;

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError>;

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError>;

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> Tip;

    /// Get the public key of the wallet
    fn pubkey(&self) -> String;

    fn derive_bip32_key(&self, path: Vec<ChildNumber>) -> Result<ExtendedPrivKey, PaymentError>;

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError>;
}

pub(crate) struct LiquidOnchainWallet {
    wallet: Arc<Mutex<Wollet>>,
    config: Config,
    pub(crate) lwk_signer: SwSigner,
}

impl LiquidOnchainWallet {
    pub(crate) fn new(mnemonic: String, config: Config) -> Result<Self> {
        let is_mainnet = config.network == LiquidNetwork::Mainnet;
        let lwk_signer = SwSigner::new(&mnemonic, is_mainnet)?;
        let descriptor = LiquidOnchainWallet::get_descriptor(&lwk_signer, config.network)?;
        let elements_network: ElementsNetwork = config.network.into();

        let lwk_persister = FsPersister::new(
            config.get_wallet_working_dir(&lwk_signer)?,
            elements_network,
            &descriptor,
        )?;
        let wollet = Wollet::new(elements_network, lwk_persister, descriptor)?;
        Ok(Self {
            wallet: Arc::new(Mutex::new(wollet)),
            lwk_signer,
            config,
        })
    }

    fn get_descriptor(
        signer: &SwSigner,
        network: LiquidNetwork,
    ) -> Result<WolletDescriptor, PaymentError> {
        let is_mainnet = network == LiquidNetwork::Mainnet;
        let descriptor_str = singlesig_desc(
            signer,
            Singlesig::Wpkh,
            lwk_common::DescriptorBlindingKey::Slip77,
            is_mainnet,
        )
        .map_err(|e| anyhow!("Invalid descriptor: {e}"))?;
        Ok(descriptor_str.parse()?)
    }
}

#[async_trait]
impl OnchainWallet for LiquidOnchainWallet {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError> {
        let wallet = self.wallet.lock().await;
        wallet.transactions().map_err(|e| PaymentError::Generic {
            err: format!("Failed to fetch wallet transactions: {e:?}"),
        })
    }

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate_sats_per_kvb: Option<f32>,
        recipient_address: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        let lwk_wollet = self.wallet.lock().await;
        let mut pset = lwk_wollet::TxBuilder::new(self.config.network.into())
            .add_lbtc_recipient(
                &ElementsAddress::from_str(recipient_address).map_err(|e| {
                    PaymentError::Generic {
                        err: format!(
                      "Recipient address {recipient_address} is not a valid ElementsAddress: {e:?}"
                  ),
                    }
                })?,
                amount_sat,
            )?
            .fee_rate(fee_rate_sats_per_kvb)
            .finish(&lwk_wollet)?;
        let signer = AnySigner::Software(self.lwk_signer.clone());
        signer.sign(&mut pset)?;
        Ok(lwk_wollet.finalize(&mut pset)?)
    }

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError> {
        Ok(self.wallet.lock().await.address(None)?.address().clone())
    }

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> Tip {
        self.wallet.lock().await.tip()
    }

    /// Get the public key of the wallet
    fn pubkey(&self) -> String {
        self.lwk_signer.xpub().public_key.to_string()
    }

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError> {
        let mut wallet = self.wallet.lock().await;
        let mut electrum_client = ElectrumClient::new(&ElectrumUrl::new(
            &self.config.liquid_electrum_url,
            true,
            true,
        ))?;
        lwk_wollet::full_scan_with_electrum_client(&mut wallet, &mut electrum_client)?;
        Ok(())
    }

    fn derive_bip32_key(&self, path: Vec<ChildNumber>) -> Result<ExtendedPrivKey, PaymentError> {
        let seed = self.lwk_signer.seed().ok_or(PaymentError::SignerError {
            err: "Could not get signer seed".to_string(),
        })?;

        let bip32_xpriv = ExtendedPrivKey::new_master(self.config.network.into(), &seed)?
            .derive_priv(&Secp256k1::new(), &path)?;
        Ok(bip32_xpriv)
    }
}
