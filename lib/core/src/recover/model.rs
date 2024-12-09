use std::collections::HashMap;
use std::str::FromStr;

use anyhow::anyhow;
use boltz_client::ElementsAddress;
use lwk_wollet::elements::Txid;
use lwk_wollet::History;
use lwk_wollet::WalletTx;

use crate::prelude::*;

pub(crate) type BtcScript = lwk_wollet::bitcoin::ScriptBuf;
pub(crate) type LBtcScript = lwk_wollet::elements::Script;
pub(crate) type SendSwapHistory = Vec<HistoryTxId>;

#[derive(Clone)]
pub(crate) struct HistoryTxId {
    pub txid: Txid,
    /// Confirmation height of txid
    ///
    /// -1 means unconfirmed with unconfirmed parents
    ///  0 means unconfirmed with confirmed parents
    pub height: i32,
}
impl HistoryTxId {
    pub(crate) fn confirmed(&self) -> bool {
        self.height > 0
    }
}
impl From<History> for HistoryTxId {
    fn from(value: History) -> Self {
        Self::from(&value)
    }
}
impl From<&History> for HistoryTxId {
    fn from(value: &History) -> Self {
        Self {
            txid: value.txid,
            height: value.height,
        }
    }
}

/// A map of all our known LWK onchain txs, indexed by tx ID. Essentially our own cache of the LWK txs.
pub(crate) struct TxMap {
    pub(crate) outgoing_tx_map: HashMap<Txid, WalletTx>,
    pub(crate) incoming_tx_map: HashMap<Txid, WalletTx>,
}
impl TxMap {
    pub(crate) fn from_raw_tx_map(raw_tx_map: HashMap<Txid, WalletTx>) -> Self {
        let (outgoing_tx_map, incoming_tx_map): (HashMap<Txid, WalletTx>, HashMap<Txid, WalletTx>) =
            raw_tx_map
                .into_iter()
                .partition(|(_, tx)| tx.balance.values().sum::<i64>() < 0);

        Self {
            outgoing_tx_map,
            incoming_tx_map,
        }
    }
}

pub(crate) trait PartialSwapState {
    /// Determine partial swap state, based on recovered chain data.
    ///
    /// This is a partial state, which means it may be incomplete because it's based on partial
    /// information. Some swap states cannot be determined based only on chain data.
    /// In these cases we do not assume any swap state.
    fn derive_partial_state(&self) -> Option<PaymentState>;
}

pub(crate) struct RecoveredOnchainDataSend {
    pub(crate) lockup_tx_id: Option<HistoryTxId>,
    pub(crate) claim_tx_id: Option<HistoryTxId>,
    pub(crate) refund_tx_id: Option<HistoryTxId>,
}
impl PartialSwapState for RecoveredOnchainDataSend {
    fn derive_partial_state(&self) -> Option<PaymentState> {
        match &self.lockup_tx_id {
            Some(_) => match &self.claim_tx_id {
                Some(_) => Some(PaymentState::Complete),
                None => match &self.refund_tx_id {
                    Some(refund_tx_id) => match refund_tx_id.confirmed() {
                        true => Some(PaymentState::Failed),
                        false => Some(PaymentState::RefundPending),
                    },
                    None => Some(PaymentState::Pending),
                },
            },
            // We have no onchain data to support deriving the state as the swap could
            // potentially be Created, TimedOut or Failed after expiry. In this case we return None.
            None => None,
        }
    }
}

pub(crate) struct RecoveredOnchainDataReceive {
    pub(crate) lockup_tx_id: Option<HistoryTxId>,
    pub(crate) claim_tx_id: Option<HistoryTxId>,
    pub(crate) mrh_tx_id: Option<HistoryTxId>,
    pub(crate) mrh_amount_sat: Option<u64>, // TODO: Add mrh_amount recovery
}
impl PartialSwapState for RecoveredOnchainDataReceive {
    fn derive_partial_state(&self) -> Option<PaymentState> {
        match &self.lockup_tx_id {
            Some(_) => match &self.claim_tx_id {
                Some(claim_tx_id) => match claim_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                None => Some(PaymentState::Pending),
            },
            None => match &self.mrh_tx_id {
                Some(mrh_tx_id) => match mrh_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                // We have no onchain data to support deriving the state as the swap could
                // potentially be Created or Failed after expiry. In this case we return None.
                None => None,
            },
        }
    }
}

