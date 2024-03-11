use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use boltz_client::{
    network::{electrum::ElectrumConfig, Chain},
    swaps::{
        boltz::{
            BoltzApiClient, CreateSwapRequest, SwapStatusRequest, BOLTZ_MAINNET_URL,
            BOLTZ_TESTNET_URL,
        },
        liquid::{LBtcSwapScript, LBtcSwapTx},
    },
    util::{
        error::S5Error,
        secrets::{LBtcReverseRecovery, LiquidSwapKey, Preimage, SwapKey},
    },
    Keypair, ZKKeyPair,
};
use lightning_invoice::Bolt11Invoice;
use log::{debug, info};
use lwk_common::Signer;
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::{
    elements::{Address, Txid},
    full_scan_with_electrum_client, BlockchainBackend, ElectrumClient, ElectrumUrl,
    ElementsNetwork, EncryptedFsPersister, Wollet as LwkWollet, WolletDescriptor,
};

const DEFAULT_DB_DIR: &str = ".wollet";
const MAX_SCAN_RETRIES: u64 = 100;
const SCAN_DELAY_SEC: u64 = 6;

const BLOCKSTREAM_ELECTRUM_URL: &str = "blockstream.info:465";

pub struct BreezWollet {
    signer: SwSigner,
    electrum_url: ElectrumUrl,
    network: ElementsNetwork,
    wollet: Arc<Mutex<LwkWollet>>,
    pending_claims: Arc<Mutex<Vec<ClaimDetails>>>,
}

pub enum Network {
    Liquid,
    LiquidTestnet,
}

impl From<Network> for ElementsNetwork {
    fn from(value: Network) -> Self {
        match value {
            Network::Liquid => ElementsNetwork::Liquid,
            Network::LiquidTestnet => ElementsNetwork::LiquidTestnet,
        }
    }
}

pub struct WolletOptions {
    pub signer: SwSigner,
    pub network: Network,
    pub desc: String,
    pub db_root_dir: Option<String>,
    pub electrum_url: Option<ElectrumUrl>,
}

#[derive(Debug)]
pub struct SwapLbtcResponse {
    pub id: String,
    pub invoice: String,
    pub claim_details: ClaimDetails,
    pub recovery_details: LBtcReverseRecovery,
}

#[derive(Debug, Clone)]
pub struct ClaimDetails {
    pub redeem_script: String,
    pub lockup_address: String,
    pub blinding_str: String,
    pub preimage: Preimage,
    pub absolute_fees: u64,
}

pub enum SwapStatus {
    Created,
    Mempool,
    Completed,
}

impl ToString for SwapStatus {
    fn to_string(&self) -> String {
        match self {
            SwapStatus::Mempool => "transaction.mempool",
            SwapStatus::Completed => "transaction.mempool",
            SwapStatus::Created => "swap.created",
        }
        .to_string()
    }
}

pub struct SendPaymentResponse {
    pub txid: String,
}

#[derive(thiserror::Error, Debug)]
pub enum SwapError {
    #[error("Could not contact Boltz servers")]
    ServersUnreachable,

    #[error("Invoice amount is out of range")]
    AmountOutOfRange,

    #[error("Wrong response received from Boltz servers")]
    BadResponse,

    #[error("The specified invoice is not valid")]
    InvalidInvoice,

    #[error("Could not sign/send the transaction")]
    SendError,

    #[error("Could not fetch the required wallet information")]
    WalletError,

    #[error("Generic boltz error: {err}")]
    BoltzGeneric { err: String },
}

impl From<S5Error> for SwapError {
    fn from(err: S5Error) -> Self {
        match err.kind {
            boltz_client::util::error::ErrorKind::Network
            | boltz_client::util::error::ErrorKind::BoltzApi => SwapError::ServersUnreachable,
            boltz_client::util::error::ErrorKind::Input => SwapError::BadResponse,
            _ => SwapError::BoltzGeneric { err: err.message },
        }
    }
}

#[allow(dead_code)]
impl BreezWollet {
    pub fn new(opts: WolletOptions) -> Result<Arc<Self>> {
        let desc: WolletDescriptor = opts.desc.parse()?;
        let db_root_dir = opts.db_root_dir.unwrap_or(DEFAULT_DB_DIR.to_string());
        let network: ElementsNetwork = opts.network.into();

        let persister = EncryptedFsPersister::new(db_root_dir, network, &desc)?;
        let wollet = Arc::new(Mutex::new(LwkWollet::new(network, persister, &opts.desc)?));

        let electrum_url = opts.electrum_url.unwrap_or(match network {
            ElementsNetwork::Liquid | ElementsNetwork::LiquidTestnet => {
                ElectrumUrl::new(BLOCKSTREAM_ELECTRUM_URL, true, false)
            }
            ElementsNetwork::ElementsRegtest { .. } => todo!(),
        });

        let wollet = Arc::new(BreezWollet {
            wollet,
            network,
            electrum_url,
            signer: opts.signer,
            pending_claims: Arc::default(),
        });

        BreezWollet::track_claims(&wollet)?;

        Ok(wollet)
    }

