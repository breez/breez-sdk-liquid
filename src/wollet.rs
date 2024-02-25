use std::{time::Duration, thread, str::FromStr};

use boltz_client::{ZKKeyPair, swaps::{boltz::{BoltzApiClient, BOLTZ_MAINNET_URL, BOLTZ_TESTNET_URL, CreateSwapRequest, SwapStatusRequest}, liquid::{LBtcSwapScript, LBtcSwapTx}}, util::secrets::{Preimage, LBtcReverseRecovery}, network::{Chain, electrum::ElectrumConfig}, Keypair};
use lightning_invoice::Bolt11Invoice;
use log::debug;
use lwk_common::Signer;
use lwk_signer::{AnySigner, SwSigner};
use anyhow::{anyhow, Result};
use lwk_wollet::{Wollet as LwkWollet, ElectrumUrl, ElementsNetwork, EncryptedFsPersister, WolletDescriptor, full_scan_with_electrum_client, ElectrumClient, elements::{Address, pset::PartiallySignedTransaction, Txid}, BlockchainBackend};

const DEFAULT_DB_DIR: &str = ".wollet";
const MAX_SCAN_RETRIES: u64 = 100;
const SCAN_DELAY_SEC: u64 = 6;

const BLOCKSTREAM_ELECTRUM_URL: &str = "blockstream.info:465";

pub struct Wollet {
    wollet: LwkWollet,
    signer: SwSigner,
    electrum_url: ElectrumUrl,
    network: ElementsNetwork,
}

pub struct WolletOptions {
    pub signer: SwSigner,
    pub network: ElementsNetwork,
    pub desc: String,
    pub db_root_dir: Option<String>,
    pub electrum_url: Option<ElectrumUrl>,
}

pub struct SwapLbtcResponse {
    pub id: String,
    pub invoice: String,
    pub recovery: LBtcReverseRecovery,
    pub claim: ClaimDetails,
}

pub struct ClaimDetails {
    pub redeem_script: String,
    pub blinding_str: String,
    pub preimage: Preimage,
    pub absolute_fees: u64
}

pub enum SwapStatus {
    Created,
    Mempool,
}

