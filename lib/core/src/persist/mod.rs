mod address;
mod backup;
pub(crate) mod cache;
pub(crate) mod chain;
mod migrations;
pub(crate) mod model;
pub(crate) mod receive;
pub(crate) mod send;
pub(crate) mod sync;

use std::collections::{HashMap, HashSet};
use std::ops::Not;
use std::sync::RwLock;
use std::{fs::create_dir_all, path::PathBuf, str::FromStr};

use crate::lightning_invoice::{Bolt11Invoice, Bolt11InvoiceDescription};
use crate::model::*;
use crate::sync::model::RecordType;
use crate::{get_invoice_description, utils};
use anyhow::{anyhow, Result};
use boltz_client::boltz::{ChainPair, ReversePair, SubmarinePair};
use lwk_wollet::WalletTx;
use migrations::current_migrations;
use model::PaymentTxDetails;
use rusqlite::{
    params, params_from_iter, Connection, OptionalExtension, Row, ToSql, TransactionBehavior,
};
use rusqlite_migration::{Migrations, M};
use sdk_common::bitcoin::hashes::hex::ToHex;
use tokio::sync::mpsc::Sender;

const DEFAULT_DB_FILENAME: &str = "storage.sql";

pub(crate) struct Persister {
    main_db_dir: PathBuf,
    network: LiquidNetwork,
    pub(crate) sync_trigger: RwLock<Option<Sender<()>>>,
}

