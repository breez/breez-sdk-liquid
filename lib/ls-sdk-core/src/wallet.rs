use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use boltz_client::{
    network::electrum::ElectrumConfig,
    swaps::{
        boltz::{
            BoltzApiClient, CreateSwapRequest, SubSwapStates, SwapStatusRequest, BOLTZ_MAINNET_URL,
            BOLTZ_TESTNET_URL,
        },
        liquid::{LBtcSwapScript, LBtcSwapTx},
    },
    util::secrets::{LBtcReverseRecovery, LiquidSwapKey, Preimage, SwapKey},
    Bolt11Invoice, Keypair,
};
use log::{debug, warn};
use lwk_common::{singlesig_desc, Signer, Singlesig};
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::{
    elements::{Address, Transaction},
    full_scan_with_electrum_client,
    hashes::{sha256t_hash_newtype, Hash},
    BlockchainBackend, ElectrumClient, ElectrumUrl, ElementsNetwork, FsPersister,
    Wollet as LwkWollet, WolletDescriptor,
};

use crate::{
    ensure_sdk, get_invoice_amount, persist::Persister, Network, OngoingSwap, Payment, PaymentData,
    PaymentError, PaymentType, PrepareReceiveRequest, PrepareReceiveResponse, PrepareSendResponse,
    ReceivePaymentResponse, SendPaymentResponse, WalletInfo, WalletOptions, CLAIM_ABSOLUTE_FEES,
    DEFAULT_DATA_DIR,
};

sha256t_hash_newtype! {
    struct DirectoryIdTag = hash_str("LWK-FS-Directory-Id/1.0");

    #[hash_newtype(forward)]
    struct DirectoryIdHash(_);
}

pub struct Wallet {
    signer: SwSigner,
    electrum_url: ElectrumUrl,
    network: Network,
    wallet: Arc<Mutex<LwkWollet>>,
    active_address: Option<u32>,
    persister: Persister,
    data_dir_path: String,
}

impl Wallet {
    pub fn init(mnemonic: &str, data_dir: Option<String>, network: Network) -> Result<Arc<Wallet>> {
        let is_mainnet = network == Network::Liquid;
        let signer = SwSigner::new(mnemonic, is_mainnet)?;
        let descriptor = Wallet::get_descriptor(&signer, network)?;

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
        let electrum_url = opts.get_electrum_url();
        let data_dir_path = opts.data_dir_path.unwrap_or(DEFAULT_DATA_DIR.to_string());

        let lwk_persister = FsPersister::new(&data_dir_path, network.into(), &opts.descriptor)?;
        let wallet = Arc::new(Mutex::new(LwkWollet::new(
            elements_network,
            lwk_persister,
            opts.descriptor,
        )?));

        fs::create_dir_all(&data_dir_path)?;

        let persister = Persister::new(&data_dir_path, network)?;
        persister.init()?;

        let wallet = Arc::new(Wallet {
            wallet,
            network,
            electrum_url,
            signer: opts.signer,
            active_address: None,
            persister,
            data_dir_path,
        });

        Wallet::track_pending_swaps(&wallet)?;

        Ok(wallet)
    }

    fn get_descriptor(signer: &SwSigner, network: Network) -> Result<WolletDescriptor> {
        let is_mainnet = network == Network::Liquid;
        let descriptor_str = singlesig_desc(
            signer,
            Singlesig::Wpkh,
            lwk_common::DescriptorBlindingKey::Slip77,
            is_mainnet,
        )
        .map_err(|e| anyhow!("Invalid descriptor: {e}"))?;
        Ok(descriptor_str.parse()?)
    }