impl ToString for SwapStatus {
    fn to_string(&self) -> String {
        match self {
            SwapStatus::Mempool => "transaction.mempool",
            SwapStatus::Created => "swap.created"
        }.to_string()
    }
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

#[allow(dead_code)]
impl Wollet {
    pub fn new(opts: WolletOptions) -> Result<Self> {
        let desc: WolletDescriptor = opts.desc.parse()?;
        let db_root_dir = opts.db_root_dir.unwrap_or(DEFAULT_DB_DIR.to_string());

        let persister = EncryptedFsPersister::new(&db_root_dir, opts.network, &desc)?;
        let wollet = LwkWollet::new(opts.network, persister, &opts.desc)?;

        let electrum_url = opts.electrum_url.unwrap_or(match opts.network {
            ElementsNetwork::Liquid | ElementsNetwork::LiquidTestnet => ElectrumUrl::new(BLOCKSTREAM_ELECTRUM_URL, true, false),
            ElementsNetwork::ElementsRegtest { .. } => todo!()
        });
        
        Ok(Wollet {
            wollet,
            electrum_url,
            signer: opts.signer,
            network: opts.network,
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
        self.scan()?;
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
    ) -> Result<String> {
        let mut pset = self.wollet
            .send_lbtc(amount_sat, &recipient.to_string(), fee_rate)?;

        for signer in signers {
            signer.sign(&mut pset)?;
        }

        self.send(&mut pset).map(|txid| txid.to_string())
    }

    fn send(&mut self, pset: &mut PartiallySignedTransaction) -> Result<String> {
        let tx = self.wollet.finalize(pset)?;
        let electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let txid = electrum_client.broadcast(&tx)?;

        self.wait_for_tx(&txid)?;
        Ok(txid.to_string())
    }

    fn boltz_client(&self) -> BoltzApiClient {
        let base_url = match self.network {
            ElementsNetwork::LiquidTestnet => BOLTZ_TESTNET_URL,
            ElementsNetwork::Liquid => BOLTZ_MAINNET_URL,
            ElementsNetwork::ElementsRegtest { .. } => todo!(),
        };

        BoltzApiClient::new(base_url)
    }

    pub fn get_chain(&self) -> Chain {
        match self.network {
            ElementsNetwork::Liquid => Chain::Liquid,
            ElementsNetwork::LiquidTestnet => Chain::LiquidTestnet,
            ElementsNetwork::ElementsRegtest { .. } => todo!(),
        }
    }

    fn get_network_config(&self) -> ElectrumConfig {
        ElectrumConfig::new(
            self.get_chain(),
            &self.electrum_url.to_string(),
            true,
            false,
            100
        )
    }

    pub fn swap_to_ln(&mut self, invoice: &str) -> Result<()> {
        let client = self.boltz_client();
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

    pub fn wait_boltz_swap(&self, id: &str, swap_status: SwapStatus) -> Result<()> {
        let client = self.boltz_client();
        let request = SwapStatusRequest { id: id.to_string() };

        loop {
            let response = client.swap_status(request).unwrap();

            if response.status == swap_status.to_string() {
                break
            }

            thread::sleep(Duration::from_secs(5));
        }

        Ok(())
    }

    pub fn swap_to_lbtc(
        &mut self, 
        amount_sat: u64
    ) -> Result<SwapLbtcResponse> {
        let client = self.boltz_client();

        let pairs = client.get_pairs()
            .map_err(|_| SwapError::ServersUnreachable)?;

        let lbtc_pair = pairs.get_lbtc_pair()
            .map_err(|_| SwapError::ServersUnreachable)?;

        if lbtc_pair.limits.minimal > amount_sat as i64 {
            return Err(SwapError::AmountTooLow.into());
        }

        let preimage = Preimage::new();
        let pubkey = self.address(None)?.to_string();

        let swap_response = client.create_swap(
            CreateSwapRequest::new_lbtc_reverse_invoice_amt(
                lbtc_pair.hash, 
                preimage.sha256.to_string(),
                pubkey, 
                amount_sat
            )
        )
            .unwrap();

        let swap_id = swap_response.get_id();
        let invoice = swap_response.get_invoice().unwrap();
        let blinding_str = swap_response.get_blinding_key().unwrap();
        let blinding_key = ZKKeyPair::from_str(&blinding_str).unwrap();
        let redeem_script = swap_response.get_redeem_script().unwrap();

        let recovery = LBtcReverseRecovery::new(
            &swap_id,
            &preimage,
            &keypair,
            &blinding_key,
            &redeem_script
        );

        self.wait_boltz_swap(&swap_id, SwapStatus::Created)?;

        Ok(SwapLbtcResponse {
            id: swap_id,
            invoice: invoice.to_string(),
            recovery,
            claim: ClaimDetails {
                redeem_script,
                blinding_str,
                preimage,
                absolute_fees: 900
            }
        })
    }

    pub fn claim_lbtc_swap_funds(
        &self, 
        claim: &ClaimDetails
    ) -> Result<String> {
        let network_config = &self.get_network_config();
        let rev_swap_tx = LBtcSwapTx::new_claim(
            LBtcSwapScript::reverse_from_str(&claim.redeem_script, &claim.blinding_str).unwrap(),
            self.address(None)?.to_string(),
            network_config
        )
        .unwrap();

        let signed_tx = rev_swap_tx
            .sign_claim(todo!(), &claim.preimage, claim.absolute_fees)
            .unwrap();
        let txid = rev_swap_tx.broadcast(signed_tx, network_config).unwrap();

        debug!("Funds claimed successfully! Txid: {txid}");

        Ok(txid)
    }

    pub fn recover_lbtc_swap_funds(&self, recovery: &LBtcReverseRecovery) -> Result<String> {
        let script: LBtcSwapScript = recovery.try_into().unwrap();
        let network_config = self.get_network_config();
        debug!("{:?}", script.fetch_utxo(&network_config));

        let mut tx =
            LBtcSwapTx::new_claim(script.clone(), self.address(None)?.to_string(), &network_config).expect("Expecting valid tx");
        let keypair: Keypair = recovery.try_into().unwrap();
        let preimage: Preimage = recovery.try_into().unwrap();

        let signed_tx = tx.sign_claim(&keypair, &preimage, 1_000).unwrap();
        let txid = tx.broadcast(signed_tx, &network_config).unwrap();

        debug!("Funds recovered successfully! Txid: {txid}");

        Ok(txid)
    }

}
