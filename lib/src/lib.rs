mod model;
mod persist;
mod wallet;

pub use model::*;
pub use wallet::*;

// To avoid sendrawtransaction error "min relay fee not met"
const CLAIM_ABSOLUTE_FEES: u64 = 134;
const DEFAULT_DATA_DIR: &str = ".data";
const DEFAULT_ELECTRUM_URL: &str = "blockstream.info:465";

#[macro_export]
macro_rules! ensure_sdk {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

#[cfg(test)]
mod tests {
    use std::{env, fs, io, path::PathBuf, str::FromStr};

    use anyhow::Result;
    use bip39::{Language, Mnemonic};

    use crate::{ReceivePaymentRequest, Wallet, DEFAULT_DATA_DIR};

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

    #[test]
    fn normal_submarine_swap() -> Result<()> {
        let breez_wallet = Wallet::init(get_mnemonic()?.to_string())?;

        let mut invoice = String::new();
        println!("Please paste the invoice to be paid: ");
        io::stdin().read_line(&mut invoice)?;

        let prepare_response = breez_wallet.prepare_payment(&invoice)?;
        breez_wallet.send_payment(&prepare_response)?;

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap_success() -> Result<()> {
        let breez_wallet = Wallet::init(get_mnemonic()?.to_string())?;

        let swap_response = breez_wallet.receive_payment(ReceivePaymentRequest {
            onchain_amount_sat: Some(1000),
            invoice_amount_sat: None,
        })?;

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