pub(crate) struct RecoveredOnchainDataChainSend {
    /// LBTC tx initiated by the SDK (the "user" as per Boltz), sending funds to the swap funding address.
    pub(crate) lbtc_user_lockup_tx_id: Option<HistoryTxId>,
    /// LBTC tx initiated by the SDK to itself, in case the initial funds have to be refunded.
    pub(crate) lbtc_refund_tx_id: Option<HistoryTxId>,
    /// BTC tx locking up funds by the swapper
    pub(crate) btc_server_lockup_tx_id: Option<HistoryTxId>,
    /// BTC tx that claims to the final BTC destination address. The final step in a successful swap.
    pub(crate) btc_claim_tx_id: Option<HistoryTxId>,
}
impl PartialSwapState for RecoveredOnchainDataChainSend {
    fn derive_partial_state(&self) -> Option<PaymentState> {
        match &self.lbtc_user_lockup_tx_id {
            Some(_) => match &self.btc_claim_tx_id {
                Some(btc_claim_tx_id) => match btc_claim_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                None => match &self.lbtc_refund_tx_id {
                    Some(tx) => match tx.confirmed() {
                        true => Some(PaymentState::Failed),
                        false => Some(PaymentState::RefundPending),
                    },
                    None => Some(PaymentState::Pending),
                },
            },
            // We have no onchain data to support deriving the state as the swap could
            // potentially be Created, TimedOut or Failed after expiry. In this case we return None.
            None => None,
        }
    }
}

pub(crate) struct RecoveredOnchainDataChainReceive {
    /// LBTC tx locking up funds by the swapper
    pub(crate) lbtc_server_lockup_tx_id: Option<HistoryTxId>,
    /// LBTC tx that claims to our wallet. The final step in a successful swap.
    pub(crate) lbtc_claim_tx_id: Option<HistoryTxId>,
    /// LBTC tx out address for the claim tx.
    pub(crate) lbtc_claim_address: Option<String>,
    /// BTC tx initiated by the payer (the "user" as per Boltz), sending funds to the swap funding address.
    pub(crate) btc_user_lockup_tx_id: Option<HistoryTxId>,
    /// BTC tx initiated by the SDK to a user-chosen address, in case the initial funds have to be refunded.
    pub(crate) btc_refund_tx_id: Option<HistoryTxId>,
}
impl PartialSwapState for RecoveredOnchainDataChainReceive {
    fn derive_partial_state(&self) -> Option<PaymentState> {
        match &self.btc_user_lockup_tx_id {
            Some(_) => match &self.lbtc_claim_tx_id {
                Some(lbtc_claim_tx_id) => match lbtc_claim_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                None => match &self.btc_refund_tx_id {
                    Some(tx) => match tx.confirmed() {
                        true => Some(PaymentState::Failed),
                        false => Some(PaymentState::RefundPending),
                    },
                    None => Some(PaymentState::Pending),
                },
            },
            // We have no onchain data to support deriving the state as the swap could
            // potentially be Created or Failed after expiry. In this case we return None.
            None => None,
        }
    }
}

#[derive(Clone)]
pub(crate) struct SendSwapImmutableData {
    pub(crate) swap_id: String,
    pub(crate) lockup_script: LBtcScript,
}

impl TryFrom<SendSwap> for SendSwapImmutableData {
    type Error = anyhow::Error;

    fn try_from(swap: SendSwap) -> std::result::Result<Self, Self::Error> {
        let swap_script = swap.get_swap_script()?;

        let funding_address = swap_script.funding_addrs.ok_or(anyhow!(
            "No funding address found for Send Swap {}",
            swap.id
        ))?;

        let swap_id = swap.id;
        Ok(SendSwapImmutableData {
            swap_id,
            lockup_script: funding_address.script_pubkey(),
        })
    }
}

#[derive(Clone)]
pub(crate) struct ReceiveSwapImmutableData {
    pub(crate) swap_id: String,
    pub(crate) timeout_block_height: u32,
    pub(crate) claim_script: LBtcScript,
    pub(crate) mrh_script: Option<LBtcScript>,
}

impl TryFrom<ReceiveSwap> for ReceiveSwapImmutableData {
    type Error = anyhow::Error;

