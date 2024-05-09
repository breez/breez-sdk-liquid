use std::{
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use boltz_client::{
    network::electrum::ElectrumConfig,
    swaps::{
        boltz::{
            BoltzApiClient, CreateSwapRequest, RevSwapStates, SubSwapStates, SwapStatusRequest,
            BOLTZ_MAINNET_URL, BOLTZ_TESTNET_URL,
        },
        boltzv2::{
            BoltzApiClientV2, CreateSubmarineRequest, ReversePair, SubmarinePair, Subscription,
            BOLTZ_MAINNET_URL_V2, BOLTZ_TESTNET_URL_V2,
        },
        liquid::{LBtcSwapScript, LBtcSwapTx},
    },
    util::secrets::{LBtcReverseRecovery, LiquidSwapKey, Preimage, SwapKey},
    Bolt11Invoice, Keypair,
};
use elements::hashes::hex::DisplayHex;
use log::{debug, error, info, warn};
use lwk_common::{singlesig_desc, Signer, Singlesig};
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::{
    elements::{Address, Transaction},
    full_scan_with_electrum_client, BlockchainBackend, ElectrumClient, ElectrumUrl,
    ElementsNetwork, FsPersister, Wollet as LwkWollet, WolletDescriptor,
};

use crate::{
    ensure_sdk, error::PaymentError, get_invoice_amount, model::*, persist::Persister,
    utils::get_swap_status_v2,
};

/// Claim tx feerate, in sats per vbyte.
/// Since the  Liquid blocks are consistently empty for now, we hardcode the minimum feerate.
pub const LIQUID_CLAIM_TX_FEERATE: f32 = 0.1;

pub const DEFAULT_DATA_DIR: &str = ".data";

pub struct LiquidSdk {
    electrum_url: ElectrumUrl,
    network: Network,
    /// LWK Wollet, a watch-only Liquid wallet for this instance
    lwk_wollet: Arc<Mutex<LwkWollet>>,
    /// LWK Signer, for signing Liquid transactions
    lwk_signer: SwSigner,
    active_address: Option<u32>,
    persister: Persister,
    data_dir_path: String,
}

impl LiquidSdk {
    pub fn connect(req: ConnectRequest) -> Result<Arc<LiquidSdk>> {
        let is_mainnet = req.network == Network::Liquid;
        let signer = SwSigner::new(&req.mnemonic, is_mainnet)?;
        let descriptor = LiquidSdk::get_descriptor(&signer, req.network)?;

        LiquidSdk::new(LiquidSdkOptions {
            signer,
            descriptor,
            electrum_url: None,
            data_dir_path: req.data_dir,
            network: req.network,
        })
    }

    fn new(opts: LiquidSdkOptions) -> Result<Arc<Self>> {
        let network = opts.network;
        let elements_network: ElementsNetwork = opts.network.into();
        let electrum_url = opts.get_electrum_url();
        let data_dir_path = opts.data_dir_path.unwrap_or(DEFAULT_DATA_DIR.to_string());

        let lwk_persister = FsPersister::new(&data_dir_path, network.into(), &opts.descriptor)?;
        let lwk_wollet = Arc::new(Mutex::new(LwkWollet::new(
            elements_network,
            lwk_persister,
            opts.descriptor,
        )?));

        fs::create_dir_all(&data_dir_path)?;

        let persister = Persister::new(&data_dir_path, network)?;
        persister.init()?;

        let sdk = Arc::new(LiquidSdk {
            lwk_wollet,
            network,
            electrum_url,
            lwk_signer: opts.signer,
            active_address: None,
            persister,
            data_dir_path,
        });

        LiquidSdk::track_pending_swaps(&sdk)?;

        Ok(sdk)
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

    fn try_resolve_pending_swap(
        sdk: &Arc<LiquidSdk>,
        client: &BoltzApiClient,
        swap: &OngoingSwap,
    ) -> Result<()> {
        match swap {
            OngoingSwap::Receive {
                id,
                preimage,
                redeem_script,
                blinding_key,
                invoice,
                claim_fees_sat,
                ..
            } => {
                let status = client
                    .swap_status(SwapStatusRequest { id: id.clone() })
                    .map_err(|e| anyhow!("Failed to fetch swap status for ID {id}: {e:?}"))?
                    .status;

                let swap_state = status.parse::<RevSwapStates>().map_err(|_| {
                    anyhow!("Invalid reverse swap state received for swap {id}: {status}",)
                })?;

                match swap_state {
                    RevSwapStates::SwapExpired
                    | RevSwapStates::InvoiceExpired
                    | RevSwapStates::TransactionFailed
                    | RevSwapStates::TransactionRefunded => {
                        warn!("Cannot claim swap {id}, unrecoverable state: {status}");
                        sdk.persister
                            .resolve_ongoing_swap(id, None)
                            .map_err(|_| anyhow!("Could not resolve swap {id} in database"))?;
                    }
                    RevSwapStates::TransactionConfirmed => {}
                    _ => {
                        return Err(anyhow!("New swap state for reverse swap {id}: {status}"));
                    }
                }

                match sdk.try_claim(preimage, redeem_script, blinding_key, *claim_fees_sat) {
                    Ok(txid) => {
                        let payer_amount_sat = get_invoice_amount!(invoice);
                        sdk.persister
                            .resolve_ongoing_swap(
                                id,
                                Some((txid, PaymentData { payer_amount_sat })),
                            )
                            .map_err(|_| anyhow!("Could not resolve swap {id} in database"))?;
                    }
                    Err(err) => {
                        if let PaymentError::AlreadyClaimed = err {
                            warn!("Funds already claimed");
                            sdk.persister
                                .resolve_ongoing_swap(id, None)
                                .map_err(|_| anyhow!("Could not resolve swap {id} in database"))?;
                        }
                        warn!("Could not claim swap {id} yet. Err: {err}");
                    }
                }
            }
            OngoingSwap::Send {
                id, invoice, txid, ..
            } => {
                let Some(txid) = txid.clone() else {
                    return Err(anyhow!("Transaction not broadcast yet for swap {id}"));
                };

                let status = client
                    .swap_status(SwapStatusRequest { id: id.clone() })
                    .map_err(|e| anyhow!("Failed to fetch swap status for ID {id}: {e:?}"))?
                    .status;

                let state: SubSwapStates = status.parse().map_err(|_| {
                    anyhow!("Invalid submarine swap state received for swap {id}: {status}")
                })?;

                match state {
                    SubSwapStates::TransactionClaimed
                    | SubSwapStates::InvoiceFailedToPay
                    | SubSwapStates::SwapExpired => {
                        warn!("Cannot positively resolve swap {id}, unrecoverable state: {status}");

                        let payer_amount_sat = get_invoice_amount!(invoice);
                        sdk.persister
                            .resolve_ongoing_swap(
                                id,
                                Some((txid, PaymentData { payer_amount_sat })),
                            )
                            .map_err(|_| anyhow!("Could not resolve swap {id} in database"))?;
                    }
                    _ => {
                        return Err(anyhow!("New swap state for submarine swap {id}: {status}"));
                    }
                }
            }
        };

        Ok(())
    }

    fn track_pending_swaps(self: &Arc<LiquidSdk>) -> Result<()> {
        let cloned = self.clone();
        let client = self.boltz_client();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            match cloned.persister.list_ongoing_swaps() {
                Ok(ongoing_swaps) => {
                    for swap in ongoing_swaps {
                        match LiquidSdk::try_resolve_pending_swap(&cloned, &client, &swap) {
                            Ok(_) => info!("Resolved pending swap {}", swap.id()),
                            Err(err) => match swap {
                                OngoingSwap::Send { .. } => error!("[Ongoing Send] {err}"),
                                OngoingSwap::Receive { .. } => error!("[Ongoing Receive] {err}"),
                            },
                        }
                    }
                }
                Err(e) => {
                    error!("Could not read ongoing swaps from database: {e}");
                    continue;
                }
            }
        });

        Ok(())
    }

    fn scan(&self) -> Result<(), lwk_wollet::Error> {
        let mut electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let mut lwk_wollet = self.lwk_wollet.lock().unwrap();
        full_scan_with_electrum_client(&mut lwk_wollet, &mut electrum_client)
    }

    fn address(&self) -> Result<Address, lwk_wollet::Error> {
        let lwk_wollet = self.lwk_wollet.lock().unwrap();
        Ok(lwk_wollet.address(self.active_address)?.address().clone())
    }

    fn total_balance_sat(&self, with_scan: bool) -> Result<u64> {
        if with_scan {
            self.scan()?;
        }
        let balance = self.lwk_wollet.lock().unwrap().balance()?;
        Ok(balance.values().sum())
    }

    pub fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse> {
        debug!("active_address: {}", self.address()?);

        Ok(GetInfoResponse {
            balance_sat: self.total_balance_sat(req.with_scan)?,
            pubkey: self.lwk_signer.xpub().public_key.to_string(),
        })
    }

    fn get_signer(&self) -> SwSigner {
        self.lwk_signer.clone()
    }

    fn boltz_client(&self) -> BoltzApiClient {
        let base_url = match self.network {
            Network::LiquidTestnet => BOLTZ_TESTNET_URL,
            Network::Liquid => BOLTZ_MAINNET_URL,
        };

        BoltzApiClient::new(base_url)
    }

    fn boltz_client_v2(&self) -> BoltzApiClientV2 {
        let base_url = match self.network {
            Network::LiquidTestnet => BOLTZ_TESTNET_URL_V2,
            Network::Liquid => BOLTZ_MAINNET_URL_V2,
        };

        BoltzApiClientV2::new(base_url)
    }

    fn network_config(&self) -> ElectrumConfig {
        ElectrumConfig::new(
            self.network.into(),
            &self.electrum_url.to_string(),
            true,
            true,
            100,
        )
    }

    fn build_tx(
        &self,
        fee_rate: Option<f32>,
        recipient_address: &str,
        amount_sat: u64,
    ) -> Result<Transaction, PaymentError> {
        let lwk_wollet = self.lwk_wollet.lock().unwrap();
        let mut pset = lwk_wollet.send_lbtc(amount_sat, recipient_address, fee_rate)?;
        let signer = AnySigner::Software(self.get_signer());
        signer.sign(&mut pset)?;
        Ok(lwk_wollet.finalize(&mut pset)?)
    }

    fn validate_invoice(&self, invoice: &str) -> Result<Bolt11Invoice, PaymentError> {
        let invoice = invoice
            .trim()
            .parse::<Bolt11Invoice>()
            .map_err(|_| PaymentError::InvalidInvoice)?;

        match (invoice.network().to_string().as_str(), self.network) {
            ("bitcoin", Network::Liquid) => {}
            ("testnet", Network::LiquidTestnet) => {}
            _ => return Err(PaymentError::InvalidInvoice),
        }

        ensure_sdk!(!invoice.is_expired(), PaymentError::InvalidInvoice);

        Ok(invoice)
    }

    #[allow(dead_code)]
    fn validate_submarine_pairs(
        client: &BoltzApiClientV2,
        receiver_amount_sat: u64,
    ) -> Result<SubmarinePair, PaymentError> {
        let lbtc_pair = client
            .get_submarine_pairs()?
            .get_lbtc_to_btc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        lbtc_pair.limits.within(receiver_amount_sat)?;

        let fees_sat = lbtc_pair.fees.total(receiver_amount_sat);

        ensure_sdk!(
            receiver_amount_sat > fees_sat,
            PaymentError::AmountOutOfRange
        );

        Ok(lbtc_pair)
    }

    pub fn prepare_send_payment(
        &self,
        req: &PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        let invoice = self.validate_invoice(&req.invoice)?;
        let receiver_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::AmountOutOfRange)?
            / 1000;

        let client = self.boltz_client_v2();
        let lbtc_pair = Self::validate_submarine_pairs(&client, receiver_amount_sat)?;

        Ok(PrepareSendResponse {
            invoice: req.invoice.clone(),
            fees_sat: lbtc_pair.fees.total(receiver_amount_sat),
        })
    }

    pub fn send_payment(
        &self,
        req: &PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        let invoice = self.validate_invoice(&req.invoice)?;
        let receiver_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::AmountOutOfRange)?
            / 1000;

        let client = self.boltz_client_v2();
        let lbtc_pair = Self::validate_submarine_pairs(&client, receiver_amount_sat)?;

        ensure_sdk!(
            req.fees_sat == lbtc_pair.fees.total(receiver_amount_sat),
            PaymentError::InvalidOrExpiredFees
        );

        let lwk_wollet = self.lwk_wollet.lock().unwrap();
        // let our_pubkey = self
        //     .address()?
        //     .to_unconfidential()
        //     .blinding_pubkey
        //     .ok_or(PaymentError::Generic {
        //         err: "Could not retrieve wallet pubkey".to_string(),
        //     })?
        //     .into();
        let refund_public_key = lwk_wollet
            .address(None)?
            .address()
            .blinding_pubkey
            .ok_or(PaymentError::Generic {
                err: "Could not generate refund pubkey".to_string(),
            })?
            .into();

        // Unlock lwk wallet so it can be used to build the tx
        std::mem::drop(lwk_wollet);

        let create_response = client.post_swap_req(&CreateSubmarineRequest {
            from: "L-BTC".to_string(),
            to: "BTC".to_string(),
            invoice: req.invoice.to_string(),
            // TODO: Add refund flow
            refund_public_key,
            // TODO: Add referral id
            referral_id: None,
        })?;

        // let swap_script = LBtcSwapScriptV2::submarine_from_swap_resp(&create_response, our_pubkey)?;
        debug!("Opening WS connection for swap {}", create_response.id);

        let mut socket = client.connect_ws().unwrap();
        let subscription = Subscription::new(&create_response.id);
        let subscribe_json = serde_json::to_string(&subscription)
            .map_err(|e| anyhow!("Failed to serialize subscription msg: {e:?}"))?;
        socket
            .send(tungstenite::Message::Text(subscribe_json))
            .map_err(|e| anyhow!("Failed to subscribe to websocket updates: {e:?}"))?;

        self.persister
            .insert_or_update_ongoing_swap(&[OngoingSwap::Send {
                id: create_response.id.clone(),
                invoice: req.invoice.clone(),
                payer_amount_sat: req.fees_sat + receiver_amount_sat,
                txid: None,
            }])?;

        let result;
        loop {
            let data = match get_swap_status_v2(&mut socket, &create_response.id) {
                Ok(data) => data,
                Err(_) => continue,
            };

            let state = data
                .parse::<SubSwapStates>()
                .map_err(|_| PaymentError::Generic {
                    err: "Invalid state received from swapper".to_string(),
                })?;

            match state {
                SubSwapStates::TransactionMempool | SubSwapStates::TransactionConfirmed => {
                    // Send detected by Boltz, waiting for invoice
                    // to be settled
                }
                SubSwapStates::InvoiceSet => {
                    debug!(
                        "Send {} sats to BTC address {}",
                        create_response.expected_amount, create_response.address
                    );
                    // let absolute_fees = self
                    //     .build_tx(
                    //         None,
                    //         &create_response.address,
                    //         create_response.expected_amount,
                    //     )?
                    //     .all_fees()
                    //     .values()
                    //     .sum::<u64>();
                    // let fee_rate =
                    //     req.fees_sat as f32 / absolute_fees as f32 * LIQUID_CLAIM_TX_FEERATE;
                    let tx = self.build_tx(
                        None,
                        &create_response.address,
                        create_response.expected_amount,
                    )?;

                    let txid = match self.network {
                        Network::Liquid => {
                            let tx_hex = elements::encode::serialize(&tx).to_lower_hex_string();
                            let response = client.broadcast_tx(self.network.into(), &tx_hex)?;
                            response
                                .as_object()
                                .ok_or(PaymentError::Generic {
                                    err: "Invalid data received from swapper".to_string(),
                                })?
                                .get("id")
                                .ok_or(PaymentError::Generic {
                                    err: "Invalid data received from swapper".to_string(),
                                })?
                                .as_str()
                                .ok_or(PaymentError::Generic {
                                    err: "Invalid data received from swapper".to_string(),
                                })?
                                .to_string()
                        }
                        Network::LiquidTestnet => {
                            let electrum_client = ElectrumClient::new(&self.electrum_url)?;
                            electrum_client.broadcast(&tx)?.to_string()
                        }
                    };

                    self.persister
                        .insert_or_update_ongoing_swap(&[OngoingSwap::Send {
                            id: create_response.id.clone(),
                            invoice: req.invoice.clone(),
                            payer_amount_sat: req.fees_sat + receiver_amount_sat,
                            txid: Some(txid.clone()),
                        }])?;

                    result = Ok(SendPaymentResponse { txid });
                    break;
                }
                SubSwapStates::TransactionClaimed
                | SubSwapStates::InvoiceFailedToPay
                | SubSwapStates::SwapExpired => {
                    result = Err(PaymentError::Generic {
                        err: format!("Payment state is unrecoverable: {}", state.to_string()),
                    });
                    break;
                }
                _ => info!(
                    "New state for swap {}: {}",
                    create_response.id,
                    state.to_string()
                ),
            };

            thread::sleep(Duration::from_millis(500));
        }

        socket.close(None).unwrap();
        result
    }

    fn try_claim(
        &self,
        preimage: &str,
        redeem_script: &str,
        blinding_key: &str,
        claim_fees_sat: u64,
    ) -> Result<String, PaymentError> {
        let network_config = &self.network_config();
        let rev_swap_tx = LBtcSwapTx::new_claim(
            LBtcSwapScript::reverse_from_str(redeem_script, blinding_key)?,
            self.address()?.to_string(),
            network_config,
        )?;

        let mnemonic = self
            .lwk_signer
            .mnemonic()
            .ok_or(PaymentError::SignerError {
                err: "Could not claim: Mnemonic not found".to_string(),
            })?;
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.network.into(), 0)?;

        let lsk = LiquidSwapKey::try_from(swap_key)?;
        let preimage = Preimage::from_str(preimage)?;

        let signed_tx = rev_swap_tx.sign_claim(&lsk.keypair, &preimage, claim_fees_sat)?;
        let tx_hex = elements::encode::serialize(&signed_tx).to_lower_hex_string();

        let client = self.boltz_client_v2();
        let response = client.broadcast_tx(self.network.into(), &tx_hex)?;
        let txid = response
            .as_object()
            .ok_or(PaymentError::Generic {
                err: "Invalid data received from swapper".to_string(),
            })?
            .get("id")
            .ok_or(PaymentError::Generic {
                err: "Invalid data received from swapper".to_string(),
            })?
            .as_str()
            .ok_or(PaymentError::Generic {
                err: "Invalid data received from swapper".to_string(),
            })?
            .to_string();

        Ok(txid)
    }

    #[allow(dead_code)]
    fn validate_reverse_pairs(
        client: &BoltzApiClientV2,
        payer_amount_sat: u64,
    ) -> Result<ReversePair, PaymentError> {
        let lbtc_pair = client
            .get_reverse_pairs()?
            .get_btc_to_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        lbtc_pair.limits.within(payer_amount_sat)?;

        let fees_sat = lbtc_pair.fees.total(payer_amount_sat);

        ensure_sdk!(payer_amount_sat > fees_sat, PaymentError::AmountOutOfRange);

        Ok(lbtc_pair)
    }

    pub fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        // let client = self.boltz_client_v2();
        // let lbtc_pair = Self::validate_reverse_pairs(&client, req.payer_amount_sat)?;

        let client = self.boltz_client();
        let lbtc_pair = client
            .get_pairs()?
            .get_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        let fees_sat = lbtc_pair.fees.reverse_total(req.payer_amount_sat);

        Ok(PrepareReceiveResponse {
            payer_amount_sat: req.payer_amount_sat,
            fees_sat,
        })
    }

    pub fn receive_payment(
        &self,
        req: &PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        // let client = self.boltz_client_v2();
        // let lbtc_pair = Self::validate_reverse_pairs(&client, req.payer_amount_sat)?;

        let client = self.boltz_client();
        let lbtc_pair = client
            .get_pairs()?
            .get_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        ensure_sdk!(
            req.fees_sat == lbtc_pair.fees.reverse_total(req.payer_amount_sat),
            PaymentError::InvalidOrExpiredFees
        );

        let client = self.boltz_client();
        let mnemonic = self
            .lwk_signer
            .mnemonic()
            .ok_or(PaymentError::SignerError {
                err: "Could not start receive: Mnemonic not found".to_string(),
            })?;
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.network.into(), 0)?;
        let lsk = LiquidSwapKey::try_from(swap_key)?;

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;
        let preimage_hash = preimage.sha256.to_string();

        debug!(
            "Creating reverse swap with: payer_amount_sat {} sat, fees_total {} sat",
            req.payer_amount_sat, req.fees_sat
        );
        let swap_response = client.create_swap(CreateSwapRequest::new_lbtc_reverse_invoice_amt(
            lbtc_pair.hash.clone(),
            preimage_hash.clone(),
            lsk.keypair.public_key().to_string(),
            req.payer_amount_sat,
        ))?;

        let swap_id = swap_response.get_id();
        let invoice = swap_response.get_invoice()?;
        let blinding_str = swap_response.get_blinding_key()?;
        let redeem_script = swap_response.get_redeem_script()?;
        let payer_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::InvalidInvoice)?
            / 1000;

        // Double check that the generated invoice includes our data
        // https://docs.boltz.exchange/v/api/dont-trust-verify#lightning-invoice-verification
        if invoice.payment_hash().to_string() != preimage_hash {
            return Err(PaymentError::InvalidInvoice);
        };

        self.persister
            .insert_or_update_ongoing_swap(&[OngoingSwap::Receive {
                id: swap_id.clone(),
                preimage: preimage_str,
                blinding_key: blinding_str,
                redeem_script,
                invoice: invoice.to_string(),
                receiver_amount_sat: payer_amount_sat - req.fees_sat,
                claim_fees_sat: lbtc_pair.fees.reverse_claim_estimate(),
            }])
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

        let transactions = self.lwk_wollet.lock().unwrap().transactions()?;

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
                        .map(|d| (amount_sat.abs() - d.payer_amount_sat as i64).unsigned_abs()),
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
        let network_config = self.network_config();
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

    /// Empties all Liquid Wallet caches for this network type.
    pub fn empty_wallet_cache(&self) -> Result<()> {
        let mut path = PathBuf::from(self.data_dir_path.clone());
        path.push(Into::<ElementsNetwork>::into(self.network).as_str());
        path.push("enc_cache");

        fs::remove_dir_all(&path)?;
        fs::create_dir_all(path)?;

        Ok(())
    }

    pub fn restore(&self, req: RestoreRequest) -> Result<()> {
        let backup_path = match req.backup_path {
            Some(p) => PathBuf::from_str(&p)?,
            None => self.persister.get_backup_path(),
        };
        self.persister.restore_from_backup(backup_path)
    }

    pub fn backup(&self) -> Result<()> {
        self.persister.backup()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tempdir::TempDir;

    use crate::model::*;
    use crate::sdk::{LiquidSdk, Network};

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn create_temp_dir() -> Result<(TempDir, String)> {
        let data_dir = TempDir::new(&uuid::Uuid::new_v4().to_string())?;
        let data_dir_str = data_dir
            .as_ref()
            .to_path_buf()
            .to_str()
            .expect("Expecting valid temporary path")
            .to_owned();
        Ok((data_dir, data_dir_str))
    }

    fn list_pending(sdk: &LiquidSdk) -> Result<Vec<Payment>> {
        let payments = sdk.list_payments(true, true)?;

        Ok(payments
            .iter()
            .filter(|p| {
                [PaymentType::PendingSend, PaymentType::PendingReceive].contains(&p.payment_type)
            })
            .cloned()
            .collect())
    }

    #[test]
    fn normal_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let sdk = LiquidSdk::connect(ConnectRequest {
            mnemonic: TEST_MNEMONIC.to_string(),
            data_dir: Some(data_dir_str),
            network: Network::LiquidTestnet,
        })?;

        let invoice = "lntb10u1pnqwkjrpp5j8ucv9mgww0ajk95yfpvuq0gg5825s207clrzl5thvtuzfn68h0sdqqcqzzsxqr23srzjqv8clnrfs9keq3zlg589jvzpw87cqh6rjks0f9g2t9tvuvcqgcl45f6pqqqqqfcqqyqqqqlgqqqqqqgq2qsp5jnuprlxrargr6hgnnahl28nvutj3gkmxmmssu8ztfhmmey3gq2ss9qyyssq9ejvcp6frwklf73xvskzdcuhnnw8dmxag6v44pffwqrxznsly4nqedem3p3zhn6u4ln7k79vk6zv55jjljhnac4gnvr677fyhfgn07qp4x6wrq".to_string();
        sdk.prepare_send_payment(&PrepareSendRequest { invoice })?;
        assert!(!list_pending(&sdk)?.is_empty());

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let sdk = LiquidSdk::connect(ConnectRequest {
            mnemonic: TEST_MNEMONIC.to_string(),
            data_dir: Some(data_dir_str),
            network: Network::LiquidTestnet,
        })?;

        let prepare_response = sdk.prepare_receive_payment(&PrepareReceiveRequest {
            payer_amount_sat: 1_000,
        })?;
        sdk.receive_payment(&prepare_response)?;
        assert!(!list_pending(&sdk)?.is_empty());

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap_recovery() -> Result<()> {
        Ok(())
    }
}
