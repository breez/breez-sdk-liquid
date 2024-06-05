use anyhow::{anyhow, Result};
use boltz_client::ElementsAddress;
use lwk_common::{singlesig_desc, Singlesig};
use lwk_signer::{AnySigner, SwSigner};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

use async_trait::async_trait;
use lwk_common::Signer;
use lwk_wollet::{
    elements::{Address, Transaction},
    ElectrumClient, ElectrumUrl, ElementsNetwork, FsPersister, Tip, WalletTx, Wollet,
    WolletDescriptor,
};

use crate::{
    error::PaymentError,
    model::{Config, Network},
};

#[async_trait]
pub trait OnchainWallet: Send + Sync {
    /// List all transactions in the wallet
    async fn transactions(&self) -> Result<Vec<WalletTx>, PaymentError>;

    /// Build a transaction to send funds to a recipient
    async fn build_tx(
        &self,
        fee_rate: Option<f32>,
        recipient_address: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError>;

    /// Get the next unused address in the wallet
    async fn next_unused_address(&self) -> Result<Address, PaymentError>;

    /// Get the current tip of the blockchain the wallet is aware of
    async fn tip(&self) -> Tip;

    /// Get the public key of the wallet
    fn pubkey(&self) -> String;

    /// Perform a full scan of the wallet
    async fn full_scan(&self) -> Result<(), PaymentError>;

    async fn get_broadcast_fee_estimation(&self, amount_sat: u64) -> Result<u64, PaymentError>;
}

pub(crate) struct LiquidOnchainWallet {
    wallet: Arc<Mutex<Wollet>>,
    lwk_signer: SwSigner,
    config: Config,
}

impl LiquidOnchainWallet {
    pub(crate) fn new(mnemonic: String, config: Config) -> Result<Self> {
        let is_mainnet = config.network == Network::Mainnet;
        let lwk_signer = SwSigner::new(&mnemonic, is_mainnet)?;
        let descriptor = LiquidOnchainWallet::get_descriptor(&lwk_signer, config.network)?;
        let elements_network: ElementsNetwork = config.network.into();

        let lwk_persister =
            FsPersister::new(config.working_dir.clone(), elements_network, &descriptor)?;
        let wollet = Wollet::new(elements_network, lwk_persister, descriptor)?;
        Ok(Self {
            wallet: Arc::new(Mutex::new(wollet)),
            lwk_signer,
            config,
        })
    }

    fn get_descriptor(
        signer: &SwSigner,
        network: Network,
    ) -> Result<WolletDescriptor, PaymentError> {
        let is_mainnet = network == Network::Mainnet;
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
        fee_rate: Option<f32>,
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
            .fee_rate(fee_rate)
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
        let mut electrum_client =
            ElectrumClient::new(&ElectrumUrl::new(&self.config.electrum_url, true, true))?;
        lwk_wollet::full_scan_with_electrum_client(&mut wallet, &mut electrum_client)?;
        Ok(())
    }

    async fn get_broadcast_fee_estimation(&self, amount_sat: u64) -> Result<u64, PaymentError> {
        // TODO Replace this with own address when LWK supports taproot
        //  https://github.com/Blockstream/lwk/issues/31
        let temp_p2tr_addr = match self.config.network {
          Network::Mainnet => "lq1pqvzxvqhrf54dd4sny4cag7497pe38252qefk46t92frs7us8r80ja9ha8r5me09nn22m4tmdqp5p4wafq3s59cql3v9n45t5trwtxrmxfsyxjnstkctj",
          Network::Testnet => "tlq1pq0wqu32e2xacxeyps22x8gjre4qk3u6r70pj4r62hzczxeyz8x3yxucrpn79zy28plc4x37aaf33kwt6dz2nn6gtkya6h02mwpzy4eh69zzexq7cf5y5"
      };

        // Create a throw-away tx similar to the lockup tx, in order to estimate fees
        Ok(self
            .build_tx(None, temp_p2tr_addr, amount_sat)
            .await?
            .all_fees()
            .values()
            .sum())
    }
}
