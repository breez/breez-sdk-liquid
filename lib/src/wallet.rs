use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
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
use lwk_common::{singlesig_desc, Signer, Singlesig};
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::{
    elements::Address, full_scan_with_electrum_client, BlockchainBackend, ElectrumClient,
    ElectrumUrl, ElementsNetwork, NoPersist, Wollet as LwkWollet,
};

use crate::{
    persist::Persister, Network, OngoingReceiveSwap, Payment, PaymentType, ReceivePaymentRequest,
    SendPaymentResponse, SwapError, SwapLbtcResponse, WalletInfo, WalletOptions,
    CLAIM_ABSOLUTE_FEES, DEFAULT_DATA_DIR, DEFAULT_ELECTRUM_URL,
};

pub struct Wallet {
    signer: SwSigner,
    electrum_url: ElectrumUrl,
    network: Network,
    wallet: Arc<Mutex<LwkWollet>>,
    active_address: Option<u32>,
    swap_persister: Persister,
}

impl Wallet {
    pub fn init(mnemonic: &str, data_dir: Option<String>, network: Network) -> Result<Arc<Wallet>> {
        let signer = SwSigner::new(mnemonic, network == Network::Liquid)?;
        let descriptor = singlesig_desc(
            &signer,
            Singlesig::Wpkh,
            lwk_common::DescriptorBlindingKey::Slip77,
            false,
        )
        .map_err(|e| anyhow!("Invalid descriptor: {e}"))?;

        Wallet::new(WalletOptions {
            signer,
            descriptor,
            electrum_url: None,
            data_dir_path: data_dir,
            network,
        })
    }

    fn new(opts: WalletOptions) -> Result<Arc<Self>> {
        let network = opts.network;
        let elements_network: ElementsNetwork = opts.network.into();

        let lwk_persister = NoPersist::new();
        let wallet = Arc::new(Mutex::new(LwkWollet::new(
            elements_network,
            lwk_persister,
            &opts.descriptor,
        )?));

        let electrum_url =
            opts.electrum_url
                .unwrap_or(ElectrumUrl::new(DEFAULT_ELECTRUM_URL, true, false));
        let persister_path = opts.data_dir_path.unwrap_or(DEFAULT_DATA_DIR.to_string());
        fs::create_dir_all(&persister_path)?;

        let swap_persister = Persister::new(persister_path);
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
                        let OngoingReceiveSwap {
                            preimage,
                            redeem_script,
                            blinding_key,
                            ..
                        } = swap;
                        match cloned.try_claim(&preimage, &redeem_script, &blinding_key, None) {
                            Ok(_) => cloned.swap_persister.resolve_ongoing_swap(swap.id).unwrap(),
                            Err(e) => warn!("Could not claim yet. Err: {e}"),
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
            Network::LiquidTestnet => BOLTZ_TESTNET_URL,
            Network::Liquid => BOLTZ_MAINNET_URL,
        };

        BoltzApiClient::new(base_url)
    }

    fn get_chain(&self) -> Chain {
        match self.network {
            Network::Liquid => Chain::Liquid,
            Network::LiquidTestnet => Chain::LiquidTestnet,
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

        Ok(txid)
    }

    pub fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<SwapLbtcResponse, SwapError> {
        let mut amount_sat = req
            .onchain_amount_sat
            .or(req.invoice_amount_sat)
            .ok_or(SwapError::AmountOutOfRange)?;

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
        let preimage_str = preimage.to_string().ok_or(SwapError::InvalidPreimage)?;
        let preimage_hash = preimage.sha256.to_string();

        let swap_response = if req.onchain_amount_sat.is_some() {
            amount_sat += CLAIM_ABSOLUTE_FEES;
            client.create_swap(CreateSwapRequest::new_lbtc_reverse_onchain_amt(
                lbtc_pair.hash,
                preimage_hash.clone(),
                lsk.keypair.public_key().to_string(),
                amount_sat,
            ))?
        } else {
            client.create_swap(CreateSwapRequest::new_lbtc_reverse_invoice_amt(
                lbtc_pair.hash,
                preimage_hash.clone(),
                lsk.keypair.public_key().to_string(),
                amount_sat,
            ))?
        };

        let swap_id = swap_response.get_id();
        let invoice = swap_response.get_invoice()?;
        let blinding_str = swap_response.get_blinding_key()?;
        let redeem_script = swap_response.get_redeem_script()?;

        // Double check that the generated invoice includes our data
        // https://docs.boltz.exchange/v/api/dont-trust-verify#lightning-invoice-verification
        if invoice.payment_hash().to_string() != preimage_hash {
            return Err(SwapError::InvalidInvoice);
        };

        let invoice_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(SwapError::InvalidInvoice)?
            / 1000;

        self.swap_persister
            .insert_ongoing_swaps(dbg!(&[OngoingReceiveSwap {
                id: swap_id.clone(),
                preimage: preimage_str,
                blinding_key: blinding_str,
                redeem_script,
                invoice_amount_sat,
                onchain_amount_sat: req.onchain_amount_sat.unwrap_or(
                    invoice_amount_sat
                        - lbtc_pair.fees.reverse_boltz(invoice_amount_sat)?
                        - lbtc_pair.fees.reverse_lockup()?
                        - CLAIM_ABSOLUTE_FEES
                ),
            }]))
            .map_err(|_| SwapError::PersistError)?;

        Ok(SwapLbtcResponse {
            id: swap_id,
            invoice: invoice.to_string(),
        })
    }

    pub fn list_payments(&self, with_scan: bool, include_pending: bool) -> Result<Vec<Payment>> {
        if with_scan {
            self.scan()?;
        }

        let transactions = self.wallet.lock().unwrap().transactions()?;

        let mut payments: Vec<Payment> = transactions
            .iter()
            .map(|tx| {
                let amount_sat = tx.balance.values().sum::<i64>();

                Payment {
                    id: Some(tx.tx.txid().to_string()),
                    timestamp: tx.timestamp,
                    amount_sat: amount_sat.unsigned_abs(),
                    payment_type: match amount_sat >= 0 {
                        true => PaymentType::Received,
                        false => PaymentType::Sent,
                    },
                }
            })
            .collect();

        if include_pending {
            let pending_swaps = self.swap_persister.list_ongoing_swaps()?;

            for swap in pending_swaps {
                payments.insert(
                    0,
                    Payment {
                        id: None,
                        timestamp: None,
                        payment_type: PaymentType::PendingReceive,
                        amount_sat: swap.invoice_amount_sat,
                    },
                );
            }
        }

        Ok(payments)
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
