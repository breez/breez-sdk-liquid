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
    use anyhow::Result;
    use tempdir::TempDir;

    use crate::{Network, Payment, PaymentType, ReceivePaymentRequest, Wallet};

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn create_temp_dir() -> Result<(TempDir, String)> {
        let data_dir = TempDir::new(&uuid::Uuid::new_v4().to_string())?;
        let data_dir_str = data_dir
            .as_ref()
            .to_path_buf()
            .to_str()
            .expect("Expecting valid temporary path")
            .to_owned();
        Ok((data_dir, data_dir_str))
    }

    fn list_pending(wallet: &Wallet) -> Result<Vec<Payment>> {
        let payments = wallet.list_payments(true, true)?;

        Ok(payments
            .iter()
            .filter(|p| {
                [PaymentType::PendingSend, PaymentType::PendingReceive].contains(&p.payment_type)
            })
            .map(|p| p.clone())
            .collect())
    }

    #[test]
    fn normal_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let breez_wallet = Wallet::init(TEST_MNEMONIC, Some(data_dir_str), Network::LiquidTestnet)?;

        let invoice = "lntb10u1pnqwkjrpp5j8ucv9mgww0ajk95yfpvuq0gg5825s207clrzl5thvtuzfn68h0sdqqcqzzsxqr23srzjqv8clnrfs9keq3zlg589jvzpw87cqh6rjks0f9g2t9tvuvcqgcl45f6pqqqqqfcqqyqqqqlgqqqqqqgq2qsp5jnuprlxrargr6hgnnahl28nvutj3gkmxmmssu8ztfhmmey3gq2ss9qyyssq9ejvcp6frwklf73xvskzdcuhnnw8dmxag6v44pffwqrxznsly4nqedem3p3zhn6u4ln7k79vk6zv55jjljhnac4gnvr677fyhfgn07qp4x6wrq";
        breez_wallet.prepare_payment(&invoice)?;
        assert!(!list_pending(&breez_wallet)?.is_empty());

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let breez_wallet = Wallet::init(TEST_MNEMONIC, Some(data_dir_str), Network::LiquidTestnet)?;

        breez_wallet.receive_payment(ReceivePaymentRequest {
            onchain_amount_sat: Some(1000),
            invoice_amount_sat: None,
        })?;
        assert!(!list_pending(&breez_wallet)?.is_empty());

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap_recovery() -> Result<()> {
        Ok(())
    }
}
