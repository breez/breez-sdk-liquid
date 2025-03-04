use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use boltz_client::boltz::PairLimits;
use boltz_client::ElementsAddress;
use electrum_client::GetBalanceRes;
use lwk_wollet::elements::Txid;
use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;
use lwk_wollet::History;
use lwk_wollet::WalletTx;

use crate::chain::liquid::LiquidChainService;
use crate::prelude::*;
use crate::swapper::Swapper;

pub(crate) type BtcScript = lwk_wollet::bitcoin::ScriptBuf;
pub(crate) type LBtcScript = lwk_wollet::elements::Script;

#[derive(Clone, Debug)]
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

pub(crate) struct RecoveredOnchainDataSend {
    pub(crate) lockup_tx_id: Option<HistoryTxId>,
    pub(crate) claim_tx_id: Option<HistoryTxId>,
    pub(crate) refund_tx_id: Option<HistoryTxId>,
    pub(crate) preimage: Option<String>,
}

impl RecoveredOnchainDataSend {
    pub(crate) fn derive_partial_state(&self, is_expired: bool) -> Option<PaymentState> {
        match &self.lockup_tx_id {
            Some(_) => match &self.claim_tx_id {
                Some(_) => Some(PaymentState::Complete),
                None => match &self.refund_tx_id {
                    Some(refund_tx_id) => match refund_tx_id.confirmed() {
                        true => Some(PaymentState::Failed),
                        false => Some(PaymentState::RefundPending),
                    },
                    None => match is_expired {
                        true => Some(PaymentState::RefundPending),
                        false => Some(PaymentState::Pending),
                    },
                },
            },
            None => match is_expired {
                true => Some(PaymentState::Failed),
                // We have no onchain data to support deriving the state as the swap could
                // potentially be Created or TimedOut. In this case we return None.
                false => None,
            },
        }
    }
}

pub(crate) struct RecoveredOnchainDataReceive {
    pub(crate) lockup_tx_id: Option<HistoryTxId>,
    pub(crate) claim_tx_id: Option<HistoryTxId>,
    pub(crate) mrh_tx_id: Option<HistoryTxId>,
    pub(crate) mrh_amount_sat: Option<u64>,
}

