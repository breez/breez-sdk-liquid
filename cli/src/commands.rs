use std::borrow::Cow::{self, Owned};
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use breez_sdk_liquid::prelude::*;
use clap::{arg, ArgAction, Parser};
use qrcode_rs::render::unicode;
use qrcode_rs::{EcLevel, QrCode};
use rustyline::highlight::Highlighter;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use rustyline::{hint::HistoryHinter, Completer, Helper, Hinter, Validator};

use serde::Serialize;
use serde_json::to_string_pretty;

use crate::Args;

#[derive(Parser, Debug, Clone, PartialEq)]
pub(crate) enum Command {
    /// Send a payment directly or via a swap
    SendPayment {
        /// Invoice which has to be paid (BOLT11)
        #[arg(short, long)]
        invoice: Option<String>,

        /// BOLT12 offer. If specified, amount_sat must also be set.
        #[arg(short, long)]
        offer: Option<String>,

        /// Either BIP21 URI or Liquid address we intend to pay to
        #[arg(short, long)]
        address: Option<String>,

        /// The amount to pay, in satoshi. The amount is optional if it is already provided in the
        /// invoice or BIP21 URI.
        #[arg(long)]
        amount_sat: Option<u64>,

        /// Optional id of the asset, in case of a direct Liquid address
        /// or amount-less BIP21
        #[clap(long = "asset")]
        asset_id: Option<String>,

        /// Whether or not the tx should be paid using the asset
        #[clap(long, action = ArgAction::SetTrue)]
        use_asset_fees: Option<bool>,

        /// The amount to pay, in case of a Liquid payment. The amount is optional if it is already
        /// provided in the BIP21 URI.
        /// The asset id must also be provided.
        #[arg(long)]
        amount: Option<f64>,

        /// Optional payer note, which is to be included in the BOLT12 invoice request
        #[clap(short, long)]
        payer_note: Option<String>,

        /// Whether or not this is a drain operation. If true, all available funds will be used.
        #[clap(short, long, action = ArgAction::SetTrue)]
        drain: Option<bool>,

        /// Delay for the send, in seconds
        #[arg(long)]
        delay: Option<u64>,

        /// Can only be used when a non-LBTC asset is provided, along with an amount and destination
        /// Specifies whether or not to swap bitcoin for the provided asset, or use the wallet's
        /// assets (if present)
        #[clap(long, action = ArgAction::SetTrue)]
        use_bitcoin: Option<bool>,
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
        #[clap(short, long, action = ArgAction::SetTrue)]
        drain: Option<bool>,

        /// The optional fee rate to use, in sat/vbyte
        #[clap(short = 'f', long = "fee_rate")]
        fee_rate_sat_per_vbyte: Option<u32>,
    },
    /// Receive a payment directly or via a swap
    ReceivePayment {
        /// The method to use when receiving. Either "invoice", "offer", "bitcoin" or "liquid"
        #[arg(short = 'm', long = "method")]
        payment_method: Option<String>,

        /// Optional description for the invoice
        #[clap(short = 'd', long = "description")]
        description: Option<String>,

        /// Optional if true uses the hash of the description
        #[clap(name = "use_description_hash", short = 's', long = "desc_hash")]
        use_description_hash: Option<bool>,

        /// Optional payer note, typically included in a LNURL-Pay request
        #[clap(short, long)]
        payer_note: Option<String>,

        /// The amount the payer should send, in satoshi. If not specified, it will generate a
        /// BIP21 URI/address with no amount.
        #[arg(long)]
        amount_sat: Option<u64>,

        /// Optional id of the asset to receive when the 'payment_method' is "liquid"
        #[clap(long = "asset")]
        asset_id: Option<String>,

        /// The amount the payer should send, in asset units. If not specified, it will
        /// generate a BIP21 URI/address with no amount.
        /// The asset id must also be provided.
        #[arg(long)]
        amount: Option<f64>,
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

        /// The optional payment state. Either "pending", "complete", "failed", "pendingrefund" or "refundable"
        #[clap(name = "state", short = 's', long = "state")]
        states: Option<Vec<PaymentState>>,

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

        /// Optional id of the asset for Liquid payment method
        #[clap(long = "asset")]
        asset_id: Option<String>,

        /// Optional Liquid BIP21 URI/address for Liquid payment method
        #[clap(short = 'd', long = "destination")]
        destination: Option<String>,

        /// Optional Liquid/Bitcoin address for Bitcoin payment method
        #[clap(short = 'a', long = "address")]
        address: Option<String>,

        /// Whether or not to sort the payments by ascending timestamp
        #[clap(long = "ascending", action = ArgAction::SetTrue)]
        sort_ascending: Option<bool>,
    },
    /// Retrieve a payment
    #[command(group = clap::ArgGroup::new("payment_identifiers").args(&["payment_hash", "swap_id"]).required(true))]
    GetPayment {
        /// Lightning payment hash
        #[arg(long, short = 'p')]
        payment_hash: Option<String>,
        /// Swap ID or its hash
        #[arg(long, short = 's')]
        swap_id: Option<String>,
    },
    /// Get and potentially accept proposed fees for WaitingFeeAcceptance Payment
    ReviewPaymentProposedFees { swap_id: String },
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