    fn try_from(swap: ReceiveSwap) -> std::result::Result<Self, Self::Error> {
        let swap_script = swap.get_swap_script()?;
        let create_response = swap.get_boltz_create_response()?;
        let mrh_address = ElementsAddress::from_str(&swap.mrh_address).ok();

        let funding_address = swap_script.funding_addrs.ok_or(anyhow!(
            "No funding address found for Receive Swap {}",
            swap.id
        ))?;

        let swap_id = swap.id;
        Ok(ReceiveSwapImmutableData {
            swap_id,
            timeout_block_height: create_response.timeout_block_height,
            claim_script: funding_address.script_pubkey(),
            mrh_script: mrh_address.map(|s| s.script_pubkey()),
        })
    }
}

pub(crate) struct ReceiveSwapHistory {
    pub(crate) lbtc_claim_script_history: Vec<HistoryTxId>,
    pub(crate) lbtc_mrh_script_history: Vec<HistoryTxId>,
}

#[derive(Clone)]
pub(crate) struct SendChainSwapImmutableData {
    swap_id: String,
    lockup_script: LBtcScript,
    pub(crate) claim_script: BtcScript,
}

impl TryFrom<ChainSwap> for SendChainSwapImmutableData {
    type Error = anyhow::Error;

    fn try_from(swap: ChainSwap) -> std::result::Result<Self, Self::Error> {
        if swap.direction == Direction::Incoming {
            return Err(anyhow!(
                "Cannot convert incoming chain swap to `SendChainSwapImmutableData`"
            ));
        }

        let lockup_swap_script = swap.get_lockup_swap_script()?.as_liquid_script()?;
        let claim_swap_script = swap.get_claim_swap_script()?.as_bitcoin_script()?;

        let maybe_lockup_script = lockup_swap_script
            .clone()
            .funding_addrs
            .map(|addr| addr.script_pubkey());
        let maybe_claim_script = claim_swap_script
            .clone()
            .funding_addrs
            .map(|addr| addr.script_pubkey());

        let swap_id = swap.id;
        match (maybe_lockup_script, maybe_claim_script) {
            (Some(lockup_script), Some(claim_script)) => Ok(SendChainSwapImmutableData {
                swap_id,
                lockup_script,
                claim_script,
            }),
            (lockup_script, claim_script) => Err(anyhow!("Failed to get lockup or claim script for swap {swap_id}. Lockup script: {lockup_script:?}. Claim script: {claim_script:?}")),
        }
    }
}

pub(crate) struct SendChainSwapHistory {
    pub(crate) lbtc_lockup_script_history: Vec<HistoryTxId>,
    pub(crate) btc_claim_script_history: Vec<HistoryTxId>,
    pub(crate) btc_claim_script_txs: Vec<boltz_client::bitcoin::Transaction>,
}

#[derive(Clone)]
pub(crate) struct ReceiveChainSwapImmutableData {
    swap_id: String,
    pub(crate) lockup_script: BtcScript,
    claim_script: LBtcScript,
}

impl TryFrom<ChainSwap> for ReceiveChainSwapImmutableData {
    type Error = anyhow::Error;

    fn try_from(swap: ChainSwap) -> std::result::Result<Self, Self::Error> {
        if swap.direction == Direction::Outgoing {
            return Err(anyhow!(
                "Cannot convert outgoing chain swap to `ReceiveChainSwapImmutableData`"
            ));
        }

        let lockup_swap_script = swap.get_lockup_swap_script()?.as_bitcoin_script()?;
        let claim_swap_script = swap.get_claim_swap_script()?.as_liquid_script()?;

        let maybe_lockup_script = lockup_swap_script
            .clone()
            .funding_addrs
            .map(|addr| addr.script_pubkey());
        let maybe_claim_script = claim_swap_script
            .clone()
            .funding_addrs
            .map(|addr| addr.script_pubkey());

        let swap_id = swap.id;
        match (maybe_lockup_script, maybe_claim_script) {
            (Some(lockup_script), Some(claim_script)) => Ok(ReceiveChainSwapImmutableData {
                swap_id,
                lockup_script,
                claim_script,
            }),
            (lockup_script, claim_script) => Err(anyhow!("Failed to get lockup or claim script for swap {swap_id}. Lockup script: {lockup_script:?}. Claim script: {claim_script:?}")),
        }
    }
}

