use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sdk_common::grpc::SignUrlRequest;
use sdk_common::prelude::BreezServer;
use url::Url;

use crate::model::ChainSwap;

use super::FiatOnRampProvider;

#[derive(Clone)]
struct MoonPayConfig {
    pub base_url: String,
    pub api_key: String,
    pub lock_amount: String,
    pub currency_code: String,
    pub color_code: String,
    pub redirect_url: String,
    pub enabled_payment_methods: String,
}

fn moonpay_config() -> MoonPayConfig {
    MoonPayConfig {
        base_url: String::from("https://buy.moonpay.io"),
        api_key: String::from("pk_live_Mx5g6bpD6Etd7T0bupthv7smoTNn2Vr"),
        lock_amount: String::from("true"),
        currency_code: String::from("btc"),
        color_code: String::from("#055DEB"),
        redirect_url: String::from("https://buy.moonpay.io/transaction_receipt?addFunds=true"),
        enabled_payment_methods: String::from(
            "credit_debit_card,sepa_bank_transfer,gbp_bank_transfer",
        ),
    }
}

async fn create_moonpay_url(
    wallet_address: &str,
    quote_currency_amount: &str,
    redirect_url: Option<String>,
) -> Result<Url> {
    let config = moonpay_config();
    let url = Url::parse_with_params(
        &config.base_url,
        &[
            ("apiKey", &config.api_key),
            ("currencyCode", &config.currency_code),
            ("colorCode", &config.color_code),
            ("redirectURL", &redirect_url.unwrap_or(config.redirect_url)),
            ("enabledPaymentMethods", &config.enabled_payment_methods),
            ("walletAddress", &wallet_address.to_string()),
            ("quoteCurrencyAmount", &quote_currency_amount.to_string()),
            ("lockAmount", &config.lock_amount),
        ],
    )?;
    Ok(url)
}

pub(crate) struct MoonpayProvider {
    breez_server: Arc<BreezServer>,
}

impl MoonpayProvider {
    pub fn new(breez_server: Arc<BreezServer>) -> Self {
        Self { breez_server }
    }
}

#[async_trait]
impl FiatOnRampProvider for MoonpayProvider {
    async fn buy_bitcoin_onchain(
        &self,
        chain_swap: &ChainSwap,
        redirect_url: Option<String>,
    ) -> Result<String> {
        let config = moonpay_config();
        let create_response = chain_swap.get_boltz_create_response()?;
        let address = create_response.lockup_details.lockup_address;
        let amount = create_response.lockup_details.amount as f64 / 100_000_000.0;
        let url = create_moonpay_url(
            address.as_str(),
            format!("{:.8}", amount).as_str(),
            redirect_url,
        )
        .await?;
        let mut signer = self.breez_server.get_signer_client().await;
        let signed_url = signer
            .sign_url(SignUrlRequest {
                base_url: config.base_url.clone(),
                query_string: format!("?{}", url.query().unwrap()),
            })
            .await?
            .into_inner()
            .full_url;
        Ok(signed_url)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::HashMap;

    use crate::fiat::moonpay::{create_moonpay_url, moonpay_config};

    #[tokio::test]
    async fn test_sign_moonpay_url() -> Result<(), Box<dyn std::error::Error>> {
        let wallet_address = "a wallet address";
        let quote_amount = "a quote amount";
        let config = moonpay_config();

        let url = create_moonpay_url(wallet_address, quote_amount, None).await?;

        let query_pairs = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
        assert_eq!(url.host_str(), Some("buy.moonpay.io"));
        assert_eq!(url.path(), "/");
        assert_eq!(query_pairs.get("apiKey"), Some(&config.api_key));
        assert_eq!(query_pairs.get("currencyCode"), Some(&config.currency_code));
        assert_eq!(query_pairs.get("colorCode"), Some(&config.color_code));
        assert_eq!(query_pairs.get("redirectURL"), Some(&config.redirect_url));
        assert_eq!(query_pairs.get("lockAmount"), Some(&config.lock_amount));
        assert_eq!(
            query_pairs.get("enabledPaymentMethods"),
            Some(&config.enabled_payment_methods),
        );
        assert_eq!(
            query_pairs.get("walletAddress"),
            Some(&String::from(wallet_address))
        );
        assert_eq!(
            query_pairs.get("quoteCurrencyAmount"),
            Some(&String::from(quote_amount)),
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_sign_moonpay_url_with_redirect() -> Result<(), Box<dyn std::error::Error>> {
        let wallet_address = "a wallet address";
        let quote_amount = "a quote amount";
        let redirect_url = "https://test.moonpay.url/receipt".to_string();
        let config = moonpay_config();

        let url =
            create_moonpay_url(wallet_address, quote_amount, Some(redirect_url.clone())).await?;

        let query_pairs = url.query_pairs().into_owned().collect::<HashMap<_, _>>();
        assert_eq!(url.host_str(), Some("buy.moonpay.io"));
        assert_eq!(url.path(), "/");
        assert_eq!(query_pairs.get("apiKey"), Some(&config.api_key));
        assert_eq!(query_pairs.get("currencyCode"), Some(&config.currency_code));
        assert_eq!(query_pairs.get("colorCode"), Some(&config.color_code));
        assert_eq!(query_pairs.get("redirectURL"), Some(&redirect_url));
        assert_eq!(query_pairs.get("lockAmount"), Some(&config.lock_amount));
        assert_eq!(
            query_pairs.get("enabledPaymentMethods"),
            Some(&config.enabled_payment_methods),
        );
        assert_eq!(
            query_pairs.get("walletAddress"),
            Some(&String::from(wallet_address))
        );
        assert_eq!(
            query_pairs.get("quoteCurrencyAmount"),
            Some(&String::from(quote_amount)),
        );
        Ok(())
    }
}
