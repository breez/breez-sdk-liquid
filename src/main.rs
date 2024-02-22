mod persist;
mod commands;
mod wollet;

use anyhow::{anyhow, Result};
use lwk_common::{Singlesig, singlesig_desc};
use lwk_signer::SwSigner;
use lwk_wollet::ElectrumUrl;
use std::{path::PathBuf, fs};

use clap::Parser;
use log::{info,error};
use rustyline::{Editor, hint::HistoryHinter, error::ReadlineError};

use persist::CliPersistence;
use wollet::{Wollet, WolletOptions};
use commands::{CliHelper, Command, handle_command};

const BLOCKSTREAM_ELECTRUM_URL: &str = "blockstream.info:465";

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[clap(name = "data_dir", short = 'd', long = "data_dir")]
    pub(crate) data_dir: Option<String>
}

fn show_results(res: Result<String>) {
    match res {
        Ok(inner) => println!("{inner}"),
        Err(err) => eprintln!("Error: {err}"),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let data_dir = args.data_dir.unwrap_or(".data".to_string());
    let data_dir = PathBuf::from(&data_dir);

    fs::create_dir_all(&data_dir)?;

    let persistence = CliPersistence { 
        data_dir
    };
    let history_file = &persistence.history_file();

    let rl = &mut Editor::new()?;
    rl.set_helper(Some(CliHelper {
        hinter: HistoryHinter {}
    }));

    if rl.load_history(history_file).is_err() {
        info!("No history found");
    }

    let mnemonic = persistence.get_or_create_mnemonic()?;
    let signer = SwSigner::new(&mnemonic.to_string(), false)?;
    let desc = singlesig_desc(
        &signer, 
        Singlesig::Wpkh, 
        lwk_common::DescriptorBlindingKey::Elip151, 
        false
    ).expect("Expected valid descriptor");
    let electrum_url = ElectrumUrl::new(BLOCKSTREAM_ELECTRUM_URL, true, true);

    let mut wollet = Wollet::new(WolletOptions {
        signer,
        network: lwk_wollet::ElementsNetwork::LiquidTestnet,
        electrum_url,
        desc,
        db_root_dir: None
    })?;

    loop {
        let readline = rl.readline("breez-liquid> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let mut vec: Vec<&str> = line.as_str().split_whitespace().collect();
                vec.insert(0, "");
                let cli_res = Command::try_parse_from(vec);
                if cli_res.is_err() {
                    println!("{}", cli_res.unwrap_err());
                    continue;
                }
                let res = handle_command(rl, &mut wollet, cli_res.unwrap()).await;
                show_results(res);
            },
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                break;
            }
            Err(err) => {
                error!("Error: {:?}", err);
                break;
            }
        }
    }
    
    rl.save_history(history_file).map_err(|e| anyhow!(e))
}

#[cfg(test)]
mod tests {
    use std::{process::{Command, Stdio, ChildStdin, ChildStdout}, io::{BufReader, Write, BufRead, self}};

    use anyhow::{Result, anyhow};
    use boltz_client::{swaps::boltz::{BOLTZ_TESTNET_URL, BoltzApiClient, CreateSwapRequest}, Bolt11Invoice};

    fn run_command(command: &str, input: &mut ChildStdin, reader: &mut BufReader<ChildStdout>) -> Result<()> {
        input.write_all(command.as_bytes())?;

        let mut line = String::new();
        reader.read_line(&mut line)?;

        println!("{line}");

        Ok(())
    }

    #[derive(thiserror::Error, Debug)]
    enum TestError {
        #[error("Could not contact Boltz servers")]
        ServersUnreachable
    }

    #[tokio::test]
    async fn normal_submarine_swap() -> Result<()> {
        let client = BoltzApiClient::new(BOLTZ_TESTNET_URL);
        let pairs = client.get_pairs()
            .map_err(|_| TestError::ServersUnreachable)?;

        let mut invoice = String::new();
        println!("Please paste the invoice to be paid: ");
        io::stdin().read_line(&mut invoice)?;

        let invoice = invoice.trim().parse::<Bolt11Invoice>()?;
        let amount_sat = invoice.amount_milli_satoshis().expect("Expected an invoice with an amount.") / 1000;

        let pair = pairs.get_lbtc_pair()
            .map_err(|_| TestError::ServersUnreachable)?;

        if pair.limits.minimal > amount_sat as i64 {
            return Err(anyhow!("Invoice amount is too low (minimum {} sat)", pair.limits.minimal));
        }

        let fees_sat = pair.fees
            .reverse_total(amount_sat)
            .map_err(|_| TestError::ServersUnreachable)?;

        println!("The total amount with fee subtraction will be {} sat. Is this ok (Y/n)? ", amount_sat - fees_sat);

        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;

        if vec!["n", "no", "N", "No", "NO"].iter().any(|c| confirmation.eq(c)) {
            return Err(anyhow!("Test interrupted"));
        }

        let swap_response = client.create_swap(CreateSwapRequest::new_lbtc_submarine(&pair.hash, &invoice.to_string(), ""))
            .map_err(|_| TestError::ServersUnreachable)?;

        let funding_amount = swap_response.get_funding_amount()
            .map_err(|_| anyhow!("Could not get funding address"))?;

        let funding_addr = swap_response.get_funding_address()
            .map_err(|_| anyhow!("Could not get funding address"))?;

        let mut cmd = Command::new("sh")
            .arg("-c")
            .arg("cargo run")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = cmd.stdin.as_mut().expect("expected stdin");
        let stdout = cmd.stdout.take().expect("expected stdout");
        let mut reader = BufReader::new(stdout);

        run_command(&format!("send {funding_amount} {funding_addr} \n"), stdin, &mut reader)?;

        Ok(())
    }

    #[tokio::test]
    async fn reverse_submarine_swap() -> Result<()> {
        let mut cmd = Command::new("sh")
            .arg("-c")
            .arg("cargo run")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = cmd.stdin.as_mut().expect("expected stdin");
        let stdout = cmd.stdout.take().expect("expected stdout");
        let mut reader = BufReader::new(stdout);

        run_command("get-balance\n", stdin, &mut reader)?;

        Ok(())
    }
}