    fn track_claims(self: &Arc<BreezWollet>) -> Result<()> {
        let cloned = self.clone();

        thread::spawn(move || loop {
            let pending_claims = cloned.pending_claims.lock().unwrap();

            thread::scope(|scope| {
                pending_claims.iter().for_each(|claim| {
                    info!("Trying to claim at address {}", claim.lockup_address);
                    scope.spawn(|| {
                        cloned.try_claim(claim).unwrap();
                    });
                })
            });
            thread::sleep(Duration::from_secs(5));
        });

        Ok(())
    }

    fn scan(&self) -> Result<(), lwk_wollet::Error> {
        let mut electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let mut wollet = self.wollet.lock().unwrap();
        full_scan_with_electrum_client(&mut wollet, &mut electrum_client)
    }

    fn address(&self, index: Option<u32>) -> Result<Address> {
        let wollet = self.wollet.lock().unwrap();
        Ok(wollet.address(index)?.address().clone())
    }

    pub async fn total_balance_sat(&self, with_scan: bool) -> Result<u64> {
        if with_scan {
            self.scan()?;
        }
        let balance = self.wollet.lock().unwrap().balance()?;
        Ok(balance.values().sum())
    }

    fn wait_for_tx(&self, wollet: &mut LwkWollet, txid: &Txid) -> Result<()> {
        let mut electrum_client: ElectrumClient = ElectrumClient::new(&self.electrum_url)?;

        for _ in 0..MAX_SCAN_RETRIES {
            full_scan_with_electrum_client(wollet, &mut electrum_client)?;
            let list = wollet.transactions()?;
            if list.iter().any(|e| &e.tx.txid() == txid) {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(SCAN_DELAY_SEC));
        }

        Err(anyhow!("Wallet does not have {} in its list", txid))
    }

    fn get_signer(&self) -> SwSigner {
        self.signer.clone()
    }

    fn boltz_client(&self) -> BoltzApiClient {
        let base_url = match self.network {
            ElementsNetwork::LiquidTestnet => BOLTZ_TESTNET_URL,
            ElementsNetwork::Liquid => BOLTZ_MAINNET_URL,
            ElementsNetwork::ElementsRegtest { .. } => todo!(),
        };

        BoltzApiClient::new(base_url)
    }