/// Builds a WHERE clause that checks if `state` is any of the given arguments
fn get_where_clause_state_in(allowed_states: &[PaymentState]) -> String {
    format!(
        "state in ({})",
        allowed_states
            .iter()
            .map(|t| format!("'{}'", *t as i8))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

impl Persister {
    pub fn new(
        working_dir: &str,
        network: LiquidNetwork,
        sync_trigger: Option<Sender<()>>,
    ) -> Result<Self> {
        let main_db_dir = PathBuf::from_str(working_dir)?;
        if !main_db_dir.exists() {
            create_dir_all(&main_db_dir)?;
        }
        Ok(Persister {
            main_db_dir,
            network,
            sync_trigger: RwLock::new(sync_trigger),
        })
    }

    pub(crate) fn get_connection(&self) -> Result<Connection> {
        Ok(Connection::open(
            self.main_db_dir.join(DEFAULT_DB_FILENAME),
        )?)
    }

    pub fn init(&self) -> Result<()> {
        self.migrate_main_db()?;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn get_database_dir(&self) -> &PathBuf {
        &self.main_db_dir
    }

    fn migrate_main_db(&self) -> Result<()> {
        let migrations = Migrations::new(current_migrations().into_iter().map(M::up).collect());
        let mut conn = self.get_connection()?;
        migrations.to_latest(&mut conn)?;
        Ok(())
    }

    pub(crate) fn fetch_swap_by_id(&self, id: &str) -> Result<Swap> {
        match self.fetch_send_swap_by_id(id) {
            Ok(Some(send_swap)) => Ok(Swap::Send(send_swap)),
            _ => match self.fetch_receive_swap_by_id(id) {
                Ok(Some(receive_swap)) => Ok(Swap::Receive(receive_swap)),
                _ => match self.fetch_chain_swap_by_id(id) {
                    Ok(Some(chain_swap)) => Ok(Swap::Chain(chain_swap)),
                    _ => Err(anyhow!("Could not find Swap {id}")),
                },
            },
        }
    }

    pub(crate) fn insert_or_update_payment_with_wallet_tx(&self, tx: &WalletTx) -> Result<()> {
        let tx_id = tx.txid.to_string();
        let is_tx_confirmed = tx.height.is_some();
        let amount_sat = tx
            .balance
            .iter()
            .filter_map(|(asset_id, balance)| {
                if *asset_id == utils::lbtc_asset_id(self.network) {
                    return Some(balance);
                }
                None
            })
            .sum::<i64>();
        if amount_sat == 0 {
            log::warn!("Attempted to persist a payment with no output amount: tx_id {tx_id}");
            return Ok(());
        }
        let maybe_script_pubkey = tx
            .outputs
            .iter()
            .find(|output| output.is_some())
            .and_then(|output| output.clone().map(|o| o.script_pubkey.to_hex()));
        let unblinding_data = tx
            .unblinded_url("")
            .replace(&format!("tx/{}#blinded=", tx_id), "");
        self.insert_or_update_payment(
            PaymentTxData {
                tx_id: tx_id.clone(),
                timestamp: tx.timestamp,
                amount_sat: amount_sat.unsigned_abs(),
                fees_sat: tx.fee,
                payment_type: match amount_sat >= 0 {
                    true => PaymentType::Receive,
                    false => PaymentType::Send,
                },
                is_confirmed: is_tx_confirmed,
                unblinding_data: Some(unblinding_data),
            },
            maybe_script_pubkey.map(|destination| PaymentTxDetails {
                tx_id,
                destination,
                ..Default::default()
            }),
            true,
        )
    }

    pub(crate) fn list_unconfirmed_payment_txs_data(&self) -> Result<Vec<PaymentTxData>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(
            "SELECT tx_id, 
                        timestamp, 
                        amount_sat, 
                        fees_sat, 
                        payment_type, 
                        is_confirmed,
                        unblinding_data
            FROM payment_tx_data
            WHERE is_confirmed = 0",
        )?;
        let payments: Vec<PaymentTxData> = stmt
            .query_map([], |row| {
                Ok(PaymentTxData {
                    tx_id: row.get(0)?,
                    timestamp: row.get(1)?,
                    amount_sat: row.get(2)?,
                    fees_sat: row.get(3)?,
                    payment_type: row.get(4)?,
                    is_confirmed: row.get(5)?,
                    unblinding_data: row.get(6)?,
                })
            })?
            .map(|i| i.unwrap())
            .collect();
        Ok(payments)
    }

    pub(crate) fn insert_or_update_payment(
        &self,
        ptx: PaymentTxData,
        payment_tx_details: Option<PaymentTxDetails>,
        from_wallet_tx_data: bool,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT INTO payment_tx_data (
           tx_id,
           timestamp,
           amount_sat,
           fees_sat,
           payment_type,
           is_confirmed,
           unblinding_data
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT (tx_id)
        DO UPDATE SET timestamp = CASE WHEN excluded.is_confirmed = 1 THEN excluded.timestamp ELSE timestamp END,
                      amount_sat = excluded.amount_sat,
                      fees_sat = excluded.fees_sat,
                      payment_type = excluded.payment_type,
                      is_confirmed = excluded.is_confirmed,
                      unblinding_data = excluded.unblinding_data
        ",
            (
                &ptx.tx_id,
                ptx.timestamp.or(Some(utils::now())),
                ptx.amount_sat,
                ptx.fees_sat,
                ptx.payment_type,
                ptx.is_confirmed,
                ptx.unblinding_data,
            ),
        )?;

        let mut trigger_sync = false;
        if let Some(ref payment_tx_details) = payment_tx_details {
            // If the update comes from the wallet tx:
            // - Skip updating the destination from the script_pubkey
            // - Skip syncing the payment_tx_details
            Self::insert_or_update_payment_details_inner(
                &tx,
                payment_tx_details,
                from_wallet_tx_data,
            )?;
            if !from_wallet_tx_data {
                self.commit_outgoing(
                    &tx,
                    &payment_tx_details.tx_id,
                    RecordType::PaymentDetails,
                    None,
                )?;
                trigger_sync = true;
            }
        }

        tx.commit()?;
        if trigger_sync {
            self.trigger_sync()?;
        }

        Ok(())
    }

    fn insert_or_update_payment_details_inner(
        con: &Connection,
        payment_tx_details: &PaymentTxDetails,
        skip_destination_update: bool,
    ) -> Result<()> {
        let destination_update = skip_destination_update
            .not()
            .then_some("destination = excluded.destination,")
            .unwrap_or_default();
        con.execute(
            &format!(
                "INSERT INTO payment_details (
                tx_id,
                destination,
                description,
                lnurl_info_json
            )
            VALUES (?, ?, ?, ?)
            ON CONFLICT (tx_id)
            DO UPDATE SET
                {destination_update}
                description = COALESCE(excluded.description, description),
                lnurl_info_json = COALESCE(excluded.lnurl_info_json, lnurl_info_json)
        "
            ),
            (
                &payment_tx_details.tx_id,
                &payment_tx_details.destination,
                &payment_tx_details.description,
                payment_tx_details
                    .lnurl_info
                    .as_ref()
                    .map(|info| serde_json::to_string(&info).ok()),
            ),
        )?;
        Ok(())
    }

    pub(crate) fn insert_or_update_payment_details(
        &self,
        payment_tx_details: PaymentTxDetails,
    ) -> Result<()> {
        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(TransactionBehavior::Immediate)?;

        Self::insert_or_update_payment_details_inner(&tx, &payment_tx_details, false)?;
        self.commit_outgoing(
            &tx,
            &payment_tx_details.tx_id,
            RecordType::PaymentDetails,
            None,
        )?;
        tx.commit()?;
        self.trigger_sync()?;

        Ok(())
    }

    pub(crate) fn get_payment_details(&self, tx_id: &str) -> Result<Option<PaymentTxDetails>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(
            "SELECT destination, description, lnurl_info_json
            FROM payment_details
            WHERE tx_id = ?",
        )?;
        let res = stmt.query_row([tx_id], |row| {
            let destination = row.get(0)?;
            let description = row.get(1)?;
            let maybe_lnurl_info_json: Option<String> = row.get(2)?;
            Ok(PaymentTxDetails {
                tx_id: tx_id.to_string(),
                destination,
                description,
                lnurl_info: maybe_lnurl_info_json
                    .and_then(|info| serde_json::from_str::<LnUrlInfo>(&info).ok()),
            })
        });
        Ok(res.ok())
    }

    pub(crate) fn list_ongoing_swaps(&self) -> Result<Vec<Swap>> {
        let ongoing_send_swaps: Vec<Swap> = self
            .list_ongoing_send_swaps()?
            .into_iter()
            .map(Swap::Send)
            .collect();
        let ongoing_receive_swaps: Vec<Swap> = self
            .list_ongoing_receive_swaps()?
            .into_iter()
            .map(Swap::Receive)
            .collect();
        let ongoing_chain_swaps: Vec<Swap> = self
            .list_ongoing_chain_swaps()?
            .into_iter()
            .map(Swap::Chain)
            .collect();
        Ok([
            ongoing_send_swaps,
            ongoing_receive_swaps,
            ongoing_chain_swaps,
        ]
        .concat())
    }

    fn select_payment_query(
        &self,
        where_clause: Option<&str>,
        offset: Option<u32>,
        limit: Option<u32>,
        sort_ascending: Option<bool>,
    ) -> String {
        format!(
            "
            SELECT
                ptx.tx_id,
                ptx.timestamp,
                ptx.amount_sat,
                ptx.fees_sat,
                ptx.payment_type,
                ptx.is_confirmed,
                ptx.unblinding_data,
                rs.id,
                rs.created_at,
                rs.timeout_block_height,
                rs.invoice,
                rs.payment_hash,
                rs.destination_pubkey,
                rs.description,
                rs.preimage,
                rs.payer_amount_sat,
                rs.receiver_amount_sat,
                rs.state,
                rs.pair_fees_json,
                ss.id,
                ss.created_at,
                ss.timeout_block_height,
                ss.invoice,
                ss.bolt12_offer,
                ss.payment_hash,
                ss.destination_pubkey,
                ss.description,
                ss.preimage,
                ss.refund_tx_id,
                ss.payer_amount_sat,
                ss.receiver_amount_sat,
                ss.state,
                ss.pair_fees_json,
                cs.id,
                cs.created_at,
                cs.timeout_block_height,
                cs.direction,
                cs.preimage,
                cs.description,
                cs.refund_tx_id,
                cs.payer_amount_sat,
                cs.receiver_amount_sat,
                cs.claim_address,
                cs.state,
                cs.pair_fees_json,
                cs.actual_payer_amount_sat,
                cs.accepted_receiver_amount_sat,
                rtx.amount_sat,
                pd.destination,
                pd.description,
                pd.lnurl_info_json
            FROM payment_tx_data AS ptx          -- Payment tx (each tx results in a Payment)
            FULL JOIN (
                SELECT * FROM receive_swaps
                WHERE 
                    COALESCE(claim_tx_id, lockup_tx_id, mrh_tx_id) IS NOT NULL
                    AND state NOT IN (0, 3, 4)   -- Ignore Created, Failed and TimedOut
            ) rs                                 -- Receive Swap data
                ON ptx.tx_id in (rs.claim_tx_id, rs.mrh_tx_id)
            FULL JOIN (
                SELECT * FROM chain_swaps
                WHERE 
                    COALESCE(user_lockup_tx_id, claim_tx_id) IS NOT NULL
                    AND state NOT IN (0, 4)      -- Ignore Created and TimedOut
            ) cs                                 -- Chain Swap data
                ON ptx.tx_id in (cs.user_lockup_tx_id, cs.claim_tx_id)
            LEFT JOIN send_swaps AS ss           -- Send Swap data
                ON ptx.tx_id = ss.lockup_tx_id
            LEFT JOIN payment_tx_data AS rtx     -- Refund tx data
                ON rtx.tx_id in (ss.refund_tx_id, cs.refund_tx_id)
            LEFT JOIN payment_details AS pd      -- Payment details
                ON pd.tx_id = ptx.tx_id
            WHERE                                
                (ptx.tx_id IS NULL               -- Filter out refund txs from Chain/Send Swaps
                    OR ptx.tx_id NOT IN (SELECT refund_tx_id FROM send_swaps WHERE refund_tx_id NOT NULL)
                    AND ptx.tx_id NOT IN (SELECT refund_tx_id FROM chain_swaps WHERE refund_tx_id NOT NULL))
            AND {}
            ORDER BY                             -- Order by swap creation time or tx timestamp (in case of direct tx)
                COALESCE(rs.created_at, ss.created_at, cs.created_at, ptx.timestamp) {}
            LIMIT {}
            OFFSET {}
            ",
            where_clause.unwrap_or("true"),
            match sort_ascending.unwrap_or(false) {
                true => "ASC",
                false => "DESC",
            },
            limit.unwrap_or(u32::MAX),
            offset.unwrap_or(0),
        )
    }

    fn sql_row_to_payment(&self, row: &Row) -> Result<Payment, rusqlite::Error> {
        let maybe_tx_tx_id: Result<String, rusqlite::Error> = row.get(0);
        let tx = match maybe_tx_tx_id {
            Ok(ref tx_id) => Some(PaymentTxData {
                tx_id: tx_id.to_string(),
                timestamp: row.get(1)?,
                amount_sat: row.get(2)?,
                fees_sat: row.get(3)?,
                payment_type: row.get(4)?,
                is_confirmed: row.get(5)?,
                unblinding_data: row.get(6)?,
            }),
            _ => None,
        };

        let maybe_receive_swap_id: Option<String> = row.get(7)?;
        let maybe_receive_swap_created_at: Option<u32> = row.get(8)?;
        let maybe_receive_swap_timeout_block_height: Option<u32> = row.get(9)?;
        let maybe_receive_swap_invoice: Option<String> = row.get(10)?;
        let maybe_receive_swap_payment_hash: Option<String> = row.get(11)?;
        let maybe_receive_swap_destination_pubkey: Option<String> = row.get(12)?;
        let maybe_receive_swap_description: Option<String> = row.get(13)?;
        let maybe_receive_swap_preimage: Option<String> = row.get(14)?;
        let maybe_receive_swap_payer_amount_sat: Option<u64> = row.get(15)?;
        let maybe_receive_swap_receiver_amount_sat: Option<u64> = row.get(16)?;
        let maybe_receive_swap_receiver_state: Option<PaymentState> = row.get(17)?;
        let maybe_receive_swap_pair_fees_json: Option<String> = row.get(18)?;
        let maybe_receive_swap_pair_fees: Option<ReversePair> =
            maybe_receive_swap_pair_fees_json.and_then(|pair| serde_json::from_str(&pair).ok());

        let maybe_send_swap_id: Option<String> = row.get(19)?;
        let maybe_send_swap_created_at: Option<u32> = row.get(20)?;
        let maybe_send_swap_timeout_block_height: Option<u32> = row.get(21)?;
        let maybe_send_swap_invoice: Option<String> = row.get(22)?;
        let maybe_send_swap_bolt12_offer: Option<String> = row.get(23)?;
        let maybe_send_swap_payment_hash: Option<String> = row.get(24)?;
        let maybe_send_swap_destination_pubkey: Option<String> = row.get(25)?;
        let maybe_send_swap_description: Option<String> = row.get(26)?;
        let maybe_send_swap_preimage: Option<String> = row.get(27)?;
        let maybe_send_swap_refund_tx_id: Option<String> = row.get(28)?;
        let maybe_send_swap_payer_amount_sat: Option<u64> = row.get(29)?;
        let maybe_send_swap_receiver_amount_sat: Option<u64> = row.get(30)?;
        let maybe_send_swap_state: Option<PaymentState> = row.get(31)?;
        let maybe_send_swap_pair_fees_json: Option<String> = row.get(32)?;
        let maybe_send_swap_pair_fees: Option<SubmarinePair> =
            maybe_send_swap_pair_fees_json.and_then(|pair| serde_json::from_str(&pair).ok());

        let maybe_chain_swap_id: Option<String> = row.get(33)?;
        let maybe_chain_swap_created_at: Option<u32> = row.get(34)?;
        let maybe_chain_swap_timeout_block_height: Option<u32> = row.get(35)?;
        let maybe_chain_swap_direction: Option<Direction> = row.get(36)?;
        let maybe_chain_swap_preimage: Option<String> = row.get(37)?;
        let maybe_chain_swap_description: Option<String> = row.get(38)?;
        let maybe_chain_swap_refund_tx_id: Option<String> = row.get(39)?;
        let maybe_chain_swap_payer_amount_sat: Option<u64> = row.get(40)?;
        let maybe_chain_swap_receiver_amount_sat: Option<u64> = row.get(41)?;
        let maybe_chain_swap_claim_address: Option<String> = row.get(42)?;
        let maybe_chain_swap_state: Option<PaymentState> = row.get(43)?;
        let maybe_chain_swap_pair_fees_json: Option<String> = row.get(44)?;
        let maybe_chain_swap_pair_fees: Option<ChainPair> =
            maybe_chain_swap_pair_fees_json.and_then(|pair| serde_json::from_str(&pair).ok());
        let maybe_chain_swap_actual_payer_amount_sat: Option<u64> = row.get(45)?;
        let maybe_chain_swap_accepted_receiver_amount_sat: Option<u64> = row.get(46)?;

        let maybe_swap_refund_tx_amount_sat: Option<u64> = row.get(47)?;

        let maybe_payment_details_destination: Option<String> = row.get(48)?;
        let maybe_payment_details_description: Option<String> = row.get(49)?;
        let maybe_payment_details_lnurl_info_json: Option<String> = row.get(50)?;
        let maybe_payment_details_lnurl_info: Option<LnUrlInfo> =
            maybe_payment_details_lnurl_info_json.and_then(|info| serde_json::from_str(&info).ok());

        let (swap, payment_type) = match maybe_receive_swap_id {
            Some(receive_swap_id) => {
                let payer_amount_sat = maybe_receive_swap_payer_amount_sat.unwrap_or(0);

                (
                    Some(PaymentSwapData {
                        swap_id: receive_swap_id,
                        swap_type: PaymentSwapType::Receive,
                        created_at: maybe_receive_swap_created_at.unwrap_or(utils::now()),
                        expiration_blockheight: maybe_receive_swap_timeout_block_height
                            .unwrap_or(0),
                        preimage: maybe_receive_swap_preimage,
                        invoice: maybe_receive_swap_invoice.clone(),
                        bolt12_offer: None, // Bolt12 not supported for Receive Swaps
                        payment_hash: maybe_receive_swap_payment_hash,
                        destination_pubkey: maybe_receive_swap_destination_pubkey,
                        description: maybe_receive_swap_description.unwrap_or_else(|| {
                            maybe_receive_swap_invoice
                                .and_then(|bolt11| get_invoice_description!(bolt11))
                                .unwrap_or("Lightning payment".to_string())
                        }),
                        payer_amount_sat,
                        receiver_amount_sat: maybe_receive_swap_receiver_amount_sat.unwrap_or(0),
                        swapper_fees_sat: maybe_receive_swap_pair_fees
                            .map(|pair| pair.fees.boltz(payer_amount_sat))
                            .unwrap_or(0),
                        refund_tx_id: None,
                        refund_tx_amount_sat: None,
                        claim_address: None,
                        status: maybe_receive_swap_receiver_state.unwrap_or(PaymentState::Created),
                    }),
                    PaymentType::Receive,
                )
            }
            None => match maybe_send_swap_id {
                Some(send_swap_id) => {
                    let receiver_amount_sat = maybe_send_swap_receiver_amount_sat.unwrap_or(0);
                    (
                        Some(PaymentSwapData {
                            swap_id: send_swap_id,
                            swap_type: PaymentSwapType::Send,
                            created_at: maybe_send_swap_created_at.unwrap_or(utils::now()),
                            expiration_blockheight: maybe_send_swap_timeout_block_height
                                .unwrap_or(0),
                            preimage: maybe_send_swap_preimage,
                            invoice: maybe_send_swap_invoice,
                            bolt12_offer: maybe_send_swap_bolt12_offer,
                            payment_hash: maybe_send_swap_payment_hash,
                            destination_pubkey: maybe_send_swap_destination_pubkey,
                            description: maybe_send_swap_description
                                .unwrap_or("Lightning payment".to_string()),
                            payer_amount_sat: maybe_send_swap_payer_amount_sat.unwrap_or(0),
                            receiver_amount_sat,
                            swapper_fees_sat: maybe_send_swap_pair_fees
                                .map(|pair| pair.fees.boltz(receiver_amount_sat))
                                .unwrap_or(0),
                            refund_tx_id: maybe_send_swap_refund_tx_id,
                            refund_tx_amount_sat: maybe_swap_refund_tx_amount_sat,
                            claim_address: None,
                            status: maybe_send_swap_state.unwrap_or(PaymentState::Created),
                        }),
                        PaymentType::Send,
                    )
                }
                None => match maybe_chain_swap_id {
                    Some(chain_swap_id) => {
                        let payer_amount_sat = match maybe_chain_swap_actual_payer_amount_sat {
                            Some(actual_payer_amount_sat) => actual_payer_amount_sat,
                            None => maybe_chain_swap_payer_amount_sat.unwrap_or(0),
                        };
                        let receiver_amount_sat =
                            match maybe_chain_swap_accepted_receiver_amount_sat {
                                Some(accepted_receiver_amount_sat) => accepted_receiver_amount_sat,
                                None => match (
                                    maybe_chain_swap_actual_payer_amount_sat,
                                    maybe_chain_swap_payer_amount_sat,
                                ) {
                                    (Some(actual), Some(0)) => actual, // For amountless chain swaps WaitingFeeAcceptance, show zero fees
                                    _ => maybe_chain_swap_receiver_amount_sat.unwrap_or(0),
                                },
                            };
                        let swapper_fees_sat = maybe_chain_swap_pair_fees
                            .map(|pair| pair.fees.percentage)
                            .map(|fr| ((fr / 100.0) * payer_amount_sat as f64).ceil() as u64)
                            .unwrap_or(0);

                        (
                            Some(PaymentSwapData {
                                swap_id: chain_swap_id,
                                swap_type: PaymentSwapType::Chain,
                                created_at: maybe_chain_swap_created_at.unwrap_or(utils::now()),
                                expiration_blockheight: maybe_chain_swap_timeout_block_height
                                    .unwrap_or(0),
                                preimage: maybe_chain_swap_preimage,
                                invoice: None,
                                bolt12_offer: None, // Bolt12 not supported for Chain Swaps
                                payment_hash: None,
                                destination_pubkey: None,
                                description: maybe_chain_swap_description
                                    .unwrap_or("Bitcoin transfer".to_string()),
                                payer_amount_sat,
                                receiver_amount_sat,
                                swapper_fees_sat,
                                refund_tx_id: maybe_chain_swap_refund_tx_id,
                                refund_tx_amount_sat: maybe_swap_refund_tx_amount_sat,
                                claim_address: maybe_chain_swap_claim_address,
                                status: maybe_chain_swap_state.unwrap_or(PaymentState::Created),
                            }),
                            maybe_chain_swap_direction
                                .unwrap_or(Direction::Outgoing)
                                .into(),
                        )
                    }
                    None => (None, PaymentType::Send),
                },
            },
        };

        let description = swap.as_ref().map(|s| s.description.clone());
        let payment_details = match swap.clone() {
            Some(
                PaymentSwapData {
                    swap_type: PaymentSwapType::Receive,
                    swap_id,
                    invoice,
                    bolt12_offer,
                    payment_hash,
                    destination_pubkey,
                    refund_tx_id,
                    preimage,
                    refund_tx_amount_sat,
                    expiration_blockheight,
                    ..
                }
                | PaymentSwapData {
                    swap_type: PaymentSwapType::Send,
                    swap_id,
                    invoice,
                    bolt12_offer,
                    payment_hash,
                    destination_pubkey,
                    preimage,
                    refund_tx_id,
                    refund_tx_amount_sat,
                    expiration_blockheight,
                    ..
                },
            ) => PaymentDetails::Lightning {
                swap_id,
                preimage,
                invoice: invoice.clone(),
                bolt12_offer: bolt12_offer.clone(),
                payment_hash,
                destination_pubkey: destination_pubkey.or_else(|| {
                    invoice.and_then(|invoice| {
                        utils::get_invoice_destination_pubkey(&invoice, bolt12_offer.is_some()).ok()
                    })
                }),
                lnurl_info: maybe_payment_details_lnurl_info,
                refund_tx_id,
                refund_tx_amount_sat,
                description: description.unwrap_or("Lightning transfer".to_string()),
                liquid_expiration_blockheight: expiration_blockheight,
            },
            Some(PaymentSwapData {
                swap_type: PaymentSwapType::Chain,
                swap_id,
                refund_tx_id,
                refund_tx_amount_sat,
                expiration_blockheight,
                ..
            }) => {
                let (bitcoin_expiration_blockheight, liquid_expiration_blockheight) =
                    match maybe_chain_swap_direction {
                        Some(Direction::Incoming) => (Some(expiration_blockheight), None),
                        Some(Direction::Outgoing) | None => (None, Some(expiration_blockheight)),
                    };

                PaymentDetails::Bitcoin {
                    swap_id,
                    refund_tx_id,
                    refund_tx_amount_sat,
                    description: description.unwrap_or("Bitcoin transfer".to_string()),
                    liquid_expiration_blockheight,
                    bitcoin_expiration_blockheight,
                }
            }
            _ => PaymentDetails::Liquid {
                destination: maybe_payment_details_destination
                    .unwrap_or("Destination unknown".to_string()),
                description: maybe_payment_details_description
                    .unwrap_or("Liquid transfer".to_string()),
            },
        };

        match (tx, swap.clone()) {
            (None, None) => Err(maybe_tx_tx_id.err().unwrap()),
            (None, Some(swap)) => Ok(Payment::from_pending_swap(
                swap,
                payment_type,
                payment_details,
            )),
            (Some(tx), None) => Ok(Payment::from_tx_data(tx, None, payment_details)),
            (Some(tx), Some(swap)) => Ok(Payment::from_tx_data(tx, Some(swap), payment_details)),
        }
    }

    pub fn get_payment(&self, id: &str) -> Result<Option<Payment>> {
        Ok(self
            .get_connection()?
            .query_row(
                &self.select_payment_query(
                    Some("(ptx.tx_id = ?1 OR COALESCE(rs.id, ss.id, cs.id) = ?1)"),
                    None,
                    None,
                    None,
                ),
                params![id],
                |row| self.sql_row_to_payment(row),
            )
            .optional()?)
    }

    pub fn get_payment_by_request(&self, req: &GetPaymentRequest) -> Result<Option<Payment>> {
        let (where_clause, param) = match req {
            GetPaymentRequest::Lightning { payment_hash } => (
                "(rs.payment_hash = ?1 OR ss.payment_hash = ?1)",
                payment_hash,
            ),
        };
        Ok(self
            .get_connection()?
            .query_row(
                &self.select_payment_query(Some(where_clause), None, None, None),
                params![param],
                |row| self.sql_row_to_payment(row),
            )
            .optional()?)
    }

    pub fn get_payments(&self, req: &ListPaymentsRequest) -> Result<Vec<Payment>> {
        let (where_clause, where_params) = filter_to_where_clause(req);
        let maybe_where_clause = match where_clause.is_empty() {
            false => Some(where_clause.as_str()),
            true => None,
        };

        // Assumes there is no swap chaining (send swap lockup tx = receive swap claim tx)
        let con = self.get_connection()?;
        let mut stmt = con.prepare(&self.select_payment_query(
            maybe_where_clause,
            req.offset,
            req.limit,
            req.sort_ascending,
        ))?;
        let payments: Vec<Payment> = stmt
            .query_map(params_from_iter(where_params), |row| {
                self.sql_row_to_payment(row)
            })?
            .map(|i| i.unwrap())
            .collect();
        Ok(payments)
    }

    pub fn get_payments_by_tx_id(
        &self,
        req: &ListPaymentsRequest,
    ) -> Result<HashMap<String, Payment>> {
        let res: HashMap<String, Payment> = self
            .get_payments(req)?
            .into_iter()
            .flat_map(|payment| {
                // Index payments by both tx_id (lockup/claim) and refund_tx_id
                let mut res = vec![];
                if let Some(tx_id) = payment.tx_id.clone() {
                    res.push((tx_id, payment.clone()));
                }
                if let Some(refund_tx_id) = payment.get_refund_tx_id() {
                    res.push((refund_tx_id, payment));
                }
                res
            })
            .collect();
        Ok(res)
    }
}

