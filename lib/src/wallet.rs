use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;
use boltz_client::{
    network::{electrum::ElectrumConfig, Chain},
    swaps::{
        boltz::{BoltzApiClient, CreateSwapRequest, BOLTZ_MAINNET_URL, BOLTZ_TESTNET_URL},
        liquid::{LBtcSwapScript, LBtcSwapTx},
    },
    util::secrets::{LBtcReverseRecovery, LiquidSwapKey, Preimage, SwapKey},
    Bolt11Invoice, Keypair,
};
use log::{debug, warn};
use lwk_common::Signer;
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::{
    elements::Address, full_scan_with_electrum_client, BlockchainBackend, ElectrumClient,
    ElectrumUrl, ElementsNetwork, NoPersist, Wollet as LwkWollet,
};

use crate::{
    persist::Persister, OngoingSwap, SendPaymentResponse, SwapError, SwapLbtcResponse, WalletInfo,
    WalletOptions,
};

// To avoid sendrawtransaction error "min relay fee not met"
const CLAIM_ABSOLUTE_FEES: u64 = 134;
const DEFAULT_SWAPS_DIR: &str = ".data";
const BLOCKSTREAM_ELECTRUM_URL: &str = "blockstream.info:465";

pub struct Wallet {
    signer: SwSigner,
    electrum_url: ElectrumUrl,
    network: ElementsNetwork,
    wallet: Arc<Mutex<LwkWollet>>,
    active_address: Option<u32>,
    swap_persister: Persister,
}

#[allow(dead_code)]
impl Wallet {
    pub fn new(opts: WalletOptions) -> Result<Arc<Self>> {
        opts.desc.parse::<String>()?;
        let network: ElementsNetwork = opts.network.into();

        let lwk_persister = NoPersist::new();
        let wallet = Arc::new(Mutex::new(LwkWollet::new(
            network,
            lwk_persister,
            &opts.desc,
        )?));

        let electrum_url = opts.electrum_url.unwrap_or(match network {
            ElementsNetwork::Liquid | ElementsNetwork::LiquidTestnet => {
                ElectrumUrl::new(BLOCKSTREAM_ELECTRUM_URL, true, false)
            }
            ElementsNetwork::ElementsRegtest { .. } => todo!(),
        });

        let swap_persister =
            Persister::new(opts.db_root_path.unwrap_or(DEFAULT_SWAPS_DIR.to_string()));
        swap_persister.init()?;

        let wallet = Arc::new(Wallet {
            wallet,
            network,
            electrum_url,
            signer: opts.signer,
            active_address: None,
            swap_persister,
        });

        Wallet::track_claims(&wallet)?;

        Ok(wallet)
    }

    fn track_claims(self: &Arc<Wallet>) -> Result<()> {
        let cloned = self.clone();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let ongoing_swaps = cloned.swap_persister.list_ongoing_swaps().unwrap();

            thread::scope(|scope| {
                for swap in ongoing_swaps {
                    scope.spawn(|| {
                        let OngoingSwap {
                            preimage,
                            redeem_script,
                            blinding_key,
                            ..
                        } = swap;
                        match cloned.try_claim(&preimage, &redeem_script, &blinding_key, None) {
                            Ok(_) => cloned.swap_persister.resolve_ongoing_swap(swap.id).unwrap(),
                            Err(e) => warn!("Could not claim yet. Err: {}", e),
                        }
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

        let lbtc_pair = client.get_pairs()?.get_lbtc_pair()?;

        let amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(SwapError::AmountOutOfRange)?
            / 1000;

        lbtc_pair
            .limits
            .within(amount_sat)
            .map_err(|_| SwapError::AmountOutOfRange)?;

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

    fn try_claim(
        &self,
        preimage: &str,
        redeem_script: &str,
        blinding_key: &str,
        absolute_fees: Option<u64>,
    ) -> Result<String, SwapError> {
        let network_config = &self.get_network_config();
        let mut rev_swap_tx = LBtcSwapTx::new_claim(
            LBtcSwapScript::reverse_from_str(redeem_script, blinding_key)?,
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
            &Preimage::from_str(preimage)?,
            absolute_fees.unwrap_or(CLAIM_ABSOLUTE_FEES),
        )?;
        let txid = rev_swap_tx.broadcast(signed_tx, network_config)?;

        debug!("Funds claimed successfully! Txid: {txid}");

        Ok(txid)
    }

    pub fn receive_payment(&self, amount_sat: u64) -> Result<SwapLbtcResponse, SwapError> {
        let client = self.boltz_client();

        let lbtc_pair = client.get_pairs()?.get_lbtc_pair()?;

        lbtc_pair
            .limits
            .within(amount_sat)
            .map_err(|_| SwapError::AmountOutOfRange)?;

        let mnemonic = self.signer.mnemonic();
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.get_chain(), 0)?;
        let lsk = LiquidSwapKey::from(swap_key);

        let preimage = Preimage::new();
        let preimage_hash = preimage.sha256.to_string();

        let swap_response = client.create_swap(CreateSwapRequest::new_lbtc_reverse_invoice_amt(
            lbtc_pair.hash,
            preimage_hash.clone(),
            lsk.keypair.public_key().to_string(),
            amount_sat,
        ))?;

        let swap_id = swap_response.get_id();
        let invoice = swap_response.get_invoice()?;
        let blinding_str = swap_response.get_blinding_key()?;
        let redeem_script = swap_response.get_redeem_script()?;

        // Double check that the generated invoice includes our data
        // https://docs.boltz.exchange/v/api/dont-trust-verify#lightning-invoice-verification
        if invoice.payment_hash().to_string() != preimage_hash
            || invoice
                .amount_milli_satoshis()
                .ok_or(SwapError::InvalidInvoice)?
                / 1000
                != amount_sat
        {
            return Err(SwapError::InvalidInvoice);
        };

        self.swap_persister
            .insert_ongoing_swaps(&[OngoingSwap {
                id: swap_id.clone(),
                preimage: preimage.to_string().expect("Expecting valid preimage"),
                blinding_key: blinding_str,
                redeem_script,
            }])
            .map_err(|_| SwapError::WalletError)?;

        Ok(SwapLbtcResponse {
            id: swap_id,
            invoice: invoice.to_string(),
        })
    }

    pub fn recover_funds(&self, recovery: &LBtcReverseRecovery) -> Result<String> {
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