pub(crate) struct ReceiveChainSwapHistory {
    pub(crate) lbtc_claim_script_history: Vec<HistoryTxId>,
    pub(crate) btc_lockup_script_history: Vec<HistoryTxId>,
    pub(crate) btc_lockup_script_txs: Vec<boltz_client::bitcoin::Transaction>,
}

/// Swap immutable data
#[derive(Default)]
pub(crate) struct SwapsList {
    pub(crate) send_swap_immutable_data_by_swap_id: HashMap<String, SendSwapImmutableData>,
    pub(crate) receive_swap_immutable_data_by_swap_id: HashMap<String, ReceiveSwapImmutableData>,
    pub(crate) send_chain_swap_immutable_data_by_swap_id:
        HashMap<String, SendChainSwapImmutableData>,
    pub(crate) receive_chain_swap_immutable_data_by_swap_id:
        HashMap<String, ReceiveChainSwapImmutableData>,
}

impl TryFrom<Vec<Swap>> for SwapsList {
    type Error = anyhow::Error;

    fn try_from(swaps: Vec<Swap>) -> std::result::Result<Self, Self::Error> {
        let mut swaps_list = Self::default();

        for swap in swaps.into_iter() {
            let swap_id = swap.id();
            match swap {
                Swap::Send(send_swap) => match send_swap.try_into() {
                    Ok(send_swap_immutable_data) => {
                        swaps_list
                            .send_swap_immutable_data_by_swap_id
                            .insert(swap_id, send_swap_immutable_data);
                    }
                    Err(e) => {
                        log::error!("Could not retrieve send swap immutable data: {e:?}");
                        continue;
                    }
                },
                Swap::Receive(receive_swap) => match receive_swap.try_into() {
                    Ok(receive_swap_immutable_data) => {
                        swaps_list
                            .receive_swap_immutable_data_by_swap_id
                            .insert(swap_id, receive_swap_immutable_data);
                    }
                    Err(e) => {
                        log::error!("Could not retrieve receive swap immutable data: {e:?}");
                        continue;
                    }
                },
                Swap::Chain(chain_swap) => match chain_swap.direction {
                    Direction::Incoming => match chain_swap.try_into() {
                        Ok(receive_chain_swap_immutable_data) => {
                            swaps_list
                                .receive_chain_swap_immutable_data_by_swap_id
                                .insert(swap_id, receive_chain_swap_immutable_data);
                        }
                        Err(e) => {
                            log::error!(
                                "Could not retrieve incoming chain swap immutable data: {e:?}"
                            );
                            continue;
                        }
                    },
                    Direction::Outgoing => match chain_swap.try_into() {
                        Ok(send_chain_swap_immutable_data) => {
                            swaps_list
                                .send_chain_swap_immutable_data_by_swap_id
                                .insert(swap_id, send_chain_swap_immutable_data);
                        }
                        Err(e) => {
                            log::error!(
                                "Could not retrieve outgoing chain swap immutable data: {e:?}"
                            );
                            continue;
                        }
                    },
                },
            }
        }

        Ok(swaps_list)
    }
}

impl SwapsList {
    fn send_swaps_by_script(&self) -> HashMap<LBtcScript, SendSwapImmutableData> {
        self.send_swap_immutable_data_by_swap_id
            .clone()
            .into_values()
            .map(|imm| (imm.lockup_script.clone(), imm))
            .collect()
    }

    pub(crate) fn send_histories_by_swap_id(
        &self,
        lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
    ) -> HashMap<String, SendSwapHistory> {
        let send_swaps_by_script = self.send_swaps_by_script();

        let mut data: HashMap<String, SendSwapHistory> = HashMap::new();
        lbtc_script_to_history_map
            .iter()
            .for_each(|(lbtc_script, lbtc_script_history)| {
                if let Some(imm) = send_swaps_by_script.get(lbtc_script) {
                    data.insert(imm.swap_id.clone(), lbtc_script_history.clone());
                }
            });
        data
    }

    fn receive_swaps_by_claim_script(&self) -> HashMap<LBtcScript, ReceiveSwapImmutableData> {
        self.receive_swap_immutable_data_by_swap_id
            .clone()
            .into_values()
            .map(|imm| (imm.claim_script.clone(), imm))
            .collect()
    }

