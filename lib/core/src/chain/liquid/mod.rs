pub(crate) mod electrum;
pub(crate) mod esplora;

use anyhow::Result;
use elements::{Address, Script, Transaction, Txid};
use mockall::automock;

use crate::{
    elements,
    model::{LBtcHistory, Utxo},
};

pub(crate) type History = LBtcHistory;

#[automock]
#[sdk_macros::async_trait]
pub trait LiquidChainService: Send + Sync {
    /// Get the blockchain latest block
    async fn tip(&self) -> Result<u32>;

    /// Broadcast a transaction
    async fn broadcast(&self, tx: &Transaction) -> Result<Txid>;

    /// Get a single transaction from its raw hash
    async fn get_transaction_hex(&self, txid: &Txid) -> Result<Option<Transaction>>;

    /// Get a list of transactions
    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>>;

    /// Get the transactions involved in a script
    async fn get_script_history(&self, scripts: &Script) -> Result<Vec<History>>;

    /// Get the transactions involved in a list of scripts
    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>>;

    /// Get the transactions involved in a list of scripts.
    ///
    /// The data is fetched in a single call from the Electrum endpoint.
    async fn get_scripts_history_with_retry(
        &self,
        scripts: &[Script],
        retries: u64,
    ) -> Result<Vec<Vec<History>>>;

    /// Get the utxos associated with a script pubkey
    async fn get_script_utxos(&self, script: &Script) -> Result<Vec<Utxo>>;

    /// Verify that a transaction appears in the address script history
    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction>;
}
