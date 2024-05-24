use std::borrow::Cow::{self, Owned};
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use breez_liquid_sdk::model::*;
use breez_liquid_sdk::sdk::LiquidSdk;
use clap::{arg, Parser};
use qrcode_rs::render::unicode;
use qrcode_rs::{EcLevel, QrCode};
use rustyline::highlight::Highlighter;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use rustyline::{hint::HistoryHinter, Completer, Helper, Hinter, Validator};

use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Parser, Debug, Clone, PartialEq)]
pub(crate) enum Command {
    /// Send lbtc and receive btc through a swap
    SendPayment {
        /// Invoice which has to be paid
        bolt11: String,

        /// Delay for the send, in seconds
        #[arg(short, long)]
        delay: Option<u64>,
    },
    /// Receive lbtc and send btc through a swap
    ReceivePayment {
        /// Amount the payer will send, in satoshi
        payer_amount_sat: u64,
    },
    /// List incoming and outgoing payments
    ListPayments,
    /// Get the balance and general info of the current instance
    GetInfo,
    /// Sync local data with mempool and onchain data
    Sync,
    /// Empties the encrypted transaction cache
    EmptyCache,
    /// Backs up the current pending swaps
    Backup {
        #[arg(short, long)]
        backup_path: Option<String>,
    },
    /// Retrieve a list of backups
    Restore {
        #[arg(short, long)]
        backup_path: Option<String>,
    },
}

#[derive(Helper, Completer, Hinter, Validator)]
pub(crate) struct CliHelper {
    #[rustyline(Hinter)]
    pub(crate) hinter: HistoryHinter,
}

impl Highlighter for CliHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }
}

#[derive(Serialize)]
pub(crate) struct CommandResult<T: Serialize> {
    pub success: bool,
    pub message: T,
}

macro_rules! command_result {
    ($expr:expr) => {{
        to_string_pretty(&CommandResult {
            success: true,
            message: $expr,
        })?
    }};
}

macro_rules! wait_confirmation {
    ($prompt:expr,$result:expr) => {
        print!("{}", $prompt);
        std::io::stdout().flush()?;

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        if !['y', 'Y'].contains(&(buf.as_bytes()[0] as char)) {
            return Ok(command_result!($result));
        }
    };
}

pub(crate) async fn handle_command(
    _rl: &mut Editor<CliHelper, DefaultHistory>,
    sdk: &Arc<LiquidSdk>,
    command: Command,
) -> Result<String> {
    Ok(match command {
        Command::ReceivePayment { payer_amount_sat } => {
            let prepare_response =
                sdk.prepare_receive_payment(&PrepareReceiveRequest { payer_amount_sat })?;

            wait_confirmation!(
                format!(
                    "Fees: {} sat. Are the fees acceptable? (y/N) ",
                    prepare_response.fees_sat
                ),
                "Payment receive halted"
            );

            let response = sdk.receive_payment(&prepare_response)?;
            let invoice = response.invoice.clone();

            let mut result = command_result!(response);
            result.push('\n');
            result.push_str(&build_qr_text(&invoice));
            result
        }
        Command::SendPayment { bolt11, delay } => {
            let prepare_response = sdk
                .prepare_send_payment(&PrepareSendRequest { invoice: bolt11 })
                .await?;

            wait_confirmation!(
                format!(
                    "Fees: {} sat. Are the fees acceptable? (y/N) ",
                    prepare_response.fees_sat
                ),
                "Payment send halted"
            );

            if let Some(delay) = delay {
                let sdk_cloned = sdk.clone();
                let prepare_cloned = prepare_response.clone();

                tokio::spawn(async move {
                    thread::sleep(Duration::from_secs(delay));
                    sdk_cloned.send_payment(&prepare_cloned).await.unwrap();
                });
                command_result!(prepare_response)
            } else {
                let response = sdk.send_payment(&prepare_response).await?;
                command_result!(response)
            }
        }
        Command::GetInfo => {
            command_result!(sdk.get_info(GetInfoRequest { with_scan: true }).await?)
        }
        Command::ListPayments => {
            let payments = sdk.list_payments()?;
            command_result!(payments)
        }
        Command::Sync => {
            sdk.sync().await?;
            command_result!("Synced successfully")
        }
        Command::EmptyCache => {
            sdk.empty_wallet_cache()?;
            command_result!("Cache emptied successfully")
        }
        Command::Backup { backup_path } => {
            sdk.backup(BackupRequest { backup_path })?;
            command_result!("Backup created successfully!")
        }
        Command::Restore { backup_path } => {
            sdk.restore(RestoreRequest { backup_path })?;
            command_result!("Backup restored successfully!")
        }
    })
}

fn build_qr_text(text: &str) -> String {
    QrCode::with_error_correction_level(text, EcLevel::L)
        .unwrap()
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build()
}
