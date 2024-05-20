use std::{
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use anyhow::{anyhow, Result};
use boltz_client::{
    network::electrum::ElectrumConfig,
    swaps::{
        boltz::{RevSwapStates, SubSwapStates},
        boltzv2::*,
        liquidv2::LBtcSwapTxV2,
    },
    util::secrets::{LiquidSwapKey, Preimage, SwapKey},
    Amount, Bolt11Invoice, ElementsAddress, Keypair, LBtcSwapScriptV2, SwapType,
};
use log::{debug, error, info, warn};
use lwk_common::{singlesig_desc, Signer, Singlesig};
use lwk_signer::{AnySigner, SwSigner};
use lwk_wollet::{
    elements::{Address, Transaction},
    full_scan_with_electrum_client, BlockchainBackend, ElectrumClient, ElectrumUrl,
    ElementsNetwork, FsPersister, Wollet as LwkWollet, WolletDescriptor,
};

use crate::{
    boltz_status_stream::BoltzStatusStream, ensure_sdk, error::PaymentError, get_invoice_amount,
    model::*, persist::Persister, utils,
};

/// Claim tx feerate, in sats per vbyte.
/// Since the  Liquid blocks are consistently empty for now, we hardcode the minimum feerate.
pub const LIQUID_CLAIM_TX_FEERATE_MSAT: f32 = 100.0;

pub const DEFAULT_DATA_DIR: &str = ".data";

pub struct LiquidSdk {
    electrum_url: ElectrumUrl,
    network: Network,
    /// LWK Wollet, a watch-only Liquid wallet for this instance
    lwk_wollet: Arc<Mutex<LwkWollet>>,
    /// LWK Signer, for signing Liquid transactions
    lwk_signer: SwSigner,
    persister: Persister,
    status_stream: BoltzStatusStream,
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

        let status_stream = BoltzStatusStream::new();

        let sdk = Arc::new(LiquidSdk {
            lwk_wollet,
            network,
            electrum_url,
            lwk_signer: opts.signer,
            persister,
            data_dir_path,
            status_stream,
        });

        sdk.status_stream.track_pending_swaps(sdk.clone())?;

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

    fn get_submarine_keys(&self, derivation_index: i32) -> Result<Keypair, PaymentError> {
        let mnemonic = self
            .lwk_signer
            .mnemonic()
            .ok_or(PaymentError::SignerError {
                err: "Could not claim: Mnemonic not found".to_string(),
            })?;
        let swap_key = SwapKey::from_submarine_account(
            &mnemonic.to_string(),
            "",
            self.network.into(),
            derivation_index as u64,
        )?;
        let lsk = LiquidSwapKey::try_from(swap_key)?;
        Ok(lsk.keypair)
    }

    pub(crate) fn try_handle_reverse_swap_status(
        &self,
        swap_state: RevSwapStates,
        id: &str,
    ) -> Result<()> {
        info!("Handling reverse swap transition to {swap_state:?} for swap {id}");

        let con = self.persister.get_connection()?;
        let swap_out = Persister::fetch_swap_out(&con, id)?
            .ok_or(anyhow!("No ongoing swap out found for ID {id}"))?;

        match swap_state {
            RevSwapStates::SwapExpired
            | RevSwapStates::InvoiceExpired
            | RevSwapStates::TransactionFailed
            | RevSwapStates::TransactionRefunded => {
                error!("Cannot claim swap {id}, unrecoverable state: {swap_state:?}");
                // TODO Update swap state, not payment state
            }
            // The lockup tx is in the mempool and we accept 0-conf => try to claim
            // TODO Add 0-conf preconditions check: https://github.com/breez/breez-liquid-sdk/issues/187
            RevSwapStates::TransactionMempool
            // The lockup tx is confirmed => try to claim
            | RevSwapStates::TransactionConfirmed => {
                match swap_out.claim_txid {
                    Some(claim_txid) => {
                        warn!("Claim tx for reverse swap {id} was already broadcast: txid {claim_txid}")
                    }
                    None => match self.try_claim(&swap_out) {
                        Ok(txid) => {
                            // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
                            // This makes the tx known to the SDK (get_info, list_payments) instantly
                            self.persister
                                .insert_or_update_payment(
                                    PaymentTxData {
                                        txid,
                                        timestamp: None,
                                        amount_sat: swap_out.receiver_amount_sat,
                                        payment_type: PaymentType::Receive,
                                        status: PaymentStatus::Pending,
                                    }
                                )?;
                        }
                        Err(err) => {
                            if let PaymentError::AlreadyClaimed = err {
                                warn!("Funds already claimed");
                            }
                            error!("Claim reverse swap {id} failed: {err}");
                        }
                    },
                }
            }
            // Too soon to try to claim
            RevSwapStates::Created | RevSwapStates::MinerFeePaid => {}
            // Swap completed successfully (HODL invoice settled), the claim already happened
            RevSwapStates::InvoiceSettled => {}
        }

        Ok(())
    }

    pub(crate) fn try_handle_submarine_swap_status(
        &self,
        swap_state: SubSwapStates,
        id: &str,
    ) -> Result<()> {
        info!("Handling submarine swap transition to {swap_state:?} for swap {id}");

        let con = self.persister.get_connection()?;
        let ongoing_swap_in = Persister::fetch_swap_in(&con, id)?
            .ok_or(anyhow!("No ongoing swap in found for ID {id}"))?;

        let Some(txid) = ongoing_swap_in.lockup_txid.clone() else {
            return Err(anyhow!("Swap-in {id} is pending but no txid is present"));
        };
        let receiver_amount_sat = get_invoice_amount!(ongoing_swap_in.invoice);
        let keypair = self.get_submarine_keys(0)?;
        let create_response: CreateSubmarineResponse =
            serde_json::from_str(&ongoing_swap_in.create_response_json)?;

        match swap_state {
            SubSwapStates::TransactionClaimPending => {
                let swap_script = LBtcSwapScriptV2::submarine_from_swap_resp(
                    &create_response,
                    keypair.public_key().into(),
                )
                .map_err(|e| anyhow!("Could not rebuild refund details for swap-in {id}: {e:?}"))?;

                self.post_submarine_claim_details(
                    id,
                    &swap_script,
                    &ongoing_swap_in.invoice,
                    &keypair,
                )
                .map_err(|e| anyhow!("Could not post claim details. Err: {e:?}"))?;

                // We insert a pseudo-lockup-tx in case LWK fails to pick up the new mempool tx for a while
                // This makes the tx known to the SDK (get_info, list_payments) instantly
                self.persister.insert_or_update_payment(PaymentTxData {
                    txid,
                    timestamp: None,
                    amount_sat: ongoing_swap_in.payer_amount_sat,
                    payment_type: PaymentType::Send,
                    status: PaymentStatus::Pending,
                })?;

                Ok(())
            }
            SubSwapStates::TransactionClaimed => {
                warn!("Swap-in {id} has already been claimed");
                Ok(())
            }
            SubSwapStates::TransactionLockupFailed
            | SubSwapStates::InvoiceFailedToPay
            | SubSwapStates::SwapExpired => {
                warn!("Swap-in {id} is in an unrecoverable state: {swap_state:?}");

                // If swap state is unrecoverable, try refunding
                let swap_script = LBtcSwapScriptV2::submarine_from_swap_resp(
                    &create_response,
                    keypair.public_key().into(),
                )
                .map_err(|e| anyhow!("Could not rebuild refund details for swap-in {id}: {e:?}"))?;

                let refund_txid =
                    self.try_refund(id, &swap_script, &keypair, receiver_amount_sat)?;
                info!("Swap-in {id} refunded successfully. Txid: {refund_txid}");

                // TODO Update swap state: persist refund txid
                Ok(())
            }
            _ => Err(anyhow!("New state for submarine swap {id}: {swap_state:?}")),
        }
    }

    pub(crate) fn list_ongoing_swaps(&self) -> Result<Vec<Swap>> {
        self.persister.list_ongoing_swaps()
    }

    fn scan(&self) -> Result<(), lwk_wollet::Error> {
        let mut electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let mut lwk_wollet = self.lwk_wollet.lock().unwrap();
        full_scan_with_electrum_client(&mut lwk_wollet, &mut electrum_client)
    }

    /// Gets the next unused onchain Liquid address
    fn next_unused_address(&self) -> Result<Address, lwk_wollet::Error> {
        let lwk_wollet = self.lwk_wollet.lock().unwrap();
        Ok(lwk_wollet.address(None)?.address().clone())
    }

    pub fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse> {
        debug!("next_unused_address: {}", self.next_unused_address()?);

        let mut pending_send_sat = 0;
        let mut pending_receive_sat = 0;
        let mut confirmed_sent_sat = 0;
        let mut confirmed_received_sat = 0;

        for p in self.list_payments(req.with_scan, true)? {
            match p.tx.payment_type {
                PaymentType::Send => match p.status {
                    PaymentStatus::Pending => pending_send_sat += p.tx.amount_sat,
                    PaymentStatus::Complete => confirmed_sent_sat += p.tx.amount_sat,
                },
                PaymentType::Receive => match p.status {
                    PaymentStatus::Pending => pending_receive_sat += p.tx.amount_sat,
                    PaymentStatus::Complete => confirmed_received_sat += p.tx.amount_sat,
                },
            }
        }

        Ok(GetInfoResponse {
            balance_sat: confirmed_received_sat - confirmed_sent_sat - pending_send_sat,
            pending_send_sat,
            pending_receive_sat,
            pubkey: self.lwk_signer.xpub().public_key.to_string(),
        })
    }

    pub(crate) fn boltz_client_v2(&self) -> BoltzApiClientV2 {
        BoltzApiClientV2::new(self.boltz_url_v2())
    }

    pub(crate) fn boltz_url_v2(&self) -> &str {
        match self.network {
            Network::LiquidTestnet => BOLTZ_TESTNET_URL_V2,
            Network::Liquid => BOLTZ_MAINNET_URL_V2,
        }
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
        self.scan()?;
        let lwk_wollet = self.lwk_wollet.lock().unwrap();
        let mut pset = lwk_wollet::TxBuilder::new(self.network.into())
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

    fn get_broadcast_fee_estimation(&self, amount_sat: u64) -> Result<u64> {
        // TODO Replace this with own address when LWK supports taproot
        //  https://github.com/Blockstream/lwk/issues/31
        let temp_p2tr_addr = match self.network {
            Network::Liquid => "lq1pqvzxvqhrf54dd4sny4cag7497pe38252qefk46t92frs7us8r80ja9ha8r5me09nn22m4tmdqp5p4wafq3s59cql3v9n45t5trwtxrmxfsyxjnstkctj",
            Network::LiquidTestnet => "tlq1pq0wqu32e2xacxeyps22x8gjre4qk3u6r70pj4r62hzczxeyz8x3yxucrpn79zy28plc4x37aaf33kwt6dz2nn6gtkya6h02mwpzy4eh69zzexq7cf5y5"
        };

        // Create a throw-away tx similar to the lockup tx, in order to estimate fees
        Ok(self
            .build_tx(None, temp_p2tr_addr, amount_sat)?
            .all_fees()
            .values()
            .sum())
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

        let broadcast_fees_sat = self.get_broadcast_fee_estimation(receiver_amount_sat)?;

        Ok(PrepareSendResponse {
            invoice: req.invoice.clone(),
            fees_sat: lbtc_pair.fees.total(receiver_amount_sat) + broadcast_fees_sat,
        })
    }

    fn verify_payment_hash(preimage: &str, invoice: &str) -> Result<(), PaymentError> {
        let preimage = Preimage::from_str(preimage)?;
        let preimage_hash = preimage.sha256.to_string();
        let invoice = Bolt11Invoice::from_str(invoice).map_err(|_| PaymentError::InvalidInvoice)?;
        let invoice_payment_hash = invoice.payment_hash();

        (invoice_payment_hash.to_string() == preimage_hash)
            .then_some(())
            .ok_or(PaymentError::InvalidPreimage)
    }

    fn new_refund_tx(&self, swap_script: &LBtcSwapScriptV2) -> Result<LBtcSwapTxV2, PaymentError> {
        let wallet = self.lwk_wollet.lock().unwrap();
        let output_address = wallet.address(Some(0))?.address().to_string();
        let network_config = self.network_config();
        Ok(LBtcSwapTxV2::new_refund(
            swap_script.clone(),
            &output_address,
            &network_config,
        )?)
    }

    fn try_refund(
        &self,
        swap_id: &str,
        swap_script: &LBtcSwapScriptV2,
        keypair: &Keypair,
        amount_sat: u64,
    ) -> Result<String, PaymentError> {
        let refund_tx = self.new_refund_tx(swap_script)?;

        let broadcast_fees_sat = Amount::from_sat(self.get_broadcast_fee_estimation(amount_sat)?);
        let client = self.boltz_client_v2();
        let is_lowball = Some((&client, boltz_client::network::Chain::from(self.network)));

        match refund_tx.sign_refund(
            keypair,
            broadcast_fees_sat,
            Some((&client, &swap_id.to_string())),
        ) {
            // Try with cooperative refund
            Ok(tx) => {
                let txid = refund_tx.broadcast(&tx, &self.network_config(), is_lowball)?;
                debug!("Successfully broadcast cooperative refund for swap-in {swap_id}");
                Ok(txid)
            }
            // Try with non-cooperative refund
            Err(e) => {
                debug!("Cooperative refund failed: {:?}", e);
                let tx = refund_tx.sign_refund(keypair, broadcast_fees_sat, None)?;
                let txid = refund_tx.broadcast(&tx, &self.network_config(), is_lowball)?;
                debug!("Successfully broadcast non-cooperative refund for swap-in {swap_id}");
                Ok(txid)
            }
        }
    }

    fn post_submarine_claim_details(
        &self,
        swap_id: &str,
        swap_script: &LBtcSwapScriptV2,
        invoice: &str,
        keypair: &Keypair,
    ) -> Result<(), PaymentError> {
        debug!("Claim is pending for swap-in {swap_id}. Initiating cooperative claim");
        let client = self.boltz_client_v2();
        let refund_tx = self.new_refund_tx(swap_script)?;

        let claim_tx_response = client.get_claim_tx_details(&swap_id.to_string())?;

        debug!("Received claim tx details: {:?}", &claim_tx_response);

        Self::verify_payment_hash(&claim_tx_response.preimage, invoice)?;

        let (partial_sig, pub_nonce) =
            refund_tx.submarine_partial_sig(keypair, &claim_tx_response)?;

        client.post_claim_tx_details(&swap_id.to_string(), pub_nonce, partial_sig)?;
        debug!("Successfully sent claim details for swap-in {swap_id}");
        Ok(())
    }

    fn lockup_funds(
        &self,
        swap_id: &str,
        create_response: &CreateSubmarineResponse,
    ) -> Result<String, PaymentError> {
        debug!(
            "Initiated swap-in: send {} sats to liquid address {}",
            create_response.expected_amount, create_response.address
        );

        let lockup_tx = self.build_tx(
            None,
            &create_response.address,
            create_response.expected_amount,
        )?;

        let electrum_client = ElectrumClient::new(&self.electrum_url)?;
        let lockup_txid = electrum_client.broadcast(&lockup_tx)?.to_string();

        debug!(
            "Successfully broadcast lockup transaction for swap-in {swap_id}. Txid: {lockup_txid}"
        );
        Ok(lockup_txid)
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

        let broadcast_fees_sat = self.get_broadcast_fee_estimation(receiver_amount_sat)?;

        ensure_sdk!(
            req.fees_sat == lbtc_pair.fees.total(receiver_amount_sat) + broadcast_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let keypair = self.get_submarine_keys(0)?;
        let refund_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: keypair.public_key(),
        };

        let create_response = client.post_swap_req(&CreateSubmarineRequest {
            from: "L-BTC".to_string(),
            to: "BTC".to_string(),
            invoice: req.invoice.to_string(),
            refund_public_key,
            pair_hash: Some(lbtc_pair.hash),
            // TODO: Add referral id
            referral_id: None,
        })?;
        let create_response_json =
            serde_json::to_string(&create_response).map_err(|e| PaymentError::Generic {
                err: format!("Failed to deserialize CreateReverseResponse: {e:?}"),
            })?;

        let swap_id = &create_response.id;
        let swap_script = LBtcSwapScriptV2::submarine_from_swap_resp(
            &create_response,
            keypair.public_key().into(),
        )?;

        debug!("Opening WS connection for swap {}", &swap_id);

        let mut socket = client.connect_ws()?;
        let subscription = Subscription::new(swap_id);
        let subscribe_json = serde_json::to_string(&subscription)
            .map_err(|e| anyhow!("Failed to serialize subscription msg: {e:?}"))?;
        socket
            .send(tungstenite::Message::Text(subscribe_json))
            .map_err(|e| anyhow!("Failed to subscribe to websocket updates: {e:?}"))?;

        // We insert the pending send to avoid it being handled by the status stream
        self.status_stream
            .insert_tracked_swap(swap_id, SwapType::Submarine);

        self.persister.insert_or_update_swap_in(SwapIn {
            id: swap_id.clone(),
            invoice: req.invoice.clone(),
            payer_amount_sat: req.fees_sat + receiver_amount_sat,
            receiver_amount_sat,
            create_response_json,
            lockup_txid: None,
            is_claim_tx_seen: false,
            created_at: utils::now(),
        })?;

        let result;
        let mut lockup_txid = String::new();
        loop {
            let data = match utils::get_swap_status_v2(&mut socket, swap_id) {
                Ok(data) => data,
                Err(_) => {
                    // TODO: close socket if dead, skip EOF errors
                    continue;
                }
            };

            let state = data
                .parse::<SubSwapStates>()
                .map_err(|_| PaymentError::Generic {
                    err: "Invalid state received from swapper".to_string(),
                })?;

            // See https://docs.boltz.exchange/v/api/lifecycle#normal-submarine-swaps
            match state {
                // Boltz has locked the HTLC, we proceed with locking up the funds
                SubSwapStates::InvoiceSet => {
                    // Check that we have not persisted the swap already
                    let con = self.persister.get_connection()?;

                    if let Some(ongoing_swap) = Persister::fetch_swap_in(&con, swap_id)
                        .map_err(|_| PaymentError::PersistError)?
                    {
                        if ongoing_swap.lockup_txid.is_some() {
                            continue;
                        }
                    };

                    lockup_txid = self.lockup_funds(swap_id, &create_response)?;
                    self.persister
                        .set_lockup_txid_for_swap_in(swap_id, &lockup_txid)?;
                }

                // Boltz has detected the lockup in the mempool, we can speed up
                // the claim by doing so cooperatively
                SubSwapStates::TransactionClaimPending => {
                    // TODO Consolidate try_handle

                    self.post_submarine_claim_details(
                        swap_id,
                        &swap_script,
                        &req.invoice,
                        &keypair,
                    )?;
                    debug!("Boltz successfully claimed the funds");
                    self.persister.set_claim_tx_seen_for_swap_in(swap_id)?;

                    self.status_stream
                        .resolve_tracked_swap(swap_id, SwapType::ReverseSubmarine);

                    result = Ok(SendPaymentResponse { txid: lockup_txid });

                    debug!("Successfully resolved swap-in {swap_id}");
                    break;
                }

                // Either:
                // 1. Boltz failed to pay
                // 2. The swap has expired (>24h)
                // 3. Lockup failed (we sent too little funds)
                // We initiate a cooperative refund, and then fallback to a regular one
                SubSwapStates::InvoiceFailedToPay
                | SubSwapStates::SwapExpired
                | SubSwapStates::TransactionLockupFailed => {
                    let refund_txid =
                        self.try_refund(swap_id, &swap_script, &keypair, receiver_amount_sat)?;

                    result = Err(PaymentError::Refunded {
                        err: format!(
                            "Unrecoverable state for swap-in {swap_id}: {}",
                            state.to_string()
                        ),
                        txid: refund_txid.clone(),
                    });
                    break;
                }
                _ => {}
            };

            sleep(Duration::from_millis(500));
        }

        socket.close(None).unwrap();
        result
    }

    fn try_claim(&self, ongoing_swap_out: &SwapOut) -> Result<String, PaymentError> {
        ensure_sdk!(
            ongoing_swap_out.claim_txid.is_none(),
            PaymentError::AlreadyClaimed
        );

        let rev_swap_id = &ongoing_swap_out.id;
        debug!("Trying to claim reverse swap {rev_swap_id}",);

        let lsk = self.get_liquid_swap_key()?;
        let our_keys = lsk.keypair;

        let create_response: CreateReverseResponse =
            serde_json::from_str(&ongoing_swap_out.redeem_script).map_err(|e| {
                PaymentError::Generic {
                    err: format!("Failed to deserialize CreateReverseResponse: {e:?}"),
                }
            })?;
        let swap_script = LBtcSwapScriptV2::reverse_from_swap_resp(
            &create_response,
            our_keys.public_key().into(),
        )?;

        let claim_address = self.next_unused_address()?.to_string();
        let claim_tx_wrapper = LBtcSwapTxV2::new_claim(
            swap_script,
            claim_address,
            &self.network_config(),
            self.boltz_url_v2().into(),
            ongoing_swap_out.id.clone(),
        )?;

        let claim_tx = claim_tx_wrapper.sign_claim(
            &our_keys,
            &Preimage::from_str(&ongoing_swap_out.preimage)?,
            Amount::from_sat(ongoing_swap_out.claim_fees_sat),
            // Enable cooperative claim (Some) or not (None)
            Some((&self.boltz_client_v2(), rev_swap_id.clone())),
            // None
        )?;

        let claim_txid = claim_tx_wrapper.broadcast(
            &claim_tx,
            &self.network_config(),
            Some((&self.boltz_client_v2(), self.network.into())),
        )?;

        // Once successfully broadcasted, we persist the claim txid
        self.persister
            .set_claim_txid_for_swap_out(rev_swap_id, &claim_txid)?;

        info!("Succesfully broadcasted claim tx {claim_txid} for rev swap {rev_swap_id}");
        debug!("Claim Tx {:?}", claim_tx);

        Ok(claim_txid)
    }

    pub fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        let reverse_pair = self
            .boltz_client_v2()
            .get_reverse_pairs()?
            .get_btc_to_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;

        let payer_amount_sat = req.payer_amount_sat;
        let fees_sat = reverse_pair.fees.total(req.payer_amount_sat);

        ensure_sdk!(payer_amount_sat > fees_sat, PaymentError::AmountOutOfRange);

        reverse_pair
            .limits
            .within(payer_amount_sat)
            .map_err(|_| PaymentError::AmountOutOfRange)?;

        debug!("Preparing reverse swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat");

        Ok(PrepareReceiveResponse {
            payer_amount_sat,
            fees_sat,
        })
    }

    pub fn receive_payment(
        &self,
        req: &PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        let payer_amount_sat = req.payer_amount_sat;
        let fees_sat = req.fees_sat;

        let reverse_pair = self
            .boltz_client_v2()
            .get_reverse_pairs()?
            .get_btc_to_lbtc_pair()
            .ok_or(PaymentError::PairsNotFound)?;
        let new_fees_sat = reverse_pair.fees.total(req.payer_amount_sat);
        ensure_sdk!(fees_sat == new_fees_sat, PaymentError::InvalidOrExpiredFees);

        debug!("Creating reverse swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat");

        let lsk = self.get_liquid_swap_key()?;

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;
        let preimage_hash = preimage.sha256.to_string();

        let v2_req = CreateReverseRequest {
            invoice_amount: req.payer_amount_sat as u32, // TODO update our model
            from: "BTC".to_string(),
            to: "L-BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: lsk.keypair.public_key().into(),
            address: None,
            address_signature: None,
            referral_id: None,
        };
        let create_response = self.boltz_client_v2().post_reverse_req(v2_req)?;

        // TODO Persisting this in the DB (reusing "redeem_script" field), as we need it later when claiming
        let redeem_script = serde_json::to_string(&create_response).unwrap();

        let swap_id = create_response.id;
        let invoice = Bolt11Invoice::from_str(&create_response.invoice)
            .map_err(|_| PaymentError::InvalidInvoice)?;
        let blinding_str =
            create_response
                .blinding_key
                .ok_or(boltz_client::error::Error::Protocol(
                    "Boltz response does not contain a blinding key.".to_string(),
                ))?;
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
            .insert_or_update_swap_out(SwapOut {
                id: swap_id.clone(),
                preimage: preimage_str,
                blinding_key: blinding_str,
                redeem_script,
                invoice: invoice.to_string(),
                payer_amount_sat,
                receiver_amount_sat: payer_amount_sat - req.fees_sat,
                claim_fees_sat: reverse_pair.fees.claim_estimate(),
                claim_txid: None,
                created_at: utils::now(),
                is_claim_tx_confirmed: false,
            })
            .map_err(|_| PaymentError::PersistError)?;

        Ok(ReceivePaymentResponse {
            id: swap_id,
            invoice: invoice.to_string(),
        })
    }

    // TODO When to best do this? On every list_payments / get_info? In background thread?
    /// This method fetches the chain tx data (onchain and mempool) using LWK. For every wallet tx,
    /// it inserts or updates a corresponding entry in our Payments table.
    fn sync_payments_with_chain_data(&self, with_scan: bool) -> Result<()> {
        if with_scan {
            self.scan()?;
        }

        for tx in self.lwk_wollet.lock().unwrap().transactions()? {
            let txid = tx.txid.to_string();
            let is_tx_confirmed = tx.height.is_some();
            let amount_sat = tx.balance.values().sum::<i64>();

            self.persister.insert_or_update_payment(PaymentTxData {
                txid,
                timestamp: tx.timestamp,
                amount_sat: amount_sat.unsigned_abs(),
                payment_type: match amount_sat >= 0 {
                    true => PaymentType::Receive,
                    false => PaymentType::Send,
                },
                status: match is_tx_confirmed {
                    true => PaymentStatus::Complete,
                    false => PaymentStatus::Pending,
                },
            })?;
        }

        Ok(())
    }

    /// Lists the SDK payments. The payments are determined based on onchain transactions and swaps.
    ///
    /// ## Arguments
    ///
    /// - `with_scan`: Rescan the onchain wallet before listing the payments.
    /// - `include_pending`: Includes the payments that are still pending.
    pub fn list_payments(&self, with_scan: bool, _include_pending: bool) -> Result<Vec<Payment>> {
        // TODO Remove include_pending ?
        self.sync_payments_with_chain_data(with_scan)?;

        let mut payments: Vec<Payment> = self.persister.get_payments()?.values().cloned().collect();
        payments.sort_by_key(|p| p.timestamp);
        Ok(payments)
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

    fn get_liquid_swap_key(&self) -> Result<LiquidSwapKey, PaymentError> {
        let mnemonic = self
            .lwk_signer
            .mnemonic()
            .ok_or(PaymentError::SignerError {
                err: "Mnemonic not found".to_string(),
            })?;
        let swap_key =
            SwapKey::from_reverse_account(&mnemonic.to_string(), "", self.network.into(), 0)?;
        LiquidSwapKey::try_from(swap_key).map_err(|e| PaymentError::SignerError {
            err: format!("Could not create LiquidSwapKey: {e:?}"),
        })
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
            .filter(|p| matches!(&p.status, PaymentStatus::Pending))
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
