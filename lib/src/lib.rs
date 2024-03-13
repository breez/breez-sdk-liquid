pub mod model;
pub mod wallet;

pub use model::*;
pub use wallet::*;

#[cfg(test)]
mod tests {
    use crate::{Network, Wallet, WalletOptions};
    use anyhow::Result;
    use bip39::{Language, Mnemonic};
    use lwk_common::{singlesig_desc, Singlesig};
    use lwk_signer::SwSigner;
    use std::{env, fs, io, path::PathBuf, str::FromStr, sync::Arc};

    const DEFAULT_DATA_DIR: &str = ".data";
    const PHRASE_FILE_NAME: &str = "phrase";

    fn get_mnemonic() -> Result<Mnemonic> {
        let data_dir = PathBuf::from(env::var("DATA_DIR").unwrap_or(DEFAULT_DATA_DIR.to_string()));
        fs::create_dir_all(&data_dir)?;

        let filename = data_dir.join(PHRASE_FILE_NAME);

        let mnemonic = match fs::read_to_string(filename.clone()) {
            Ok(phrase) => Mnemonic::from_str(&phrase).unwrap(),
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    panic!(
                        "Can't read from file: {}, err {e}",
                        filename.to_str().unwrap()
                    );
                }
                let mnemonic = Mnemonic::generate_in(Language::English, 24)?;
                fs::write(filename, mnemonic.to_string())?;
                mnemonic
            }
        };

        Ok(mnemonic)
    }

    fn init_wallet() -> Result<Arc<Wallet>> {
        let mnemonic = get_mnemonic()?;
        let signer = SwSigner::new(&mnemonic.to_string(), false)?;
        let desc = singlesig_desc(
            &signer,
            Singlesig::Wpkh,
            lwk_common::DescriptorBlindingKey::Elip151,
            false,
        )
        .expect("Expected valid descriptor");

        Wallet::new(WalletOptions {
            signer,
            desc,
            electrum_url: None,
            db_root_dir: None,
            network: Network::LiquidTestnet,
        })
    }

    #[test]
    fn normal_submarine_swap() -> Result<()> {
        let breez_wallet = init_wallet()?;

        let mut invoice = String::new();
        println!("Please paste the invoice to be paid: ");
        io::stdin().read_line(&mut invoice)?;

        breez_wallet.send_payment(&invoice)?;

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap_success() -> Result<()> {
        let breez_wallet = init_wallet()?;

        let swap_response = breez_wallet.receive_payment(1000)?;

        println!(
            "Please pay the following invoice: {}",
            swap_response.invoice
        );

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap_recovery() -> Result<()> {
        Ok(())
    }
}
