use std::borrow::Cow::{self, Owned};
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use rustyline::highlight::Highlighter;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use rustyline::{hint::HistoryHinter, Completer, Helper, Hinter, Validator};

use breez_sdk_liquid::Wallet;

#[derive(Parser, Debug, Clone, PartialEq)]
pub(crate) enum Command {
    /// Send lbtc and receive btc through a swap
    SendPayment { bolt11: String },
    /// Receive lbtc and send btc through a swap
    ReceivePayment { amount_sat: u64 },
    /// List incoming and outgoing payments
    ListPayments,
    /// Get the balance of the currently loaded wallet
    GetInfo,
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

pub(crate) async fn handle_command(
    _rl: &mut Editor<CliHelper, DefaultHistory>,
    wallet: &Arc<Wallet>,
    command: Command,
) -> Result<String> {
    match command {
        Command::ReceivePayment { amount_sat } => {
            let response = wallet.receive_payment(amount_sat)?;
            dbg!(&response);
            Ok(format!(
                "Please pay the following invoice: {}",
                response.invoice
            ))
        }
        Command::SendPayment { bolt11 } => {
            let response = wallet.send_payment(&bolt11)?;

            Ok(format!(
                r#"
                Successfully paid the invoice!
                You can view the onchain transaction at https://blockstream.info/liquidtestnet/tx/{}"#,
                response.txid
            ))
        }
        Command::GetInfo => {
            let info = wallet.get_info(true)?;

            Ok(format!(
                "Current Balance: {} sat\nPublic Key: {}\nLiquid Address: {}",
                info.balance_sat, info.pubkey, info.active_address
            ))
        }
        Command::ListPayments => {
            let payments_str = wallet
                .list_payments(true, true)?
                .iter()
                .map(|tx| {
                    format!(
                        "Id: {} | Type: {} | Amount: {} sat | Timestamp: {}",
                        tx.id.clone().unwrap_or("None".to_string()),
                        tx.payment_type.to_string(),
                        tx.amount_sat,
                        match tx.timestamp {
                            Some(t) => t.to_string(),
                            None => "None".to_string(),
                        },
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            Ok(payments_str)
        }
    }
}