    fn track_pending_swaps(self: &Arc<Wallet>) -> Result<()> {
        let cloned = self.clone();
        let client = self.boltz_client();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let ongoing_swaps = cloned.persister.list_ongoing_swaps().unwrap();

            for swap in ongoing_swaps {
                match swap {
                    OngoingSwap::Receive {
                        id,
                        preimage,
                        redeem_script,
                        blinding_key,
                        invoice,
                        ..
                    } => match cloned.try_claim(&preimage, &redeem_script, &blinding_key, None) {
                        Ok(txid) => {
                            let invoice_amount_sat = get_invoice_amount!(invoice);
                            cloned
                                .persister
                                .resolve_ongoing_swap(
                                    &id,
                                    Some((txid, PaymentData { invoice_amount_sat })),
                                )
                                .unwrap_or_else(|err| {
                                    warn!("Could not write to database. Err: {err:?}")
                                });
                        }
                        Err(err) => {
                            if let PaymentError::AlreadyClaimed = err {
                                warn!("Funds already claimed");
                                cloned.persister.resolve_ongoing_swap(&id, None).unwrap()
                            }
                            warn!("Could not claim yet. Err: {err}");
                        }
                    },
                    OngoingSwap::Send {
                        id, invoice, txid, ..
                    } => {
                        let Some(txid) = txid else {
                            continue;
                        };

                        let Ok(status_response) =
                            client.swap_status(SwapStatusRequest { id: id.clone() })
                        else {
                            continue;
                        };

                        if status_response.status == SubSwapStates::TransactionClaimed.to_string() {
                            let invoice_amount_sat = get_invoice_amount!(invoice);
                            cloned
                                .persister
                                .resolve_ongoing_swap(
                                    &id,
                                    Some((txid, PaymentData { invoice_amount_sat })),
                                )
                                .unwrap_or_else(|err| {
                                    warn!("Could not write to database. Err: {err:?}")
                                });
                        }
                    }
                }
            }
        });

        Ok(())
    }

    fn scan(&self) -> Result<(), lwk_wollet::Error> {
        let mut electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let mut wallet = self.wallet.lock().unwrap();
        full_scan_with_electrum_client(&mut wallet, &mut electrum_client)
    }

    fn address(&self) -> Result<Address, lwk_wollet::Error> {
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

    fn get_network_config(&self) -> ElectrumConfig {
        ElectrumConfig::new(
            self.network.into(),
            &self.electrum_url.to_string(),
            true,
            false,
            100,
        )
    }

    fn build_tx(
        &self,
        fee_rate: Option<f32>,
        recipient_address: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        let wallet = self.wallet.lock().unwrap();
        let mut pset = wallet.send_lbtc(amount_sat, recipient_address, fee_rate)?;
        let signer = AnySigner::Software(self.get_signer());
        signer.sign(&mut pset)?;
        Ok(wallet.finalize(&mut pset)?)
    }

    pub fn prepare_send_payment(&self, invoice: &str) -> Result<PrepareSendResponse, PaymentError> {
        let client = self.boltz_client();
        let invoice = invoice
            .trim()
            .parse::<Bolt11Invoice>()
            .map_err(|_| PaymentError::InvalidInvoice)?;

        // TODO Separate error type? Or make WalletError more generic?
        let lbtc_pair = client
            .get_pairs()?
            .get_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        let invoice_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::AmountOutOfRange)?
            / 1000;

        lbtc_pair
            .limits
            .within(invoice_amount_sat)
            .map_err(|_| PaymentError::AmountOutOfRange)?;

        let swap_response = client.create_swap(CreateSwapRequest::new_lbtc_submarine(
            &lbtc_pair.hash,
            &invoice.to_string(),
            "",
        ))?;

        let id = swap_response.get_id();
        let funding_address = swap_response.get_funding_address()?;
        let onchain_amount_sat = swap_response.get_funding_amount()?;
        let network_fees = self
            .build_tx(None, &funding_address.to_string(), onchain_amount_sat)?
            .all_fees()
            .values()
            .sum();

        self.persister
            .insert_or_update_ongoing_swap(&[OngoingSwap::Send {
                id: id.clone(),
                funding_address: funding_address.clone(),
                invoice: invoice.to_string(),
                onchain_amount_sat: onchain_amount_sat + network_fees,
                txid: None,
            }])
            .map_err(|_| PaymentError::PersistError)?;

        Ok(PrepareSendResponse {
            id,
            funding_address,
            invoice: invoice.to_string(),
            invoice_amount_sat,
            onchain_amount_sat,
            network_fees,
        })
    }

    pub fn send_payment(
        &self,
        res: &PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        let tx = self.build_tx(None, &res.funding_address, res.onchain_amount_sat)?;

        let electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let txid = electrum_client.broadcast(&tx)?.to_string();

        self.persister
            .insert_or_update_ongoing_swap(&[OngoingSwap::Send {
                id: res.id.clone(),
                funding_address: res.funding_address.clone(),
                invoice: res.invoice.clone(),
                onchain_amount_sat: res.onchain_amount_sat + res.network_fees,
                txid: Some(txid.clone()),
            }])
            .map_err(|_| PaymentError::PersistError)?;

        Ok(SendPaymentResponse { txid })
    }

    fn try_claim(
        &self,
        preimage: &str,
        redeem_script: &str,
        blinding_key: &str,
        absolute_fees: Option<u64>,
    ) -> Result<String, PaymentError> {
        let network_config = &self.get_network_config();
        let rev_swap_tx = LBtcSwapTx::new_claim(
            LBtcSwapScript::reverse_from_str(redeem_script, blinding_key)?,
            self.address()?.to_string(),
            network_config,
        )?;

        let mnemonic = self.signer.mnemonic().ok_or(PaymentError::SignerError {
            err: "Could not claim: Mnemonic not found".to_string(),
        })?;
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.network.into(), 0)?;

        let lsk = LiquidSwapKey::try_from(swap_key)?;

        let signed_tx = rev_swap_tx.sign_claim(
            &lsk.keypair,
            &Preimage::from_str(preimage)?,
            absolute_fees.unwrap_or(CLAIM_ABSOLUTE_FEES),
        )?;
        let txid = rev_swap_tx.broadcast(signed_tx, network_config)?;

        Ok(txid)
    }

    pub fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        let client = self.boltz_client();
        let lbtc_pair = client
            .get_pairs()?
            .get_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        let (receiver_amount_sat, payer_amount_sat) =
            match (req.receiver_amount_sat, req.payer_amount_sat) {
                (Some(onchain_amount_sat), None) => {
                    let fees_lockup = lbtc_pair.fees.reverse_lockup();
                    let fees_claim = CLAIM_ABSOLUTE_FEES; // lbtc_pair.fees.reverse_claim_estimate();
                    let p = lbtc_pair.fees.percentage;

                    let temp_recv_amt = onchain_amount_sat;
                    let invoice_amt_minus_service_fee = temp_recv_amt + fees_lockup + fees_claim;
                    let invoice_amount_sat =
                        (invoice_amt_minus_service_fee as f64 * 100.0 / (100.0 - p)).ceil() as u64;

                    Ok((onchain_amount_sat, invoice_amount_sat))
                }
                (None, Some(invoice_amount_sat)) => {
                    let fees_boltz = lbtc_pair.fees.reverse_boltz(invoice_amount_sat);
                    let fees_lockup = lbtc_pair.fees.reverse_lockup();
                    let fees_claim = CLAIM_ABSOLUTE_FEES; // lbtc_pair.fees.reverse_claim_estimate();
                    let fees_total = fees_boltz + fees_lockup + fees_claim;

                    ensure_sdk!(
                        invoice_amount_sat > fees_total,
                        PaymentError::AmountOutOfRange
                    );

                    Ok((invoice_amount_sat - fees_total, invoice_amount_sat))
                }
                (None, None) => Err(PaymentError::AmountOutOfRange),

                // TODO The request should not allow setting both invoice and onchain amounts, so this case shouldn't be possible.
                //      See example of how it's done in the SDK.
                _ => Err(PaymentError::BoltzError {
                    err: "Both invoice and onchain amounts were specified".into(),
                }),
            }?;

        lbtc_pair
            .limits
            .within(payer_amount_sat)
            .map_err(|_| PaymentError::AmountOutOfRange)?;

        debug!("Creating reverse swap with: receiver_amount_sat {receiver_amount_sat} sat, payer_amount_sat {payer_amount_sat} sat");

        Ok(PrepareReceiveResponse {
            pair_hash: lbtc_pair.hash,
            payer_amount_sat,
            fees_sat: payer_amount_sat - receiver_amount_sat,
        })
    }

    pub fn receive_payment(
        &self,
        res: &PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        let client = self.boltz_client();
        let mnemonic = self.signer.mnemonic().ok_or(PaymentError::SignerError {
            err: "Could not claim: Mnemonic not found".to_string(),
        })?;
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.network.into(), 0)?;
        let lsk = LiquidSwapKey::try_from(swap_key)?;

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;
        let preimage_hash = preimage.sha256.to_string();

        let swap_response = client.create_swap(CreateSwapRequest::new_lbtc_reverse_invoice_amt(
            res.pair_hash.clone(),
            preimage_hash.clone(),
            lsk.keypair.public_key().to_string(),
            res.payer_amount_sat,
        ))?;

        let swap_id = swap_response.get_id();
        let invoice = swap_response.get_invoice()?;
        let blinding_str = swap_response.get_blinding_key()?;
        let redeem_script = swap_response.get_redeem_script()?;
        let invoice_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::InvalidInvoice)?
            / 1000;

        // Double check that the generated invoice includes our data
        // https://docs.boltz.exchange/v/api/dont-trust-verify#lightning-invoice-verification
        if invoice.payment_hash().to_string() != preimage_hash {
            return Err(PaymentError::InvalidInvoice);
        };

        self.persister
            .insert_or_update_ongoing_swap(dbg!(&[OngoingSwap::Receive {
                id: swap_id.clone(),
                preimage: preimage_str,
                blinding_key: blinding_str,
                redeem_script,
                invoice: invoice.to_string(),
                receiver_amount_sat: invoice_amount_sat - res.fees_sat,
            }]))
            .map_err(|_| PaymentError::PersistError)?;

        Ok(ReceivePaymentResponse {
            id: swap_id,
            invoice: invoice.to_string(),
        })
    }

    pub fn list_payments(&self, with_scan: bool, include_pending: bool) -> Result<Vec<Payment>> {
        if with_scan {
            self.scan()?;
        }

        let transactions = self.wallet.lock().unwrap().transactions()?;

        let payment_data = self.persister.get_payment_data()?;
        let mut payments: Vec<Payment> = transactions
            .iter()
            .map(|tx| {
                let id = tx.txid.to_string();
                let data = payment_data.get(&id);
                let amount_sat = tx.balance.values().sum::<i64>();

                Payment {
                    id: Some(id.clone()),
                    timestamp: tx.timestamp,
                    amount_sat: amount_sat.unsigned_abs(),
                    payment_type: match amount_sat >= 0 {
                        true => PaymentType::Received,
                        false => PaymentType::Sent,
                    },
                    invoice: None,
                    fees_sat: data
                        .map(|d| (amount_sat.abs() - d.invoice_amount_sat as i64).unsigned_abs()),
                }
            })
            .collect();

        if include_pending {
            for swap in self.persister.list_ongoing_swaps()? {
                payments.insert(0, swap.into());
            }
        }

        Ok(payments)
    }

    pub fn recover_funds(&self, recovery: &LBtcReverseRecovery) -> Result<String> {
        let script: LBtcSwapScript = recovery.try_into().unwrap();
        let network_config = self.get_network_config();
        debug!("{:?}", script.fetch_utxo(&network_config));

        let tx =
            LBtcSwapTx::new_claim(script.clone(), self.address()?.to_string(), &network_config)
                .expect("Expecting valid tx");
        let keypair: Keypair = recovery.try_into().unwrap();
        let preimage: Preimage = recovery.try_into().unwrap();

        let signed_tx = tx.sign_claim(&keypair, &preimage, 1_000).unwrap();
        let txid = tx.broadcast(signed_tx, &network_config).unwrap();

        debug!("Funds recovered successfully! Txid: {txid}");

        Ok(txid)
    }

    pub fn empty_wallet_cache(&self) -> Result<()> {
        let mut path = PathBuf::from(self.data_dir_path.clone());
        path.push(Into::<ElementsNetwork>::into(self.network).as_str());
        path.push("enc_cache");

        let descriptor = Wallet::get_descriptor(&self.get_signer(), self.network)?;
        path.push(DirectoryIdHash::hash(descriptor.to_string().as_bytes()).to_string());

        fs::remove_dir_all(&path)?;
        fs::create_dir_all(path)?;

        Ok(())
    }
}
