use std::borrow::Cow::{self, Owned};
use std::sync::Arc;

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
    SendPayment { amount_sat: u64, address: String },
    /// Receive lbtc and send btc through a swap
    ReceivePayment { amount_sat: u64 },
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

pub(crate) async fn handle_command(
    _rl: &mut Editor<CliHelper, DefaultHistory>,
    wollet: &Arc<BreezWollet>,
    command: Command,
) -> Result<String> {
    match command {
        Command::ReceivePayment { amount_sat } => {
            // let address = wollet.address(None).await?;
            // println!("Please send your liquid funds to the following address: {address}");
            //
            // let new_balance = wollet.wait_balance_change().await?;
            //
            // Ok(format!(
            //     "Funding successful! New balance: {new_balance} sat"
            // ))
            
            let response = wollet.receive_payment(amount_sat).await?;

            dbg!(&response);
            println!("Please pay the following invoice: {}", response.invoice);

            let new_balance_sat = wollet.wait_balance_change().await?;

            Ok(format!(
                r#"
                Wollet funded successfully! New balance: {} sat
                "#,
               new_balance_sat 
            ))
        }
        Command::SendPayment {
            amount_sat,
            address,
        } => {
            let signer = AnySigner::Software(wollet.get_signer());
            let txid = wollet.sign_and_send(&[signer], None, &address, amount_sat).await?;

            Ok(format!(
                r#"
                Succesffully sent {amount_sat} to {address}.
                You can view the transaction at https://blockstream.info/liquidtestnet/tx/{}"#,
                txid
            ))
        }
        Command::GetAddress {} => Ok(format!(
            "Here's the main funding address for your wallet: {}",
            wollet.address(None).await?
        )),
        Command::GetBalance {} => Ok(format!(
            "Current balance: {} sat",
            wollet.total_balance_sat(true).await?
        )),
    }
}
