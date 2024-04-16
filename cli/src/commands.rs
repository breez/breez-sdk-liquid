use std::borrow::Cow::{self, Owned};
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use clap::{arg, Parser};
use ls_sdk::{PrepareReceiveRequest, Wallet};
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
        bolt11: String,

        /// Delay for the send, in seconds
        #[arg(short, long)]
        delay: Option<u64>,
    },
    /// Receive lbtc and send btc through a swap
    ReceivePayment {
        #[arg(short, long)]
        receiver_amount_sat: Option<u64>,

        #[arg(short, long)]
        payer_amount_sat: Option<u64>,
    },
    /// List incoming and outgoing payments
    ListPayments,
    /// Get the balance of the currently loaded wallet
    GetInfo,
    /// Empties the encrypted wallet transaction cache
    EmptyCache,
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

pub(crate) fn handle_command(
    _rl: &mut Editor<CliHelper, DefaultHistory>,
    wallet: &Arc<Wallet>,
    command: Command,
) -> Result<String> {
    Ok(match command {
        Command::ReceivePayment {
            receiver_amount_sat,
            payer_amount_sat,
        } => {
            let prepare_response = wallet.prepare_receive_payment(&PrepareReceiveRequest {
                payer_amount_sat,
                receiver_amount_sat,
            })?;

            wait_confirmation!(
                format!(
                    "Fees: {} sat. Are the fees acceptable? (y/N) ",
                    prepare_response.fees_sat
                ),
                "Payment receive halted"
            );

            let response = wallet.receive_payment(&prepare_response)?;
            let invoice = response.invoice.clone();

            let mut result = command_result!(response);
            result.push('\n');
            result.push_str(&build_qr_text(&invoice));
            result
        }
        Command::SendPayment { bolt11, delay } => {
            let prepare_response = wallet.prepare_send_payment(&bolt11)?;
            let fees = (prepare_response.onchain_amount_sat + prepare_response.network_fees)
                - prepare_response.invoice_amount_sat;

            wait_confirmation!(
                format!("Fees: {fees} sat. Are the fees acceptable? (y/N) "),
                "Payment send halted"
            );

            if let Some(delay) = delay {
                let wallet_cloned = wallet.clone();
                let prepare_cloned = prepare_response.clone();

                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(delay));
                    wallet_cloned.send_payment(&prepare_cloned).unwrap();
                });
                command_result!(prepare_response)
            } else {
                let response = wallet.send_payment(&prepare_response)?;
                command_result!(response)
            }
        }
        Command::GetInfo => {
            command_result!(wallet.get_info(true)?)
        }
        Command::ListPayments => {
            let mut payments = wallet.list_payments(true, true)?;
            payments.reverse();
            command_result!(payments)
        }
        Command::EmptyCache => {
            wallet.empty_wallet_cache()?;
            command_result!("Cache emptied successfully")
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