fn filter_to_where_clause(req: &ListPaymentsRequest) -> (String, Vec<Box<dyn ToSql + '_>>) {
    let mut where_clause: Vec<String> = Vec::new();
    let mut where_params: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(t) = req.from_timestamp {
        where_clause.push("coalesce(ptx.timestamp, rs.created_at) >= ?".to_string());
        where_params.push(Box::new(t));
    };
    if let Some(t) = req.to_timestamp {
        where_clause.push("coalesce(ptx.timestamp, rs.created_at) <= ?".to_string());
        where_params.push(Box::new(t));
    };

    if let Some(filters) = &req.filters {
        if !filters.is_empty() {
            let mut type_filter_clause: HashSet<i8> = HashSet::new();

            for type_filter in filters {
                type_filter_clause.insert(*type_filter as i8);
            }

            where_clause.push(format!(
                "ptx.payment_type in ({})",
                type_filter_clause
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    if let Some(states) = &req.states {
        if !states.is_empty() {
            let states_hash: HashSet<i8> = HashSet::from_iter(states.iter().map(|s| *s as i8));
            where_clause.push(format!(
                "COALESCE(rs.state, ss.state, cs.state) in ({})",
                states_hash
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    if let Some(details) = &req.details {
        match details {
            ListPaymentDetails::Bitcoin { address } => {
                // Use the lockup address if it's incoming, else use the claim address
                where_clause.push(
                    "(cs.direction = 0 AND cs.lockup_address = ? OR cs.direction = 1 AND cs.claim_address = ?)"
                        .to_string(),
                );
                where_params.push(Box::new(address));
                where_params.push(Box::new(address));
            }
            ListPaymentDetails::Liquid { destination } => {
                where_clause.push("pd.destination = ?".to_string());
                where_params.push(Box::new(destination));
            }
        }
    }

    (where_clause.join(" and "), where_params)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{
        persist::PaymentTxDetails,
        prelude::ListPaymentsRequest,
        test_utils::persist::{
            create_persister, new_payment_tx_data, new_receive_swap, new_send_swap,
        },
    };

    use super::{PaymentState, PaymentType};

    #[test]
    fn test_get_payments() -> Result<()> {
        create_persister!(storage);

        let payment_tx_data = new_payment_tx_data(PaymentType::Send);
        storage.insert_or_update_payment(
            payment_tx_data.clone(),
            Some(PaymentTxDetails {
                destination: "mock-address".to_string(),
                ..Default::default()
            }),
            false,
        )?;

        assert!(storage
            .get_payments(&ListPaymentsRequest {
                ..Default::default()
            })?
            .first()
            .is_some());
        assert!(storage.get_payment(&payment_tx_data.tx_id)?.is_some());

        Ok(())
    }

    #[test]
    fn test_list_ongoing_swaps() -> Result<()> {
        create_persister!(storage);

        storage.insert_or_update_send_swap(&new_send_swap(None))?;
        storage.insert_or_update_receive_swap(&new_receive_swap(Some(PaymentState::Pending)))?;

        assert_eq!(storage.list_ongoing_swaps()?.len(), 2);

        Ok(())
    }
}
