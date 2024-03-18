use std::{
    collections::HashSet,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;
use boltz_client::{
    network::{electrum::ElectrumConfig, Chain},
    swaps::{
        boltz::{
            BoltzApiClient, CreateSwapRequest, SwapStatusRequest, BOLTZ_MAINNET_URL,
            BOLTZ_TESTNET_URL,
        },
        liquid::{LBtcSwapScript, LBtcSwapTx},
    },
    util::secrets::{LBtcReverseRecovery, LiquidSwapKey, Preimage, SwapKey},
    Keypair, ZKKeyPair,
};
use lightning_invoice::Bolt11Invoice;
use log::{debug, info};
use lwk_common::Signer;
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::{
    elements::Address, full_scan_with_electrum_client, BlockchainBackend, ElectrumClient,
    ElectrumUrl, ElementsNetwork, NoPersist, Wollet as LwkWollet,
};

use crate::{
    ClaimDetails, SendPaymentResponse, SwapError, SwapLbtcResponse, SwapStatus, WalletInfo,
    WalletOptions,
};

const CLAIM_ABSOLUTE_FEES: u64 = 0;
const DEFAULT_DB_DIR: &str = ".wollet";
const BLOCKSTREAM_ELECTRUM_URL: &str = "blockstream.info:465";

pub struct Wallet {
    signer: SwSigner,
    electrum_url: ElectrumUrl,
    network: ElementsNetwork,
    wallet: Arc<Mutex<LwkWollet>>,
    pending_claims: Arc<Mutex<HashSet<ClaimDetails>>>,
    active_address: Option<u32>,
}

#[allow(dead_code)]
impl Wallet {
    pub fn new(opts: WalletOptions) -> Result<Arc<Self>> {
        opts.desc.parse::<String>()?;
        let network: ElementsNetwork = opts.network.into();

        let persister = NoPersist::new();
        let wallet = Arc::new(Mutex::new(LwkWollet::new(network, persister, &opts.desc)?));

        let electrum_url = opts.electrum_url.unwrap_or(match network {
            ElementsNetwork::Liquid | ElementsNetwork::LiquidTestnet => {
                ElectrumUrl::new(BLOCKSTREAM_ELECTRUM_URL, true, false)
            }
            ElementsNetwork::ElementsRegtest { .. } => todo!(),
        });

        let wallet = Arc::new(Wallet {
            wallet,
            network,
            electrum_url,
            signer: opts.signer,
            pending_claims: Default::default(),
            active_address: None,
        });

        Wallet::track_claims(&wallet)?;

        Ok(wallet)
    }

    fn track_claims(self: &Arc<Wallet>) -> Result<()> {
        let cloned = self.clone();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let pending_claims = cloned.pending_claims.lock().unwrap();

            thread::scope(|scope| {
                for claim in pending_claims.iter() {
                    info!("Trying to claim at address {}", claim.lockup_address);

                    scope.spawn(|| match cloned.try_claim(claim) {
                        Ok(txid) => info!("Claim successful! Txid: {txid}"),
                        Err(e) => info!("Could not claim yet. Err: {}", e),
                    });
                }
            });
        });

        Ok(())
    }

    fn scan(&self) -> Result<(), lwk_wollet::Error> {
        let mut electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let mut wallet = self.wallet.lock().unwrap();
        full_scan_with_electrum_client(&mut wallet, &mut electrum_client)
    }

    fn address(&self) -> Result<Address> {
        let wallet = self.wallet.lock().unwrap();
        Ok(wallet.address(self.active_address)?.address().clone())
    }

    fn total_balance_sat(&self, with_scan: bool) -> Result<u64> {
        if with_scan {
            self.scan()?;
        }
        let balance = self.wallet.lock().unwrap().balance()?;
        Ok(balance.values().sum())
    }

    pub fn get_info(&self, with_scan: bool) -> Result<WalletInfo> {
        Ok(WalletInfo {
            balance_sat: self.total_balance_sat(with_scan)?,
            pubkey: self.signer.xpub().public_key.to_string(),
            active_address: self.address()?.to_string(),
        })
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

    fn get_chain(&self) -> Chain {
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

    fn sign_and_send(
        &self,
        signers: &[AnySigner],
        fee_rate: Option<f32>,
        recipient: &str,
        amount_sat: u64,
    ) -> Result<String> {
        let wallet = self.wallet.lock().unwrap();
        let electrum_client = ElectrumClient::new(&self.electrum_url)?;

        let mut pset = wallet.send_lbtc(amount_sat, recipient, fee_rate)?;

        for signer in signers {
            signer.sign(&mut pset)?;
        }

        let tx = wallet.finalize(&mut pset)?;
        let txid = electrum_client.broadcast(&tx)?;

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
            self.address()
                .map_err(|_| SwapError::WalletError)?
                .to_string(),
            network_config,
        )?;

        let mnemonic = self.signer.mnemonic();
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.get_chain(), 0)?;
        let lsk = LiquidSwapKey::from(swap_key);

        let signed_tx = rev_swap_tx.sign_claim(
            &lsk.keypair,
            &Preimage::from_str(&claim_details.preimage)?,
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
            preimage: preimage.to_string().unwrap(),
            absolute_fees: CLAIM_ABSOLUTE_FEES
        };

        self.pending_claims
            .lock()
            .unwrap()
            .insert(claim_details.clone());

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

        let mut tx =
            LBtcSwapTx::new_claim(script.clone(), self.address()?.to_string(), &network_config)
                .expect("Expecting valid tx");
        let keypair: Keypair = recovery.try_into().unwrap();
        let preimage: Preimage = recovery.try_into().unwrap();

        let signed_tx = tx.sign_claim(&keypair, &preimage, 1_000).unwrap();
        let txid = tx.broadcast(signed_tx, &network_config).unwrap();

        debug!("Funds recovered successfully! Txid: {txid}");

        Ok(txid)
    }
}