impl RecoveredOnchainDataReceive {
    pub(crate) fn derive_partial_state(&self, is_expired: bool) -> Option<PaymentState> {
        match &self.lockup_tx_id {
            Some(_) => match &self.claim_tx_id {
                Some(claim_tx_id) => match claim_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                None => match is_expired {
                    true => Some(PaymentState::Failed),
                    false => Some(PaymentState::Pending),
                },
            },
            None => match &self.mrh_tx_id {
                Some(mrh_tx_id) => match mrh_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                // We have no onchain data to support deriving the state as the swap could
                // potentially be Created. In this case we return None.
                None => match is_expired {
                    true => Some(PaymentState::Failed),
                    false => None,
                },
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

// TODO: We have to be careful around overwriting the RefundPending state, as this swap monitored
// after the expiration of the swap and if new funds are detected on the lockup script they are refunded.
// Perhaps we should check in the recovery the lockup balance and set accordingly.
impl RecoveredOnchainDataChainSend {
    pub(crate) fn derive_partial_state(&self, is_expired: bool) -> Option<PaymentState> {
        match &self.lbtc_user_lockup_tx_id {
            Some(_) => match (&self.btc_claim_tx_id, &self.lbtc_refund_tx_id) {
                (Some(btc_claim_tx_id), None) => match btc_claim_tx_id.confirmed() {
                    true => Some(PaymentState::Complete),
                    false => Some(PaymentState::Pending),
                },
                (None, Some(lbtc_refund_tx_id)) => match lbtc_refund_tx_id.confirmed() {
                    true => Some(PaymentState::Failed),
                    false => Some(PaymentState::RefundPending),
                },
                (Some(btc_claim_tx_id), Some(lbtc_refund_tx_id)) => {
                    match btc_claim_tx_id.confirmed() {
                        true => match lbtc_refund_tx_id.confirmed() {
                            true => Some(PaymentState::Complete),
                            false => Some(PaymentState::RefundPending),
                        },
                        false => Some(PaymentState::Pending),
                    }
                }
                (None, None) => match is_expired {
                    true => Some(PaymentState::RefundPending),
                    false => Some(PaymentState::Pending),
                },
            },
            None => match is_expired {
                true => Some(PaymentState::Failed),
                // We have no onchain data to support deriving the state as the swap could
                // potentially be Created or TimedOut. In this case we return None.
                false => None,
            },
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
    /// BTC total funds currently available at the swap funding address.
    pub(crate) btc_user_lockup_address_balance_sat: u64,
    /// BTC sent to lockup address as part of lockup tx.
    pub(crate) btc_user_lockup_amount_sat: u64,
    /// BTC tx initiated by the SDK to a user-chosen address, in case the initial funds have to be refunded.
    pub(crate) btc_refund_tx_id: Option<HistoryTxId>,
}

impl RecoveredOnchainDataChainReceive {
    pub(crate) fn derive_partial_state(
        &self,
        expected_user_lockup_amount_sat: Option<u64>,
        swap_limits: Option<PairLimits>,
        is_expired: bool,
        is_waiting_fee_acceptance: bool,
    ) -> Option<PaymentState> {
        let unexpected_amount =
            expected_user_lockup_amount_sat.is_some_and(|expected_lockup_amount_sat| {
                expected_lockup_amount_sat != self.btc_user_lockup_amount_sat
            });
        let amount_out_of_bounds = swap_limits.is_some_and(|limits| {
            self.btc_user_lockup_amount_sat < limits.minimal
                || self.btc_user_lockup_amount_sat > limits.maximal
        });
        let is_expired_refundable = is_expired && self.btc_user_lockup_address_balance_sat > 0;
        let is_refundable = is_expired_refundable || unexpected_amount || amount_out_of_bounds;
        match &self.btc_user_lockup_tx_id {
            Some(_) => match (&self.lbtc_claim_tx_id, &self.btc_refund_tx_id) {
                (Some(lbtc_claim_tx_id), None) => match lbtc_claim_tx_id.confirmed() {
                    true => match is_expired_refundable {
                        true => Some(PaymentState::Refundable),
                        false => Some(PaymentState::Complete),
                    },
                    false => Some(PaymentState::Pending),
                },
                (None, Some(btc_refund_tx_id)) => match btc_refund_tx_id.confirmed() {
                    true => match is_expired_refundable {
                        true => Some(PaymentState::Refundable),
                        false => Some(PaymentState::Failed),
                    },
                    false => Some(PaymentState::RefundPending),
                },
                (Some(lbtc_claim_tx_id), Some(btc_refund_tx_id)) => {
                    match lbtc_claim_tx_id.confirmed() {
                        true => match btc_refund_tx_id.confirmed() {
                            true => match is_expired_refundable {
                                true => Some(PaymentState::Refundable),
                                false => Some(PaymentState::Complete),
                            },
                            false => Some(PaymentState::RefundPending),
                        },
                        false => Some(PaymentState::Pending),
                    }
                }
                (None, None) => match is_refundable {
                    true => Some(PaymentState::Refundable),
                    false => match is_waiting_fee_acceptance {
                        true => Some(PaymentState::WaitingFeeAcceptance),
                        false => Some(PaymentState::Pending),
                    },
                },
            },
            None => match is_expired {
                true => Some(PaymentState::Failed),
                // We have no onchain data to support deriving the state as the swap could
                // potentially be Created. In this case we return None.
                false => None,
            },
        }
    }
}

#[derive(Clone)]
pub(crate) struct ReceiveSwapHistory {
    pub(crate) lbtc_claim_script_history: Vec<HistoryTxId>,
    pub(crate) lbtc_mrh_script_history: Vec<HistoryTxId>,
}

#[derive(Clone)]
pub(crate) struct SendChainSwapHistory {
    pub(crate) lbtc_lockup_script_history: Vec<HistoryTxId>,
    pub(crate) btc_claim_script_history: Vec<HistoryTxId>,
    pub(crate) btc_claim_script_txs: Vec<boltz_client::bitcoin::Transaction>,
}

#[derive(Clone)]
pub(crate) struct ReceiveChainSwapHistory {
    pub(crate) lbtc_claim_script_history: Vec<HistoryTxId>,
    pub(crate) btc_lockup_script_history: Vec<HistoryTxId>,
    pub(crate) btc_lockup_script_txs: Vec<boltz_client::bitcoin::Transaction>,
    pub(crate) btc_lockup_script_balance: Option<GetBalanceRes>,
}

/// Swap list containing all swap data indexed by swap ID
#[derive(Default)]
pub(crate) struct SwapsList {
    // Single map for all swap types indexed by swap ID
    pub(crate) swaps_by_id: HashMap<String, Swap>,
}

impl TryFrom<Vec<Swap>> for SwapsList {
    type Error = anyhow::Error;

    fn try_from(swaps: Vec<Swap>) -> std::result::Result<Self, Self::Error> {
        let mut swaps_list = Self::default();

        for swap in swaps.into_iter() {
            let swap_id = swap.id();
            swaps_list.swaps_by_id.insert(swap_id, swap);
        }

        Ok(swaps_list)
    }
}

impl SwapsList {
    pub(crate) fn get_swap_lbtc_scripts(&self) -> Vec<LBtcScript> {
        let mut swap_scripts = Vec::new();

        for swap in self.swaps_by_id.values() {
            match swap {
                Swap::Send(send_swap) => {
                    if let Ok(script) = send_swap.get_swap_script() {
                        if let Some(funding_addr) = script.funding_addrs {
                            swap_scripts.push(funding_addr.script_pubkey());
                        }
                    }
                }
                Swap::Receive(receive_swap) => {
                    // Add claim script
                    if let Ok(script) = receive_swap.get_swap_script() {
                        if let Some(funding_addr) = script.funding_addrs {
                            swap_scripts.push(funding_addr.script_pubkey());
                        }
                    }

                    // Add MRH script if available
                    if let Ok(mrh_address) = ElementsAddress::from_str(&receive_swap.mrh_address) {
                        swap_scripts.push(mrh_address.script_pubkey());
                    }
                }
                Swap::Chain(chain_swap) => {
                    match chain_swap.direction {
                        Direction::Outgoing => {
                            // For outgoing chain swaps, add lockup script
                            if let Ok(lockup_script) = chain_swap.get_lockup_swap_script() {
                                if let Ok(liquid_script) = lockup_script.as_liquid_script() {
                                    if let Some(funding_addr) = liquid_script.funding_addrs {
                                        swap_scripts.push(funding_addr.script_pubkey());
                                    }
                                }
                            }
                        }
                        Direction::Incoming => {
                            // For incoming chain swaps, add claim script
                            if let Ok(claim_script) = chain_swap.get_claim_swap_script() {
                                if let Ok(liquid_script) = claim_script.as_liquid_script() {
                                    if let Some(funding_addr) = liquid_script.funding_addrs {
                                        swap_scripts.push(funding_addr.script_pubkey());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        swap_scripts
    }

    pub(crate) fn get_swap_btc_scripts(&self) -> Vec<BtcScript> {
        let mut swap_scripts = Vec::new();

        for swap in self.swaps_by_id.values() {
            if let Swap::Chain(chain_swap) = swap {
                match chain_swap.direction {
                    Direction::Outgoing => {
                        // For outgoing chain swaps, add claim script (BTC)
                        if let Ok(claim_script) = chain_swap.get_claim_swap_script() {
                            if let Ok(bitcoin_script) = claim_script.as_bitcoin_script() {
                                if let Some(funding_addr) = bitcoin_script.funding_addrs {
                                    swap_scripts.push(funding_addr.script_pubkey());
                                }
                            }
                        }
                    }
                    Direction::Incoming => {
                        // For incoming chain swaps, add lockup script (BTC)
                        if let Ok(lockup_script) = chain_swap.get_lockup_swap_script() {
                            if let Ok(bitcoin_script) = lockup_script.as_bitcoin_script() {
                                if let Some(funding_addr) = bitcoin_script.funding_addrs {
                                    swap_scripts.push(funding_addr.script_pubkey());
                                }
                            }
                        }
                    }
                }
            }
        }

        swap_scripts
    }
}

pub(crate) struct SwapsHistories {
    pub(crate) lbtc_script_to_history_map: HashMap<LBtcScript, Vec<HistoryTxId>>,
    pub(crate) btc_script_to_history_map: HashMap<BtcScript, Vec<HistoryTxId>>,
    pub(crate) btc_script_to_txs_map: HashMap<BtcScript, Vec<boltz_client::bitcoin::Transaction>>,
    pub(crate) btc_script_to_balance_map: HashMap<BtcScript, GetBalanceRes>,
    pub(crate) liquid_chain_service: Arc<dyn LiquidChainService>,
    pub(crate) swapper: Arc<dyn Swapper>,
    pub(crate) tx_map: TxMap,
    pub(crate) master_blinding_key: MasterBlindingKey,
    pub(crate) liquid_tip_height: u32,
    pub(crate) bitcoin_tip_height: u32,
}
