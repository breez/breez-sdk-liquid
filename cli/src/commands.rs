use std::borrow::Cow::{self, Owned};

use anyhow::Result;
use clap::Parser;
use lwk_signer::AnySigner;
use rustyline::highlight::Highlighter;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use rustyline::{hint::HistoryHinter, Completer, Helper, Hinter, Validator};

use breez_sdk_liquid::BreezWollet;

#[derive(Parser, Debug, Clone, PartialEq)]
pub(crate) enum Command {
    /// Send lbtc and receive btc through a swap
    Send { amount_sat: u64, address: String },
    /// Receive lbtc and send btc through a swap
    Receive,
    /// Get the first fungible address of the currently loaded wallet
    GetAddress,
    /// Get the balance of the currently loaded wallet
    GetBalance,
    /// Hangs the current thread until the wallet's balance has changed
    AwaitBalance,
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
    wollet: &mut BreezWollet,
    command: Command,
) -> Result<String> {
    match command {
        Command::Receive {} => {
            let address = wollet.address(None)?;
            println!("Please send your liquid funds to the following address: {address}");

            let new_balance = wollet.wait_balance_change()?;

            Ok(format!(
                "Funding successful! New balance: {new_balance} sat"
            ))
        }
        Command::Send {
            amount_sat,
            address,
        } => {
            let signer = AnySigner::Software(wollet.get_signer());
            let txid = wollet.sign_and_send(&[signer], None, &address, amount_sat)?;

            Ok(format!(
                r#"
                Succesffully sent {amount_sat} to {address}.
                You can view the transaction at https://blockstream.info/liquidtestnet/tx/{}"#,
                txid
            ))
        }
        Command::GetAddress {} => Ok(format!(
            "Here's the main funding address for your wallet: {}",
            wollet.address(None)?
        )),
        Command::GetBalance {} => Ok(format!(
            "Current balance: {} sat",
            wollet.total_balance_sat()?
        )),
        Command::AwaitBalance {} => {
            println!("Waiting for balance changes...");
            let old_balance = wollet.total_balance_sat()?;
            Ok(format!(
                "Balance has changed! Old balance: {} sat, New balance: {} sat",
                old_balance,
                wollet.wait_balance_change()?
            ))
        }
    }
}
