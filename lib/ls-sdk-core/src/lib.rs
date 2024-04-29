pub mod model;
mod persist;
mod wallet;

pub use model::*;
pub use wallet::*;

/// Claim tx feerate for Receive, in sats per vbyte.
/// Since the  Liquid blocks are consistently empty for now, we hardcode the minimum feerate.
pub const LIQUID_CLAIM_TX_FEERATE: f32 = 0.1;
pub const DEFAULT_DATA_DIR: &str = ".data";

#[macro_export]
macro_rules! ensure_sdk {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

#[macro_export]
macro_rules! get_invoice_amount {
    ($invoice:expr) => {
        $invoice
            .parse::<Bolt11Invoice>()
            .expect("Expecting valid invoice")
            .amount_milli_satoshis()
            .expect("Expecting valid amount")
            / 1000
    };
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tempdir::TempDir;

    use crate::{Network, Payment, PaymentType, PrepareReceiveRequest, Wallet};

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
        let breez_wallet =
            Wallet::connect(TEST_MNEMONIC, Some(data_dir_str), Network::LiquidTestnet)?;

        let invoice = "lntb10u1pnqwkjrpp5j8ucv9mgww0ajk95yfpvuq0gg5825s207clrzl5thvtuzfn68h0sdqqcqzzsxqr23srzjqv8clnrfs9keq3zlg589jvzpw87cqh6rjks0f9g2t9tvuvcqgcl45f6pqqqqqfcqqyqqqqlgqqqqqqgq2qsp5jnuprlxrargr6hgnnahl28nvutj3gkmxmmssu8ztfhmmey3gq2ss9qyyssq9ejvcp6frwklf73xvskzdcuhnnw8dmxag6v44pffwqrxznsly4nqedem3p3zhn6u4ln7k79vk6zv55jjljhnac4gnvr677fyhfgn07qp4x6wrq";
        breez_wallet.prepare_send_payment(&invoice)?;
        assert!(!list_pending(&breez_wallet)?.is_empty());

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let breez_wallet =
            Wallet::connect(TEST_MNEMONIC, Some(data_dir_str), Network::LiquidTestnet)?;

        let prepare_response = breez_wallet.prepare_receive_payment(&PrepareReceiveRequest {
            receiver_amount_sat: Some(1000),
            payer_amount_sat: None,
        })?;
        breez_wallet.receive_payment(&prepare_response)?;
        assert!(!list_pending(&breez_wallet)?.is_empty());

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap_recovery() -> Result<()> {
        Ok(())
    }
}
