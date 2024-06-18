use std::borrow::Cow::{self, Owned};
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use breez_liquid_sdk::model::*;
use breez_liquid_sdk::sdk::LiquidSdk;
use breez_liquid_sdk::*;
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
    /// Shuts down all background threads of this SDK instance
    Disconnect,
    /// Parse a generic string to get its type and relevant metadata
    Parse {
        /// Generic input (URL, LNURL, BIP-21 BTC Address, LN invoice, etc)
        input: String,
    },
    /// Pay using LNURL
    LnurlPay {
        /// LN Address or LNURL-pay endpoint
        lnurl: String,
    },
    LnurlWithdraw {
        /// LNURL-withdraw endpoint
        lnurl: String,
    },
    LnurlAuth {
        /// LNURL-auth endpoint
        lnurl: String,
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
    rl: &mut Editor<CliHelper, DefaultHistory>,
    sdk: &Arc<LiquidSdk>,
    command: Command,
) -> Result<String> {
    Ok(match command {
        Command::ReceivePayment { payer_amount_sat } => {
            let prepare_response = sdk
                .prepare_receive_payment(&PrepareReceiveRequest { payer_amount_sat })
                .await?;

            wait_confirmation!(
                format!(
                    "Fees: {} sat. Are the fees acceptable? (y/N) ",
                    prepare_response.fees_sat
                ),
                "Payment receive halted"
            );

            let response = sdk.receive_payment(&prepare_response).await?;
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
            command_result!(sdk.get_info().await?)
        }
        Command::ListPayments => {
            let payments = sdk.list_payments().await?;
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
        Command::Disconnect => {
            sdk.disconnect().await?;
            command_result!("Liquid SDK instance disconnected")
        }
        Command::Parse { input } => {
            let res = LiquidSdk::parse(&input).await?;
            command_result!(res)
        }
        Command::LnurlPay { lnurl } => {
            let input = LiquidSdk::parse(&lnurl).await?;
            let res = match input {
                InputType::LnUrlPay { data: pd } => {
                    let prompt = format!(
                        "Amount to pay in millisatoshi (min {} msat, max {} msat): ",
                        pd.min_sendable, pd.max_sendable
                    );

                    let amount_msat = rl.readline(&prompt)?;
                    let pay_res = sdk
                        .lnurl_pay(LnUrlPayRequest {
                            data: pd,
                            amount_msat: amount_msat.parse::<u64>()?,
                            comment: None,
                            payment_label: None,
                        })
                        .await?;
                    Ok(pay_res)
                }
                _ => Err(anyhow::anyhow!("Invalid input")),
            }?;

            command_result!(res)
        }
        Command::LnurlWithdraw { lnurl } => {
            let input = LiquidSdk::parse(&lnurl).await?;
            let res = match input {
                InputType::LnUrlWithdraw { data: pd } => {
                    let prompt = format!(
                        "Amount to withdraw in millisatoshi (min {} msat, max {} msat): ",
                        pd.min_withdrawable, pd.max_withdrawable
                    );

                    let amount_msat = rl.readline(&prompt)?;
                    let withdraw_res = sdk
                        .lnurl_withdraw(LnUrlWithdrawRequest {
                            data: pd,
                            amount_msat: amount_msat.parse()?,
                            description: Some("LNURL-withdraw".to_string()),
                        })
                        .await?;
                    Ok(withdraw_res)
                }
                _ => Err(anyhow::anyhow!("Invalid input")),
            }?;

            command_result!(res)
        }
        Command::LnurlAuth { lnurl } => {
            let lnurl_endpoint = lnurl.trim();

            let res = match parse(lnurl_endpoint).await? {
                InputType::LnUrlAuth { data: ad } => {
                    let auth_res = sdk.lnurl_auth(ad).await?;
                    serde_json::to_string_pretty(&auth_res).map_err(|e| e.into())
                }
                _ => Err(anyhow::anyhow!("Unexpected result type")),
            }?;

            command_result!(res)
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
