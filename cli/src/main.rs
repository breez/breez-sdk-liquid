mod commands;
mod persist;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use breez_sdk_liquid::{Network, Wallet};
use clap::Parser;
use commands::{handle_command, CliHelper, Command, CommandResult};
use log::{error, info};
use persist::CliPersistence;
use rustyline::{error::ReadlineError, hint::HistoryHinter, Editor};

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[clap(name = "data_dir", short = 'd', long = "data_dir")]
    pub(crate) data_dir: Option<String>,
}

fn show_results(result: Result<String>) -> Result<()> {
    let result_str = match result {
        Ok(r) => r,
        Err(err) => serde_json::to_string_pretty(&CommandResult {
            success: false,
            message: err.to_string(),
        })?,
    };

    Ok(println!("{result_str}"))
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let data_dir_str = args.data_dir.clone().unwrap_or(".data".to_string());
    let data_dir = PathBuf::from(&data_dir_str);
    fs::create_dir_all(&data_dir)?;

    let persistence = CliPersistence { data_dir };
    let history_file = &persistence.history_file();

    let rl = &mut Editor::new()?;
    rl.set_helper(Some(CliHelper {
        hinter: HistoryHinter {},
    }));

    if rl.load_history(history_file).is_err() {
        info!("No history found");
    }

    let mnemonic = persistence.get_or_create_mnemonic()?;
    let wallet = Wallet::init(
        &mnemonic.to_string(),
        Some(data_dir_str),
        Network::LiquidTestnet,
    )?;

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
                let res = handle_command(rl, &wallet, cli_res.unwrap());
                show_results(res)?;
            }
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
