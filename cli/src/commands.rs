use std::borrow::Cow::{self, Owned};
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use breez_sdk_liquid::prelude::*;
use clap::{arg, Parser};
use qrcode_rs::render::unicode;
use qrcode_rs::{EcLevel, QrCode};
use rustyline::highlight::Highlighter;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use rustyline::{hint::HistoryHinter, Completer, Helper, Hinter, Validator};

use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Parser, Debug, Clone, PartialEq)]
pub(crate) enum Command {
    /// Send a payment directly or via a swap
    SendPayment {
        /// Invoice which has to be paid (BOLT11)
        #[arg(long)]
        invoice: Option<String>,

        /// BOLT12 offer. If specified, amount_sat must also be set.
        #[arg(long)]
        offer: Option<String>,

        /// Either BIP21 URI or Liquid address we intend to pay to
        #[arg(long)]
        address: Option<String>,

        /// The amount in satoshi to pay, in case of a direct Liquid address
        /// or amount-less BIP21
        #[arg(short, long)]
        amount_sat: Option<u64>,

        /// Whether or not this is a drain operation. If true, all available funds will be used.
        #[arg(short, long)]
        drain: Option<bool>,

        /// Delay for the send, in seconds
        #[arg(long)]
        delay: Option<u64>,
    },
    /// Fetch the current limits for Send and Receive payments
    FetchLightningLimits,
    /// Fetch the current limits for Onchain Send and Receive payments
    FetchOnchainLimits,
    /// Send to a Bitcoin onchain address via a swap
    SendOnchainPayment {
        /// Bitcoin onchain address to send to
        address: String,

        /// Amount that will be received, in satoshi. Must be set if `drain` is false or unset.
        receiver_amount_sat: Option<u64>,

        /// Whether or not this is a drain operation. If true, all available funds will be used.
        #[arg(short, long)]
        drain: Option<bool>,

        /// The optional fee rate to use, in sat/vbyte
        #[clap(short = 'f', long = "fee_rate")]
        fee_rate_sat_per_vbyte: Option<u32>,
    },
    /// Receive a payment directly or via a swap
    ReceivePayment {
        /// The method to use when receiving. Either "lightning", "bitcoin" or "liquid"
        #[arg(short = 'm', long = "method")]
        payment_method: Option<PaymentMethod>,

        /// Amount the payer will send, in satoshi
        /// If not specified, it will generate a BIP21 URI/Liquid address with no amount
        #[arg(short, long)]
        payer_amount_sat: Option<u64>,

        /// Optional description for the invoice
        #[clap(short = 'd', long = "description")]
        description: Option<String>,

        /// Optional if true uses the hash of the description
        #[clap(name = "use_description_hash", short = 's', long = "desc_hash")]
        use_description_hash: Option<bool>,
    },
    /// Generates an URL to buy bitcoin from a 3rd party provider
    BuyBitcoin {
        provider: BuyBitcoinProvider,

        /// Amount to buy, in satoshi
        amount_sat: u64,
    },
    /// List incoming and outgoing payments
    ListPayments {
        /// The optional payment type filter. Either "send" or "receive"
        #[clap(name = "filter", short = 'r', long = "filter")]
        filters: Option<Vec<PaymentType>>,

        /// The optional from unix timestamp
        #[clap(name = "from_timestamp", short = 'f', long = "from")]
        from_timestamp: Option<i64>,

        /// The optional to unix timestamp
        #[clap(name = "to_timestamp", short = 't', long = "to")]
        to_timestamp: Option<i64>,

        /// Optional limit of listed payments
        #[clap(short = 'l', long = "limit")]
        limit: Option<u32>,

        /// Optional offset in payments
        #[clap(short = 'o', long = "offset")]
        offset: Option<u32>,

        /// Optional Liquid BIP21 URI/address for Liquid payment method
        #[clap(short = 'd', long = "destination")]
        destination: Option<String>,

        /// Optional Liquid/Bitcoin address for Bitcoin payment method
        #[clap(short = 'a', long = "address")]
        address: Option<String>,
    },
    /// Retrieve a payment
    GetPayment {
        /// Lightning payment hash
        payment_hash: String,
    },
    /// List refundable chain swaps
    ListRefundables,
    /// Prepare a refund transaction for an incomplete swap
    PrepareRefund {
        // Swap address of the lockup
        swap_address: String,
        // Bitcoin onchain address to send the refund to
        refund_address: String,
        // Fee rate to use, in sat/vbyte
        fee_rate_sat_per_vbyte: u32,
    },
    /// Broadcast a refund transaction for an incomplete swap
    Refund {
        // Swap address of the lockup
        swap_address: String,
        // Bitcoin onchain address to send the refund to
        refund_address: String,
        // Fee rate to use, in sat/vbyte
        fee_rate_sat_per_vbyte: u32,
    },
    /// Rescan onchain swaps
    RescanOnchainSwaps,
    /// Get the balance and general info of the current instance
    GetInfo,
    /// Sign a message using the wallet private key
    SignMessage {
        /// The message to sign
        message: String,
    },
    /// Verify a message with a public key
    CheckMessage {
        message: String,
        pubkey: String,
        signature: String,
    },
    /// Sync local data with mempool and onchain data
    Sync,
    /// Get the recommended Bitcoin fees based on the configured mempool.space instance
    RecommendedFees,
    /// Empties the encrypted transaction cache
    EmptyCache,
    /// Backs up the current pending swaps
    Backup {
        #[arg(short, long)]
        backup_path: Option<String>,
    },
    /// Retrieve a list of backups
    Restore {
        #[arg(short, long)]
        backup_path: Option<String>,
    },
    /// Shuts down all background threads of this SDK instance
    Disconnect,
    /// Parse a generic string to get its type and relevant metadata
    Parse {
        /// Generic input (URL, LNURL, BIP-21 Bitcoin Address, LN invoice, etc)
        input: String,
    },
    /// Pay using LNURL
    LnurlPay {
        /// LN Address or LNURL-pay endpoint
        lnurl: String,

        /// Validates the success action URL
        #[clap(name = "validate_success_url", short = 'v', long = "validate")]
        validate_success_url: Option<bool>,
    },
    LnurlWithdraw {
        /// LNURL-withdraw endpoint
        lnurl: String,
    },
    LnurlAuth {
        /// LNURL-auth endpoint
        lnurl: String,
    },
    /// Register a webhook URL
    RegisterWebhook { url: String },
    /// Unregister the webhook URL
    UnregisterWebhook,
    /// List fiat currencies
    ListFiat {},
    /// Fetch available fiat rates
    FetchFiatRates {},
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

#[derive(Serialize)]
pub(crate) struct CommandResult<T: Serialize> {
    pub success: bool,
    pub message: T,
}

macro_rules! command_result {
    ($expr:expr) => {{
        to_string_pretty(&CommandResult {
            success: true,
            message: $expr,
        })?
    }};
}

macro_rules! wait_confirmation {
    ($prompt:expr,$result:expr) => {
        print!("{}", $prompt);
        std::io::stdout().flush()?;

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        if !['y', 'Y'].contains(&(buf.as_bytes()[0] as char)) {
            return Ok(command_result!($result));
        }
    };
}

pub(crate) async fn handle_command(
    rl: &mut Editor<CliHelper, DefaultHistory>,
    sdk: &Arc<LiquidSdk>,
    command: Command,
) -> Result<String> {
    Ok(match command {
        Command::ReceivePayment {
            payment_method,
            payer_amount_sat,
            description,
            use_description_hash,
        } => {
            let prepare_response = sdk
                .prepare_receive_payment(&PrepareReceiveRequest {
                    payer_amount_sat,
                    payment_method: payment_method.unwrap_or(PaymentMethod::Lightning),
                })
                .await?;

            wait_confirmation!(
                format!(
                    "Fees: {} sat. Are the fees acceptable? (y/N) ",
                    prepare_response.fees_sat
                ),
                "Payment receive halted"
            );

            let response = sdk
                .receive_payment(&ReceivePaymentRequest {
                    prepare_response,
                    description,
                    use_description_hash,
                })
                .await?;

            let mut result = command_result!(&response);
            result.push('\n');

            match parse(&response.destination).await? {
                InputType::Bolt11 { invoice } => result.push_str(&build_qr_text(&invoice.bolt11)),
                InputType::LiquidAddress { address } => {
                    result.push_str(&build_qr_text(&address.to_uri().map_err(|e| {
                        anyhow!("Could not build BIP21 from address data: {e:?}")
                    })?))
                }
                InputType::BitcoinAddress { address } => {
                    result.push_str(&build_qr_text(&address.to_uri().map_err(|e| {
                        anyhow!("Could not build BIP21 from address data: {e:?}")
                    })?))
                }
                _ => {}
            }
            result
        }
        Command::FetchLightningLimits => {
            let limits = sdk.fetch_lightning_limits().await?;
            command_result!(limits)
        }
        Command::FetchOnchainLimits => {
            let limits = sdk.fetch_onchain_limits().await?;
            command_result!(limits)
        }
        Command::SendPayment {
            invoice,
            offer,
            address,
            amount_sat,
            drain,
            delay,
        } => {
            let destination = match (invoice, offer, address) {
                (Some(invoice), None, None) => Ok(invoice),
                (None, Some(offer), None) => match amount_sat {
                    Some(_) => Ok(offer),
                    None => Err(anyhow!(
                        "Must specify an amount for a BOLT12 offer."
                    ))
                },
                (None, None, Some(address)) => Ok(address),
                (Some(_), _, Some(_)) => {
                    Err(anyhow::anyhow!(
                        "Cannot specify both invoice and address at the same time."
                    ))
                }
                _ => Err(anyhow!(
                    "Must specify either a BOLT11 invoice, a BOLT12 offer or a direct/BIP21 address."
                ))
            }?;
            let amount = match (amount_sat, drain.unwrap_or(false)) {
                (Some(amount_sat), _) => Some(PayAmount::Receiver { amount_sat }),
                (_, true) => Some(PayAmount::Drain),
                (_, _) => None,
            };

            let prepare_response = sdk
                .prepare_send_payment(&PrepareSendRequest {
                    destination,
                    amount,
                })
                .await?;

            wait_confirmation!(
                format!(
                    "Fees: {} sat. Are the fees acceptable? (y/N) ",
                    prepare_response.fees_sat
                ),
                "Payment send halted"
            );

            let send_payment_req = SendPaymentRequest {
                prepare_response: prepare_response.clone(),
            };

            if let Some(delay) = delay {
                let sdk_cloned = sdk.clone();

                tokio::spawn(async move {
                    thread::sleep(Duration::from_secs(delay));
                    sdk_cloned.send_payment(&send_payment_req).await.unwrap();
                });
                command_result!(prepare_response)
            } else {
                let response = sdk.send_payment(&send_payment_req).await?;
                command_result!(response)
            }
        }
        Command::SendOnchainPayment {
            address,
            receiver_amount_sat,
            drain,
            fee_rate_sat_per_vbyte,
        } => {
            let amount = match drain.unwrap_or(false) {
                true => PayAmount::Drain,
                false => PayAmount::Receiver {
                    amount_sat: receiver_amount_sat.ok_or(anyhow::anyhow!(
                        "Must specify `receiver_amount_sat` if not draining"
                    ))?,
                },
            };
            let prepare_response = sdk
                .prepare_pay_onchain(&PreparePayOnchainRequest {
                    amount,
                    fee_rate_sat_per_vbyte,
                })
                .await?;

            wait_confirmation!(
                format!(
                    "Fees: {} sat (incl claim fee: {} sat). Receiver amount: {} sat. Are the fees acceptable? (y/N) ",
                    prepare_response.total_fees_sat, prepare_response.claim_fees_sat, prepare_response.receiver_amount_sat
                ),
                "Payment send halted"
            );

            let response = sdk
                .pay_onchain(&PayOnchainRequest {
                    address,
                    prepare_response,
                })
                .await?;
            command_result!(response)
        }
        Command::BuyBitcoin {
            provider,
            amount_sat,
        } => {
            let prepare_response = sdk
                .prepare_buy_bitcoin(&PrepareBuyBitcoinRequest {
                    provider,
                    amount_sat,
                })
                .await?;

            wait_confirmation!(
                format!(
                    "Fees: {} sat. Are the fees acceptable? (y/N) ",
                    prepare_response.fees_sat
                ),
                "Buy Bitcoin halted"
            );

            let url = sdk
                .buy_bitcoin(&BuyBitcoinRequest {
                    prepare_response,
                    redirect_url: None,
                })
                .await?;

            let mut result = command_result!(url.clone());
            result.push('\n');
            result.push_str(&build_qr_text(&url));
            result
        }
        Command::GetInfo => {
            command_result!(sdk.get_info().await?)
        }
        Command::SignMessage { message } => {
            let req = SignMessageRequest { message };
            let res = sdk.sign_message(&req)?;
            command_result!(format!("Message signature: {}", res.signature))
        }
        Command::CheckMessage {
            message,
            pubkey,
            signature,
        } => {
            let req = CheckMessageRequest {
                message,
                pubkey,
                signature,
            };
            let res = sdk.check_message(&req)?;
            command_result!(format!("Message was signed by pubkey: {}", res.is_valid))
        }
        Command::ListPayments {
            filters,
            from_timestamp,
            to_timestamp,
            limit,
            offset,
            destination,
            address,
        } => {
            let details = match (destination, address) {
                (Some(destination), None) => Some(ListPaymentDetails::Liquid { destination }),
                (None, Some(address)) => Some(ListPaymentDetails::Bitcoin { address }),
                _ => None,
            };

            let payments = sdk
                .list_payments(&ListPaymentsRequest {
                    filters,
                    from_timestamp,
                    to_timestamp,
                    limit,
                    offset,
                    details,
                })
                .await?;
            command_result!(payments)
        }
        Command::GetPayment { payment_hash } => {
            let maybe_payment = sdk
                .get_payment(&GetPaymentRequest::Lightning { payment_hash })
                .await?;
            match maybe_payment {
                Some(payment) => command_result!(payment),
                None => {
                    return Err(anyhow!("Payment not found."));
                }
            }
        }
        Command::ListRefundables => {
            let refundables = sdk.list_refundables().await?;
            command_result!(refundables)
        }
        Command::PrepareRefund {
            swap_address,
            refund_address,
            fee_rate_sat_per_vbyte,
        } => {
            let res = sdk
                .prepare_refund(&PrepareRefundRequest {
                    swap_address,
                    refund_address,
                    fee_rate_sat_per_vbyte,
                })
                .await?;
            command_result!(res)
        }
        Command::Refund {
            swap_address,
            refund_address,
            fee_rate_sat_per_vbyte,
        } => {
            let res = sdk
                .refund(&RefundRequest {
                    swap_address,
                    refund_address,
                    fee_rate_sat_per_vbyte,
                })
                .await?;
            command_result!(res)
        }
        Command::RescanOnchainSwaps => {
            sdk.rescan_onchain_swaps().await?;
            command_result!("Rescanned successfully")
        }
        Command::Sync => {
            sdk.sync().await?;
            command_result!("Synced successfully")
        }
        Command::RecommendedFees => {
            let res = sdk.recommended_fees().await?;
            command_result!(res)
        }
        Command::EmptyCache => {
            sdk.empty_wallet_cache()?;
            command_result!("Cache emptied successfully")
        }
        Command::Backup { backup_path } => {
            sdk.backup(BackupRequest { backup_path })?;
            command_result!("Backup created successfully!")
        }
        Command::Restore { backup_path } => {
            sdk.restore(RestoreRequest { backup_path })?;
            command_result!("Backup restored successfully!")
        }
        Command::Disconnect => {
            sdk.disconnect().await?;
            command_result!("Liquid SDK instance disconnected")
        }
        Command::Parse { input } => {
            let res = LiquidSdk::parse(&input).await?;
            command_result!(res)
        }
        Command::LnurlPay {
            lnurl,
            validate_success_url,
        } => {
            let input = LiquidSdk::parse(&lnurl).await?;
            let res = match input {
                InputType::LnUrlPay { data: pd } => {
                    let prompt = format!(
                        "Amount to pay in millisatoshi (min {} msat, max {} msat): ",
                        pd.min_sendable, pd.max_sendable
                    );

                    let amount_msat = rl.readline(&prompt)?;
                    let prepare_response = sdk
                        .prepare_lnurl_pay(PrepareLnUrlPayRequest {
                            data: pd,
                            amount_msat: amount_msat.parse::<u64>()?,
                            comment: None,
                            validate_success_action_url: validate_success_url,
                        })
                        .await?;

                    wait_confirmation!(
                        format!(
                            "Fees: {} sat. Are the fees acceptable? (y/N) ",
                            prepare_response.fees_sat
                        ),
                        "LNURL pay halted"
                    );

                    let pay_res = sdk
                        .lnurl_pay(model::LnUrlPayRequest { prepare_response })
                        .await?;
                    Ok(pay_res)
                }
                _ => Err(anyhow!("Invalid input")),
            }?;

            command_result!(res)
        }
        Command::LnurlWithdraw { lnurl } => {
            let input = LiquidSdk::parse(&lnurl).await?;
            let res = match input {
                InputType::LnUrlWithdraw { data: pd } => {
                    let prompt = format!(
                        "Amount to withdraw in millisatoshi (min {} msat, max {} msat): ",
                        pd.min_withdrawable, pd.max_withdrawable
                    );

                    let amount_msat = rl.readline(&prompt)?;
                    let withdraw_res = sdk
                        .lnurl_withdraw(LnUrlWithdrawRequest {
                            data: pd,
                            amount_msat: amount_msat.parse()?,
                            description: Some("LNURL-withdraw".to_string()),
                        })
                        .await?;
                    Ok(withdraw_res)
                }
                _ => Err(anyhow!("Invalid input")),
            }?;

            command_result!(res)
        }
        Command::LnurlAuth { lnurl } => {
            let lnurl_endpoint = lnurl.trim();

            let res = match parse(lnurl_endpoint).await? {
                InputType::LnUrlAuth { data: ad } => {
                    let auth_res = sdk.lnurl_auth(ad).await?;
                    serde_json::to_string_pretty(&auth_res).map_err(|e| e.into())
                }
                _ => Err(anyhow!("Unexpected result type")),
            }?;

            command_result!(res)
        }
        Command::RegisterWebhook { url } => {
            sdk.register_webhook(url).await?;
            command_result!("Url registered successfully")
        }
        Command::UnregisterWebhook => {
            sdk.unregister_webhook().await?;
            command_result!("Url unregistered successfully")
        }
        Command::FetchFiatRates {} => {
            let res = sdk.fetch_fiat_rates().await?;
            command_result!(res)
        }
        Command::ListFiat {} => {
            let res = sdk.list_fiat_currencies().await?;
            command_result!(res)
        }
    })
}

fn build_qr_text(text: &str) -> String {
    QrCode::with_error_correction_level(text, EcLevel::L)
        .unwrap()
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build()
}
