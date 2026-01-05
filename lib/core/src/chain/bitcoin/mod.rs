pub(crate) mod electrum;
pub(crate) mod esplora;

use anyhow::Result;
use bitcoin::{Address, Script, Transaction, Txid};

use crate::{
    bitcoin,
    model::{BtcHistory, BtcScriptBalance, RecommendedFees, Utxo},
};

pub(crate) type History = BtcHistory;

/// Trait implemented by types that can fetch data from a blockchain data source.
#[sdk_macros::async_trait]
pub trait BitcoinChainService: Send + Sync {
    /// Get the blockchain latest block
    async fn tip(&self) -> Result<u32>;

    /// Broadcast a transaction
    async fn broadcast(&self, tx: &Transaction) -> Result<Txid>;

    /// Get a list of transactions
    async fn get_transactions_with_retry(
        &self,
        txids: &[Txid],
        retries: u64,
    ) -> Result<Vec<Transaction>>;

    /// Get the transactions involved for a script
    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>>;

    /// Get the transactions involved for a script
    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>>;

    /// Get the transactions involved in a list of scripts
    async fn get_scripts_history_with_retry(
        &self,
        scripts: &[&Script],
        retries: u64,
    ) -> Result<Vec<Vec<History>>>;

    /// Get the utxos associated with a script pubkey
    async fn get_script_utxos(&self, script: &Script) -> Result<Vec<Utxo>>;

    /// Get the utxos associated with a list of scripts
    async fn get_scripts_utxos(&self, scripts: &[&Script]) -> Result<Vec<Vec<Utxo>>>;

    /// Return the confirmed and unconfirmed balances of a script hash
    async fn script_get_balance(&self, script: &Script) -> Result<BtcScriptBalance>;

    /// Return the confirmed and unconfirmed balances of a list of script hashes
    async fn scripts_get_balance(&self, scripts: &[&Script]) -> Result<Vec<BtcScriptBalance>>;

    /// Return the confirmed and unconfirmed balances of a script hash
    async fn script_get_balance_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<BtcScriptBalance>;

    /// Verify that a transaction appears in the address script history
    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction>;

    /// Get the recommended fees, in sat/vbyte
    async fn recommended_fees(&self) -> Result<RecommendedFees>;
}
