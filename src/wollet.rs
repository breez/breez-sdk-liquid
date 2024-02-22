use std::{time::Duration, thread, str::FromStr};

use boltz_client::swaps::boltz::{BoltzApiClient, BOLTZ_MAINNET_URL, BOLTZ_TESTNET_URL, CreateSwapRequest};
use lightning_invoice::Bolt11Invoice;
use lwk_common::Signer;
use lwk_signer::{AnySigner, SwSigner};
use anyhow::{anyhow, Result};
use lwk_wollet::{Wollet as LwkWollet, ElectrumUrl, ElementsNetwork, EncryptedFsPersister, WolletDescriptor, full_scan_with_electrum_client, ElectrumClient, elements::{Address, pset::PartiallySignedTransaction, Txid}, BlockchainBackend};

const DEFAULT_DB_DIR: &str = ".wollet";
const MAX_SCAN_RETRIES: u64 = 100;
const SCAN_DELAY_SEC: u64 = 6;

pub struct Wollet {
    wollet: LwkWollet,
    signer: SwSigner,
    db_root_dir: String,
    electrum_url: ElectrumUrl,
    network: ElementsNetwork,
}

pub struct WolletOptions {
    pub signer: SwSigner,
    pub network: ElementsNetwork,
    pub electrum_url: ElectrumUrl,
    pub desc: String,
    pub db_root_dir: Option<String>
}

#[derive(thiserror::Error, Debug)]
enum SwapError {
    #[error("Could not contact Boltz servers")]
    ServersUnreachable,

    #[error("Invoice amount is too low to be swapped")]
    AmountTooLow,

    #[error("An invoice with an amount is required")]
    ExpectedAmount,
}

impl Wollet {
    pub fn new(opts: WolletOptions) -> Result<Self> {
        let desc: WolletDescriptor = opts.desc.parse()?;
        let db_root_dir = opts.db_root_dir.unwrap_or(DEFAULT_DB_DIR.to_string());

        let persister = EncryptedFsPersister::new(&db_root_dir, opts.network, &desc)?;
        let wollet = LwkWollet::new(opts.network, persister, &opts.desc)?;
        
        Ok(Wollet {
            wollet,
            db_root_dir,
            signer: opts.signer,
            network: opts.network,
            electrum_url: opts.electrum_url,
        })
    }

    pub fn scan(&mut self) -> Result<(), lwk_wollet::Error> {
        let mut electrum_client = ElectrumClient::new(&self.electrum_url)?;
        full_scan_with_electrum_client(&mut self.wollet, &mut electrum_client)
    }

    pub fn address(&self, index: Option<u32>) -> Result<Address> {
        Ok(self.wollet.address(index)?.address().clone())
    }

    pub fn total_balance_sat(&mut self) -> Result<u64> {
        let balance = self.wollet.balance()?;
        Ok(balance.values().sum())
    }

    pub fn wait_for_tx(&mut self, txid: &Txid) -> Result<()> {
        let mut electrum_client: ElectrumClient = ElectrumClient::new(&self.electrum_url)?;
        for _ in 0..MAX_SCAN_RETRIES {
            full_scan_with_electrum_client(&mut self.wollet, &mut electrum_client)?;
            let list = self.wollet.transactions()?;
            if list.iter().any(|e| &e.tx.txid() == txid) {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(SCAN_DELAY_SEC));
        }

        Err(anyhow!("Wallet does not have {} in its list", txid))
    }

    pub fn wait_balance_change(&mut self) -> Result<u64> {
        let initial_balance = self.total_balance_sat()?;

        for _ in 0..MAX_SCAN_RETRIES {
            let new_balance = self.total_balance_sat()?;
            if new_balance != initial_balance {
                return Ok(new_balance)
            }
            thread::sleep(Duration::from_secs(SCAN_DELAY_SEC));
        }

        Err(anyhow!("Balance did not change over {} seconds", MAX_SCAN_RETRIES * SCAN_DELAY_SEC))
    }

    pub fn get_signer(&self) -> SwSigner {
        self.signer.clone()
    }

    pub fn get_descriptor(&self) -> WolletDescriptor {
        self.wollet.wollet_descriptor()
    }

    pub fn send_lbtc(
        &mut self,
        signers: &[AnySigner],
        fee_rate: Option<f32>,
        recipient: &Address,
        amount_sat: u64
    ) -> Result<Txid> {
        let mut pset = self.wollet
            .send_lbtc(amount_sat, &recipient.to_string(), fee_rate)?;

        for signer in signers {
            signer.sign(&mut pset)?;
        }

        self.send(&mut pset)
    }

    fn send(&mut self, pset: &mut PartiallySignedTransaction) -> Result<Txid> {
        let tx = self.wollet.finalize(pset)?;
        let electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let txid = electrum_client.broadcast(&tx)?;

        self.wait_for_tx(&txid)?;
        Ok(txid)
    }

    pub fn swap_to_ln(&mut self, invoice: String) -> Result<()> {
        let base_url = match self.network {
            ElementsNetwork::LiquidTestnet => BOLTZ_TESTNET_URL,
            ElementsNetwork::Liquid => BOLTZ_MAINNET_URL,
            ElementsNetwork::ElementsRegtest { .. } => todo!(),
        };

        let client = BoltzApiClient::new(base_url);
        let invoice = invoice.trim().parse::<Bolt11Invoice>()?;

        let pairs = client.get_pairs()
            .map_err(|_| SwapError::ServersUnreachable)?;

        let lbtc_pair = pairs.get_lbtc_pair()
            .map_err(|_| SwapError::ServersUnreachable)?;


        let amount_sat = invoice.amount_milli_satoshis().ok_or(SwapError::ExpectedAmount)? / 1000;
        if lbtc_pair.limits.minimal > amount_sat as i64 {
            return Err(SwapError::AmountTooLow.into());
        }

        let swap_response = client.create_swap(
            CreateSwapRequest::new_lbtc_submarine(&lbtc_pair.hash, &invoice.to_string(), "")
        ).map_err(|_| SwapError::ServersUnreachable)?;
        
        let funding_amount = swap_response.get_funding_amount()
            .map_err(|_| anyhow!("Could not get funding amount"))?;

        let funding_addr = swap_response.get_funding_address()
            .map_err(|_| anyhow!("Could not get funding address"))?;
        let funding_addr = Address::from_str(&funding_addr)?;

        let signer = AnySigner::Software(self.get_signer());
        self.send_lbtc(&[signer], None, &funding_addr, funding_amount)?;

        Ok(())
    }
}
