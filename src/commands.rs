use std::thread;
use std::time::Duration;
use std::borrow::Cow::{self, Owned};

use clap::Parser;
use log::debug;
use rustyline::Editor;
use lwk_common::Signer;
use anyhow::{Result, anyhow};
use rustyline::highlight::Highlighter;
use rustyline::history::DefaultHistory;
use rustyline::{Helper, Completer, Hinter, Validator, hint::HistoryHinter};
use lwk_wollet::{Wollet, full_scan_with_electrum_client, ElectrumClient, BlockchainBackend};

use crate::context::CliCtx;

#[derive(Parser, Debug, Clone, PartialEq)]
pub(crate) enum Command {
    /// Send lbtc and receive btc through a swap
    Send { amount_sat: u64, address: String },
    /// Receive lbtc and send btc through a swap
    Receive,
    /// Get the first fungible address of the currently loaded wallet
    GetAddress,
    /// Get the balance of the currently loaded wallet
    GetBalance
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

fn get_balance_sat(wollet: &mut Wollet, electrum: &mut ElectrumClient) -> Result<u64> {
      full_scan_with_electrum_client(wollet, electrum)?;
      Ok(wollet.balance()?.values().sum())
}

fn poll_balance_changes(wollet: &mut Wollet, electrum: &mut ElectrumClient) -> Result<u64> {
    let current_balance = get_balance_sat(wollet, electrum)?;
    let mut balance = 0u64;

    while balance <= current_balance {
      debug!("Polling for balance changes...");
      balance = get_balance_sat(wollet, electrum)?;
      thread::sleep(Duration::from_secs(5));
    }

    Ok(balance)
}

pub(crate) async fn handle_command(
    _rl: &mut Editor<CliHelper, DefaultHistory>,
    ctx: &mut CliCtx,
    command: Command,
) -> Result<String> {
    match command {
        Command::Receive {  } => {
          let address = ctx.wollet.address(None)?.address().to_string();
          println!("Please send your liquid funds to the following address: {address}");

          let new_balance = poll_balance_changes(&mut ctx.wollet, &mut ctx.electrum_client)?;

          Ok(format!("Funding successful! New balance: {new_balance} sat"))
        },
        Command::Send { amount_sat, address } => {
          let balance = get_balance_sat(&mut ctx.wollet, &mut ctx.electrum_client)?;
          if amount_sat > balance {
            return Err(anyhow!("You cannot send more than your balance ({balance} sat)"))
          }

          let mut pset = ctx.wollet.send_lbtc(amount_sat, &address, None)?;
          ctx.signer.sign(&mut pset)?;

          let tx = ctx.wollet.finalize(&mut pset)?;
          ctx.electrum_client.broadcast(&tx)?;
          
          Ok(
            format!(r#"
              Succesffully sent {amount_sat} to {address}.
              You can view the transaction at https://blockstream.info/liquidtestnet/tx/{}"#,
              tx.txid().to_string()
            )
          )
        },
        Command::GetAddress {  } => {
          Ok(format!("Here's the main funding address for your wallet: {}", ctx.wollet.address(None)?.address().to_string()))
          
        },
        Command::GetBalance {  } => {
          Ok(format!("Current balance: {} sat", get_balance_sat(&mut ctx.wollet, &mut ctx.electrum_client)?))
        }
    }
}