    fn receive_swaps_by_mrh_script(&self) -> HashMap<LBtcScript, ReceiveSwapImmutableData> {
        self.receive_swap_immutable_data_by_swap_id
            .clone()
            .into_values()
            .filter_map(|imm| imm.mrh_script.clone().map(|mrh_script| (mrh_script, imm)))
            .collect()
    }

    pub(crate) fn receive_histories_by_swap_id(
        &self,
        lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
    ) -> HashMap<String, ReceiveSwapHistory> {
        let receive_swaps_by_claim_script = self.receive_swaps_by_claim_script();
        let receive_swaps_by_mrh_script = self.receive_swaps_by_mrh_script();

        let mut data: HashMap<String, ReceiveSwapHistory> = HashMap::new();
        lbtc_script_to_history_map
            .iter()
            .for_each(|(lbtc_script, lbtc_script_history)| {
                if let Some(imm) = receive_swaps_by_claim_script.get(lbtc_script) {
                    // The MRH script history filtered by the swap timeout block height
                    let mrh_script_history = imm
                        .mrh_script
                        .clone()
                        .and_then(|mrh_script| {
                            lbtc_script_to_history_map.get(&mrh_script).map(|h| {
                                h.iter()
                                    .filter(|&tx_history| {
                                        tx_history.height < imm.timeout_block_height as i32
                                    })
                                    .cloned()
                                    .collect::<Vec<HistoryTxId>>()
                            })
                        })
                        .unwrap_or_default();
                    data.insert(
                        imm.swap_id.clone(),
                        ReceiveSwapHistory {
                            lbtc_claim_script_history: lbtc_script_history.clone(),
                            lbtc_mrh_script_history: mrh_script_history,
                        },
                    );
                }
                if let Some(imm) = receive_swaps_by_mrh_script.get(lbtc_script) {
                    let claim_script_history = lbtc_script_to_history_map
                        .get(&imm.claim_script)
                        .cloned()
                        .unwrap_or_default();
                    // The MRH script history filtered by the swap timeout block height
                    let mrh_script_history = lbtc_script_history
                        .iter()
                        .filter(|&tx_history| tx_history.height < imm.timeout_block_height as i32)
                        .cloned()
                        .collect::<Vec<HistoryTxId>>();
                    data.insert(
                        imm.swap_id.clone(),
                        ReceiveSwapHistory {
                            lbtc_claim_script_history: claim_script_history,
                            lbtc_mrh_script_history: mrh_script_history,
                        },
                    );
                }
            });
        data
    }

    fn send_chain_swaps_by_lbtc_lockup_script(
        &self,
    ) -> HashMap<LBtcScript, SendChainSwapImmutableData> {
        self.send_chain_swap_immutable_data_by_swap_id
            .clone()
            .into_values()
            .map(|imm| (imm.lockup_script.clone(), imm))
            .collect()
    }

    pub(crate) fn send_chain_histories_by_swap_id(
        &self,
        lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
        btc_script_to_history_map: &HashMap<BtcScript, Vec<HistoryTxId>>,
        btc_script_to_txs_map: &HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>>,
    ) -> HashMap<String, SendChainSwapHistory> {
        let send_chain_swaps_by_lbtc_script = self.send_chain_swaps_by_lbtc_lockup_script();

        let mut data: HashMap<String, SendChainSwapHistory> = HashMap::new();
        lbtc_script_to_history_map
            .iter()
            .for_each(|(lbtc_lockup_script, lbtc_script_history)| {
                if let Some(imm) = send_chain_swaps_by_lbtc_script.get(lbtc_lockup_script) {
                    let btc_script_history = btc_script_to_history_map
                        .get(&imm.claim_script)
                        .cloned()
                        .unwrap_or_default();
                    let btc_script_txs = btc_script_to_txs_map
                        .get(&imm.claim_script)
                        .cloned()
                        .unwrap_or_default();

                    data.insert(
                        imm.swap_id.clone(),
                        SendChainSwapHistory {
                            lbtc_lockup_script_history: lbtc_script_history.clone(),
                            btc_claim_script_history: btc_script_history,
                            btc_claim_script_txs: btc_script_txs,
                        },
                    );
                }
            });
        data
    }