        /// Optional comment, which is to be included in the invoice request sent to the LNURL endpoint
        #[clap(short, long)]
        comment: Option<String>,

        /// Whether or not this is a drain operation. If true, all available funds will be used.
        #[clap(short, long, action = ArgAction::SetTrue)]
        drain: Option<bool>,

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
    args: &Args,
    command: Command,
) -> Result<String> {
    Ok(match command {
        Command::ReceivePayment {
            payment_method,
            amount_sat,
            amount,
            asset_id,
            description,
            use_description_hash,
            payer_note,
        } => {
            let payment_method =
                payment_method.map_or(Ok(PaymentMethod::Bolt11Invoice), |method| {
                    match method.as_str() {
                        "invoice" => Ok(PaymentMethod::Bolt11Invoice),
                        "offer" => Ok(PaymentMethod::Bolt12Offer),
                        "bitcoin" => Ok(PaymentMethod::BitcoinAddress),
                        "liquid" => Ok(PaymentMethod::LiquidAddress),
                        _ => Err(anyhow!("Invalid payment method")),
                    }
                })?;
            let amount = match asset_id {
                Some(asset_id) => Some(ReceiveAmount::Asset {
                    asset_id,
                    payer_amount: amount,
                }),
                None => {
                    amount_sat.map(|payer_amount_sat| ReceiveAmount::Bitcoin { payer_amount_sat })
                }
            };
            let prepare_response = sdk
                .prepare_receive_payment(&PrepareReceiveRequest {
                    payment_method,
                    amount: amount.clone(),
                })
                .await?;

            let fees = prepare_response.fees_sat;
            let confirmation_msg = match amount {
                Some(_) => format!("Fees: {fees} sat. Are the fees acceptable? (y/N)"),
                None => {
                    let min = prepare_response.min_payer_amount_sat;
                    let max = prepare_response.max_payer_amount_sat;
                    let service_feerate = prepare_response.swapper_feerate;
                    format!(
                        "Fees: {fees} sat + {service_feerate:?}% of the sent amount. \
                        Sender should send between {min:?} sat and {max:?} sat. \
                        Are the fees acceptable? (y/N)"
                    )
                }
            };
            wait_confirmation!(confirmation_msg, "Payment receive halted");

            let response = sdk
                .receive_payment(&ReceivePaymentRequest {
                    prepare_response,
                    description,
                    use_description_hash,
                    payer_note,
                })
                .await?;

            let mut result = command_result!(&response);
            result.push('\n');

            if !args.no_qrs {
                match sdk.parse(&response.destination).await? {
                    InputType::Bolt11 { invoice } => {
                        result.push_str(&build_qr_text(&invoice.bolt11))
                    }
                    InputType::Bolt12Offer { offer, .. } => {
                        result.push_str(&build_qr_text(&offer.offer))
                    }
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
            payer_note,
            address,
            mut amount,
            amount_sat,
            mut asset_id,
            use_asset_fees,
            drain,
            delay,
            use_bitcoin: pay_with_bitcoin,
        } => {
            let destination = invoice.or(offer.or(address.clone())).ok_or(anyhow!(
                "Must specify either a BOLT11 invoice, a BOLT12 offer or a direct/BIP21 address."
            ))?;

            if let Some(address) = &address {
                if let Ok(InputType::LiquidAddress {
                    address:
                        LiquidAddressData {
                            asset_id: address_asset_id,
                            amount: address_amount,
                            ..
                        },
                }) = parse(address, None).await
                {
                    asset_id = asset_id.or(address_asset_id);
                    amount = amount.or(address_amount);
                };
            }

            let amount = match (asset_id, amount, amount_sat, drain.unwrap_or(false)) {
                (Some(asset_id), Some(receiver_amount), _, _) => Some(PayAmount::Asset {
                    asset_id,
                    receiver_amount,
                    estimate_asset_fees: use_asset_fees,
                    pay_with_bitcoin,
                }),
                (None, None, Some(receiver_amount_sat), _) => Some(PayAmount::Bitcoin {
                    receiver_amount_sat,
                }),
                (_, _, _, true) => Some(PayAmount::Drain),
                _ => None,
            };

            let prepare_response = sdk
                .prepare_send_payment(&PrepareSendRequest {
                    destination,
                    amount,
                })
                .await?;

            let confirmation_msg = match (
                use_asset_fees.unwrap_or(false),
                prepare_response.fees_sat,
                prepare_response.exchange_amount_sat,
                prepare_response.estimated_asset_fees,
            ) {
                (true, _, _, Some(asset_fees)) => {
                    format!("Fees: approx {asset_fees}. Are the fees acceptable? (y/N) ")
                }
                (false, Some(fees_sat), None, _) => {
                    format!("Fees: {fees_sat} sat. Are the fees acceptable? (y/N) ")
                }
                (false, Some(fees_sat), Some(exchange_amount_sat), _) => {
                    format!("Fees: {fees_sat} sat. Exchange amount: {exchange_amount_sat}. Are the fees and amount acceptable? (y/N) ")
                }
                (true, _, _, None) => {
                    bail!("Not able to pay asset fees")
                }
                (false, None, _, _) => {
                    bail!("Not able to pay satoshi fees")
                }
            };

            wait_confirmation!(confirmation_msg, "Payment send halted");

            let send_payment_req = SendPaymentRequest {
                prepare_response: prepare_response.clone(),
                use_asset_fees,
                payer_note,
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
                false => PayAmount::Bitcoin {
                    receiver_amount_sat: receiver_amount_sat.ok_or(anyhow::anyhow!(
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
            if !args.no_qrs {
                result.push('\n');
                result.push_str(&build_qr_text(&url));
            }
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
            states,
            from_timestamp,
            to_timestamp,
            limit,
            offset,
            asset_id,
            destination,
            address,
            sort_ascending,
        } => {
            let details = match (asset_id.clone(), destination.clone(), address) {
                (None, Some(_), None) | (Some(_), None, None) | (Some(_), Some(_), None) => {
                    Some(ListPaymentDetails::Liquid {
                        asset_id,
                        destination,
                    })
                }
                (None, None, Some(address)) => Some(ListPaymentDetails::Bitcoin {
                    address: Some(address),
                }),
                _ => None,
            };

            let payments = sdk
                .list_payments(&ListPaymentsRequest {
                    filters,
                    states,
                    from_timestamp,
                    to_timestamp,
                    limit,
                    offset,
                    details,
                    sort_ascending,
                })
                .await?;
            command_result!(payments)
        }
        Command::GetPayment {
            payment_hash,
            swap_id,
        } => {
            if payment_hash.is_none() && swap_id.is_none() {
                bail!("No payment identifiers provided.");
            }

            let maybe_payment = if let Some(payment_hash) = payment_hash {
                sdk.get_payment(&GetPaymentRequest::PaymentHash { payment_hash })
                    .await?
            } else if let Some(swap_id) = swap_id {
                sdk.get_payment(&GetPaymentRequest::SwapId { swap_id })
                    .await?
            } else {
                None
            };

            match maybe_payment {
                Some(payment) => command_result!(payment),
                None => {
                    return Err(anyhow!("Payment not found."));
                }
            }
        }
        Command::ReviewPaymentProposedFees { swap_id } => {
            let fetch_response = sdk
                .fetch_payment_proposed_fees(&FetchPaymentProposedFeesRequest { swap_id })
                .await?;

            let confirmation_msg = format!(
                "Payer amount: {} sat. Fees: {} sat. Resulting received amount: {} sat. Are the fees acceptable? (y/N) ",
                fetch_response.payer_amount_sat, fetch_response.fees_sat, fetch_response.receiver_amount_sat
            );

            wait_confirmation!(confirmation_msg, "Payment proposed fees review halted");

            sdk.accept_payment_proposed_fees(&AcceptPaymentProposedFeesRequest {
                response: fetch_response,
            })
            .await?;

            command_result!("Proposed fees accepted successfully")
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
            sdk.sync(false).await?;
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
            let res = sdk.parse(&input).await?;
            command_result!(res)
        }
        Command::LnurlPay {
            lnurl,
            comment,
            drain,
            validate_success_url,
        } => {
            let input = sdk.parse(&lnurl).await?;
            let res = match input {
                InputType::LnUrlPay {
                    data: pd,
                    bip353_address,
                } => {
                    let amount = match drain.unwrap_or(false) {
                        true => PayAmount::Drain,
                        false => {
                            let min_sendable = (pd.min_sendable as f64 / 1000.0).ceil() as u64;
                            let max_sendable = pd.max_sendable / 1000;
                            let prompt = format!(
                                "Amount to pay (min {min_sendable} sat, max {max_sendable} sat): "
                            );
                            let amount_sat = rl.readline(&prompt)?;
                            PayAmount::Bitcoin {
                                receiver_amount_sat: amount_sat.parse::<u64>()?,
                            }
                        }
                    };

                    let prepare_response = sdk
                        .prepare_lnurl_pay(PrepareLnUrlPayRequest {
                            data: pd,
                            amount,
                            bip353_address,
                            comment,
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
            let input = sdk.parse(&lnurl).await?;
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

            let res = match sdk.parse(lnurl_endpoint).await? {
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