    pub(crate) fn get_chain(&self) -> Chain {
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
            100,
        )
    }

    pub fn sign_and_send(
        &self,
        signers: &[AnySigner],
        fee_rate: Option<f32>,
        recipient: &str,
        amount_sat: u64,
    ) -> Result<String> {
        let cloned_wollet = self.wollet.clone();
        let mut wollet = cloned_wollet.lock().unwrap();
        let electrum_client = ElectrumClient::new(&self.electrum_url)?;

        let mut pset = wollet.send_lbtc(amount_sat, recipient, fee_rate)?;

        for signer in signers {
            signer.sign(&mut pset)?;
        }

        let tx = wollet.finalize(&mut pset)?;
        let txid = electrum_client.broadcast(&tx)?;

        self.wait_for_tx(&mut wollet, &txid)?;

        Ok(txid.to_string())
    }

    pub fn send_payment(&self, invoice: &str) -> Result<SendPaymentResponse, SwapError> {
        let client = self.boltz_client();
        let invoice = invoice
            .trim()
            .parse::<Bolt11Invoice>()
            .map_err(|_| SwapError::InvalidInvoice)?;

        let pairs = client.get_pairs()?;
        let lbtc_pair = pairs.get_lbtc_pair()?;

        let amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(SwapError::AmountOutOfRange)?
            / 1000;

        if lbtc_pair.limits.minimal > amount_sat as i64 {
            return Err(SwapError::AmountOutOfRange);
        }

        if lbtc_pair.limits.maximal < amount_sat as i64 {
            return Err(SwapError::AmountOutOfRange);
        }

        let swap_response = client.create_swap(CreateSwapRequest::new_lbtc_submarine(
            &lbtc_pair.hash,
            &invoice.to_string(),
            "",
        ))?;

        let funding_amount = swap_response.get_funding_amount()?;

        let funding_addr = swap_response.get_funding_address()?;

        let signer = AnySigner::Software(self.get_signer());

        let txid = self
            .sign_and_send(&[signer], None, &funding_addr, funding_amount)
            .map_err(|_| SwapError::SendError)?;

        Ok(SendPaymentResponse { txid })
    }

    fn wait_swap_status(&self, id: &str, swap_status: SwapStatus) -> Result<()> {
        let client = self.boltz_client();

        loop {
            let request = SwapStatusRequest { id: id.to_string() };
            let response = client.swap_status(request).unwrap();

            if response.status == swap_status.to_string() {
                break;
            }

            thread::sleep(Duration::from_secs(5));
        }

        Ok(())
    }

    fn try_claim(&self, claim_details: &ClaimDetails) -> Result<String, SwapError> {
        let network_config = &self.get_network_config();
        let mut rev_swap_tx = LBtcSwapTx::new_claim(
            LBtcSwapScript::reverse_from_str(
                &claim_details.redeem_script,
                &claim_details.blinding_str,
            )?,
            self.address(None)
                .map_err(|_| SwapError::WalletError)?
                .to_string(),
            network_config,
        )
        .unwrap();

        let mnemonic = self.signer.mnemonic();
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.get_chain(), 0)?;
        let lsk = LiquidSwapKey::from(swap_key);

        let signed_tx = rev_swap_tx.sign_claim(
            &lsk.keypair,
            &claim_details.preimage,
            claim_details.absolute_fees,
        )?;
        let txid = rev_swap_tx.broadcast(signed_tx, network_config)?;

        debug!("Funds claimed successfully! Txid: {txid}");

        Ok(txid)
    }

    pub fn receive_payment(&self, amount_sat: u64) -> Result<SwapLbtcResponse, SwapError> {
        let client = self.boltz_client();

        let pairs = client.get_pairs()?;

        let lbtc_pair = pairs.get_lbtc_pair()?;

        if lbtc_pair.limits.minimal > amount_sat as i64 {
            return Err(SwapError::AmountOutOfRange);
        }

        if lbtc_pair.limits.maximal < amount_sat as i64 {
            return Err(SwapError::AmountOutOfRange);
        }

        let mnemonic = self.signer.mnemonic();
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.get_chain(), 0)?;
        let lsk = LiquidSwapKey::from(swap_key);

        let preimage = Preimage::new();

        let swap_response = client.create_swap(CreateSwapRequest::new_lbtc_reverse_invoice_amt(
            lbtc_pair.hash,
            preimage.sha256.to_string(),
            lsk.keypair.public_key().to_string(),
            amount_sat,
        ))?;

        let swap_id = swap_response.get_id();
        let invoice = swap_response.get_invoice()?;
        let blinding_str = swap_response.get_blinding_key()?;
        let blinding_key =
            ZKKeyPair::from_str(&blinding_str).map_err(|_| SwapError::BadResponse)?;
        let redeem_script = swap_response.get_redeem_script()?;
        let lockup_address = swap_response.get_lockup_address()?;

        let recovery = LBtcReverseRecovery::new(
            &swap_id,
            &preimage,
            &lsk.keypair,
            &blinding_key,
            &redeem_script,
        );

        self.wait_swap_status(&swap_id, SwapStatus::Created)
            .map_err(|_| SwapError::ServersUnreachable)?;

        let claim_details = ClaimDetails {
            redeem_script,
            lockup_address,
            blinding_str,
            preimage,
            absolute_fees: 900,
        };

        self.pending_claims
            .lock()
            .unwrap()
            .push(claim_details.clone());

        Ok(SwapLbtcResponse {
            id: swap_id,
            invoice: invoice.to_string(),
            recovery_details: recovery,
            claim_details,
        })
    }

    pub async fn recover_funds(&self, recovery: &LBtcReverseRecovery) -> Result<String> {
        let script: LBtcSwapScript = recovery.try_into().unwrap();
        let network_config = self.get_network_config();
        debug!("{:?}", script.fetch_utxo(&network_config));

        let mut tx = LBtcSwapTx::new_claim(
            script.clone(),
            self.address(None)?.to_string(),
            &network_config,
        )
        .expect("Expecting valid tx");
        let keypair: Keypair = recovery.try_into().unwrap();
        let preimage: Preimage = recovery.try_into().unwrap();

        let signed_tx = tx.sign_claim(&keypair, &preimage, 1_000).unwrap();
        let txid = tx.broadcast(signed_tx, &network_config).unwrap();

        debug!("Funds recovered successfully! Txid: {txid}");

        Ok(txid)
    }
}