    fn receive_chain_swaps_by_lbtc_claim_script(
        &self,
    ) -> HashMap<LBtcScript, ReceiveChainSwapImmutableData> {
        self.receive_chain_swap_immutable_data_by_swap_id
            .clone()
            .into_values()
            .map(|imm| (imm.claim_script.clone(), imm))
            .collect()
    }

    pub(super) fn receive_chain_histories_by_swap_id(
        &self,
        lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
        btc_script_to_history_map: &HashMap<BtcScript, Vec<HistoryTxId>>,
        btc_script_to_txs_map: &HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>>,
    ) -> HashMap<String, ReceiveChainSwapHistory> {
        let receive_chain_swaps_by_lbtc_script = self.receive_chain_swaps_by_lbtc_claim_script();

        let mut data: HashMap<String, ReceiveChainSwapHistory> = HashMap::new();
        lbtc_script_to_history_map
            .iter()
            .for_each(|(lbtc_script_pk, lbtc_script_history)| {
                if let Some(imm) = receive_chain_swaps_by_lbtc_script.get(lbtc_script_pk) {
                    let btc_script_history = btc_script_to_history_map
                        .get(&imm.lockup_script)
                        .cloned()
                        .unwrap_or_default();
                    let btc_script_txs = btc_script_to_txs_map
                        .get(&imm.lockup_script)
                        .cloned()
                        .unwrap_or_default();

                    data.insert(
                        imm.swap_id.clone(),
                        ReceiveChainSwapHistory {
                            lbtc_claim_script_history: lbtc_script_history.clone(),
                            btc_lockup_script_history: btc_script_history,
                            btc_lockup_script_txs: btc_script_txs,
                        },
                    );
                }
            });
        data
    }

    pub(crate) fn get_swap_lbtc_scripts(&self, partial_sync: bool) -> Vec<LBtcScript> {
        let receive_swap_lbtc_mrh_scripts: Vec<LBtcScript> = self
            .receive_swap_immutable_data_by_swap_id
            .clone()
            .into_values()
            .filter_map(|imm| imm.mrh_script)
            .collect();
        let mut swap_scripts = receive_swap_lbtc_mrh_scripts.clone();
        if !partial_sync {
            let send_swap_scripts: Vec<LBtcScript> = self
                .send_swap_immutable_data_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.lockup_script)
                .collect();
            let receive_swap_lbtc_claim_scripts: Vec<LBtcScript> = self
                .receive_swap_immutable_data_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.claim_script)
                .collect();
            let send_chain_swap_lbtc_lockup_scripts: Vec<LBtcScript> = self
                .send_chain_swap_immutable_data_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.lockup_script)
                .collect();
            let receive_chain_swap_lbtc_claim_scripts: Vec<LBtcScript> = self
                .receive_chain_swap_immutable_data_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.claim_script)
                .collect();
            swap_scripts.extend(send_swap_scripts.clone());
            swap_scripts.extend(receive_swap_lbtc_claim_scripts.clone());
            swap_scripts.extend(send_chain_swap_lbtc_lockup_scripts.clone());
            swap_scripts.extend(receive_chain_swap_lbtc_claim_scripts.clone());
        }
        swap_scripts
    }

    pub(crate) fn get_swap_btc_scripts(&self, partial_sync: bool) -> Vec<BtcScript> {
        let mut swap_scripts = vec![];
        if !partial_sync {
            let send_chain_swap_btc_claim_scripts: Vec<BtcScript> = self
                .send_chain_swap_immutable_data_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.claim_script)
                .collect();
            let receive_chain_swap_btc_lockup_scripts: Vec<BtcScript> = self
                .receive_chain_swap_immutable_data_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.lockup_script)
                .collect();
            swap_scripts.extend(send_chain_swap_btc_claim_scripts.clone());
            swap_scripts.extend(receive_chain_swap_btc_lockup_scripts.clone());
        }
        swap_scripts
    }
}

pub(crate) struct SwapsHistories {
    pub(crate) send: HashMap<String, SendSwapHistory>,
    pub(crate) receive: HashMap<String, ReceiveSwapHistory>,
    pub(crate) send_chain: HashMap<String, SendChainSwapHistory>,
    pub(crate) receive_chain: HashMap<String, ReceiveChainSwapHistory>,
}
