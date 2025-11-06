mod commands;
mod persist;

use std::sync::Arc;
use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use breez_sdk_liquid::plugin::Plugin;
use breez_sdk_liquid::prelude::*;
use breez_sdk_liquid_nwc::{model::NwcConfig, SdkNwcService};
use clap::Parser;
use commands::{handle_command, CliHelper, Command, CommandResult};
use log::{error, info};
use persist::CliPersistence;
use rustyline::{error::ReadlineError, hint::HistoryHinter, Editor};
use tokio::sync::OnceCell;

lazy_static::lazy_static! {
    pub(crate) static ref NWC_SERVICE: OnceCell<Arc<SdkNwcService>> = OnceCell::new();
}

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[clap(short, long)]
    pub(crate) data_dir: Option<String>,

    #[clap(long, action)]
    pub(crate) no_data_sync: bool,

    #[clap(long, action)]
    pub(crate) no_mrh: bool,

    #[clap(short, long)]
    pub(crate) log_file: Option<String>,

    #[clap(short, long, value_parser = parse_network_arg)]
    pub(crate) network: Option<LiquidNetwork>,

    #[clap(short, long)]
    pub(crate) phrase_path: Option<String>,

    #[clap(long)]
    pub(crate) passphrase: Option<String>,

    #[clap(long, default_value = "false")]
    pub(crate) no_qrs: bool,

    #[clap(long, default_value = "false")]
    pub(crate) nwc: bool,
}

fn parse_network_arg(s: &str) -> Result<LiquidNetwork, String> {
    LiquidNetwork::try_from(s).map_err(|e| e.to_string())
}

fn show_results(result: Result<String>) -> Result<()> {
    let result_str = match result {
        Ok(r) => r,
        Err(err) => serde_json::to_string_pretty(&CommandResult {
            success: false,
            message: err.to_string(),
        })?,
    };

    println!("{result_str}");
    Ok(())
}

struct CliEventListener {}

#[async_trait::async_trait]
impl EventListener for CliEventListener {
    async fn on_event(&self, e: SdkEvent) {
        info!("Received event: {e:?}");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let data_dir_str = args
        .data_dir
        .clone()
        .unwrap_or(DEFAULT_DATA_DIR.to_string());
    let data_dir = PathBuf::from(&data_dir_str);
    fs::create_dir_all(&data_dir)?;

    LiquidSdk::init_logging(&data_dir_str, None)?;

    let data_sync_url = std::env::var_os("SYNC_SERVICE_URL")
        .map(|var| var.into_string().expect("Expected valid sync service url"));

    let persistence = CliPersistence { data_dir };
    let history_file = &persistence.history_file();

    let rl = &mut Editor::new()?;
    rl.set_helper(Some(CliHelper {
        hinter: HistoryHinter {},
    }));

    if rl.load_history(history_file).is_err() {
        info!("No history found");
    }

    let mnemonic = persistence.get_or_create_mnemonic(args.phrase_path.as_deref())?;
    let passphrase = args.passphrase.clone();
    let network = args.network.unwrap_or(LiquidNetwork::Testnet);
    let breez_api_key = std::env::var_os("BREEZ_API_KEY")
        .map(|var| var.into_string().expect("Expected valid API key string"));
    let mut config = LiquidSdk::default_config(network, breez_api_key)?;
    config.working_dir = data_dir_str;
    config.use_magic_routing_hints = !args.no_mrh;
    if args.no_data_sync {
        config.sync_service_url = None;
    } else if data_sync_url.is_some() {
        config.sync_service_url = data_sync_url;
    }
    let mut plugins: Vec<Arc<dyn Plugin>> = vec![];
    if args.nwc {
        let nwc_service = Arc::new(SdkNwcService::new(NwcConfig {
            relay_urls: None,
            secret_key_hex: None,
        }));
        NWC_SERVICE.set(nwc_service.clone()).unwrap_or_else(|_| {
            panic!("Could not set NWC service");
        });
        plugins.push(nwc_service);
    };
    let sdk = LiquidSdk::connect(
        ConnectRequest {
            config,
            mnemonic: Some(mnemonic.to_string()),
            passphrase,
            seed: None,
        },
        Some(plugins),
    )
    .await?;
    let listener_id = sdk
        .add_event_listener(Box::new(CliEventListener {}))
        .await?;

    let cli_prompt = match network {
        LiquidNetwork::Mainnet => "breez-liquid-cli [mainnet]> ",
        LiquidNetwork::Testnet => "breez-liquid-cli [testnet]> ",
        LiquidNetwork::Regtest => "breez-liquid-cli [regtest]> ",
    };

    loop {
        let readline = rl.readline(cli_prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let mut vec = shellwords::split(&line)
                    .map_err(|e| anyhow!("Failed to parse command line: {}", e))?;
                vec.insert(0, "".to_string());
                let cli_res = Command::try_parse_from(vec);
                if cli_res.is_err() {
                    println!("{}", cli_res.unwrap_err());
                    continue;
                }
                let res = handle_command(rl, &sdk, &args, cli_res.unwrap()).await;
                show_results(res)?;
            }
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                sdk.disconnect().await?;
                break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                sdk.disconnect().await?;
                break;
            }
            Err(err) => {
                error!("Error: {err:?}");
                break;
            }
        }
    }

    sdk.remove_event_listener(listener_id).await?;
    rl.save_history(history_file).map_err(|e| anyhow!(e))
}
