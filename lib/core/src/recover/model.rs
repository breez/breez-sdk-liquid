use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use boltz_client::ElementsAddress;
use electrum_client::GetBalanceRes;
use lwk_wollet::elements::Txid;
use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;
use lwk_wollet::History;
use lwk_wollet::WalletTx;
use tonic::async_trait;

use crate::chain::liquid::LiquidChainService;
use crate::prelude::*;
use crate::swapper::Swapper;
use anyhow::Result;

pub(crate) type BtcScript = lwk_wollet::bitcoin::ScriptBuf;
pub(crate) type LBtcScript = lwk_wollet::elements::Script;

#[async_trait]
pub(crate) trait SwapRecoverHandler: Send + Sync {
    async fn recover_swap(
        &self,
        swap: &mut Swap,
        context: &SwapsHistories,
        is_local_within_grace_period: bool,
    ) -> Result<bool>;
}

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
