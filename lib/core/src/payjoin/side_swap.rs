use std::{collections::HashMap, str::FromStr};

use super::{
    error::{PayjoinError, PayjoinResult},
    model::{
        AcceptedAsset, AcceptedAssetsRequest, AcceptedAssetsResponse, Request, Response,
        SignRequest, StartRequest, Utxo,
    },
    pset::PsetInput,
    utxo_select::utxo_select,
    PayjoinService,
};
use boltz_client::Secp256k1;
use log::{debug, error};
use lwk_wollet::{
    bitcoin::base64::{self, Engine as _},
    elements::{
        self,
        confidential::{self, AssetBlindingFactor, ValueBlindingFactor},
        pset::PartiallySignedTransaction,
        secp256k1_zkp::Generator,
        Address, AssetId, Transaction, TxOutSecrets,
    },
};
use sdk_common::{
    ensure_sdk,
    prelude::{parse_json, FiatAPI, RestClient},
    utils::Arc,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::OnceCell;

use crate::model::{Config, LiquidNetwork};
use crate::payjoin::{
    model::Recipient,
    pset::{blind::remove_explicit_values, construct_pset, ConstructPsetRequest, PsetOutput},
    utxo_select::UtxoSelectRequest,
};
use crate::wallet::utxo_select::InOut;
use crate::{
    persist::Persister,
    utils,
    wallet::{network_fee::TxFee, OnchainWallet},
};

const PRODUCTION_SIDESWAP_URL: &str = "https://api.sideswap.io/payjoin";
const TESTNET_SIDESWAP_URL: &str = "https://api-testnet.sideswap.io/payjoin";
// Base fee in USD represented in satoshis ($0.04)
const SIDESWAP_BASE_USD_FEE_SAT: f64 = 4_000_000.0;

pub(crate) struct SideSwapPayjoinService {
    config: Config,
    fiat_api: Arc<dyn FiatAPI>,
    persister: std::sync::Arc<Persister>,
    onchain_wallet: Arc<dyn OnchainWallet>,
    rest_client: Arc<dyn RestClient>,
    accepted_assets: OnceCell<AcceptedAssetsResponse>,
}

impl SideSwapPayjoinService {
    pub fn new(
        config: Config,
        fiat_api: Arc<dyn FiatAPI>,
        persister: std::sync::Arc<Persister>,
        onchain_wallet: Arc<dyn OnchainWallet>,
        rest_client: Arc<dyn RestClient>,
    ) -> Self {
        Self {
            config,
            fiat_api,
            persister,
            onchain_wallet,
            rest_client,
            accepted_assets: OnceCell::new(),
        }
    }

    fn get_url(&self) -> PayjoinResult<&str> {
        match self.config.network {
            LiquidNetwork::Mainnet => Ok(PRODUCTION_SIDESWAP_URL),
            LiquidNetwork::Testnet => Ok(TESTNET_SIDESWAP_URL),
            network => Err(PayjoinError::generic(format!(
                "Payjoin not supported on {network}"
            ))),
        }
    }

    async fn post_request<I: Serialize, O: DeserializeOwned>(&self, body: &I) -> PayjoinResult<O> {
        let headers = HashMap::from([("Content-Type".to_string(), "application/json".to_string())]);
        let body = serde_json::to_string(body)?;
        debug!("Posting request to SideSwap: {body}");
        let (response, status_code) = self
            .rest_client
            .post(self.get_url()?, Some(headers), Some(body))
            .await?;
        if status_code != 200 {
            error!("Received status code {status_code} response from SideSwap");
            return Err(PayjoinError::service_connectivity(format!(
                "Failed to post request to SideSwap: {response}"
            )));
        }
        debug!("Received response from SideSwap: {response}");
        Ok(parse_json(&response)?)
    }
}

#[sdk_macros::async_trait]
impl PayjoinService for SideSwapPayjoinService {
    async fn fetch_accepted_assets(&self) -> PayjoinResult<Vec<AcceptedAsset>> {
        let accepted_assets = self
            .accepted_assets
            .get_or_try_init(|| async {
                debug!("Initializing accepted_assets from SideSwap");
                let accepted_assets_request = Request::AcceptedAssets(AcceptedAssetsRequest {});
                let response: Response = self.post_request(&accepted_assets_request).await?;
                match response {
                    Response::AcceptedAssets(accepted_assets) => Ok(accepted_assets),
                    _ => Err(PayjoinError::service_connectivity(
                        "Failed to request accepted assets from SideSwap",
                    )),
                }
            })
            .await?;

        Ok(accepted_assets.accepted_asset.clone())
    }

    async fn estimate_payjoin_tx_fee(&self, asset_id: &str, amount_sat: u64) -> PayjoinResult<f64> {
        // Check the asset is accepted
        let fee_asset = AssetId::from_str(asset_id)?;
        let accepted_assets = self.fetch_accepted_assets().await?;
        ensure_sdk!(
            accepted_assets
                .iter()
                .any(|asset| asset.asset_id == asset_id),
            PayjoinError::generic("Asset not accepted by SideSwap")
        );

        // Get and check the wallet asset balance
        let wallet_asset_balance: u64 = self
            .onchain_wallet
            .asset_utxos(&fee_asset)
            .await?
            .iter()
            .map(|utxo| utxo.unblinded.value)
            .sum();
        ensure_sdk!(
            wallet_asset_balance > amount_sat,
            PayjoinError::InsufficientFunds
        );

        // Fetch the fiat rates
        let asset_metadata =
            self.persister
                .get_asset_metadata(asset_id)?
                .ok_or(PayjoinError::generic(format!(
                    "No asset metadata available for {asset_id}"
                )))?;
        let Some(fiat_id) = asset_metadata.fiat_id.clone() else {
            return Err(PayjoinError::generic(format!(
                "No fiat ID available in asset metadata for {asset_id}"
            )));
        };
        let fiat_rates = self.fiat_api.fetch_fiat_rates().await?;
        let usd_index_price = fiat_rates
            .iter()
            .find(|rate| rate.coin == "USD")
            .map(|rate| rate.value)
            .ok_or(PayjoinError::generic("No rate available for USD"))?;
        let asset_index_price = fiat_rates
            .iter()
            .find(|rate| rate.coin == fiat_id)
            .map(|rate| rate.value)
            .ok_or(PayjoinError::generic(format!(
                "No rate available for {fiat_id}"
            )))?;

        let fixed_fee = (SIDESWAP_BASE_USD_FEE_SAT / usd_index_price * asset_index_price) as u64;
        // Fees assuming we have:
        // - 1 input for the server (lbtc)
        // - 1 input for the user (asset)
        // - 1 output for the user (asset change)
        // - 1 output for the recipient (asset)
        // - 1 output for the server (asset fee)
        // - 1 output for the server (lbtc change)
        let network_fee = TxFee {
            native_inputs: 1,
            nested_inputs: 1,
            outputs: 4,
        }
        .fee(None);
        let fee_sat = (network_fee as f64 * asset_index_price) as u64 + fixed_fee;
        ensure_sdk!(
            wallet_asset_balance >= amount_sat + fee_sat,
            PayjoinError::InsufficientFunds
        );

        // The estimation accuracy gives a fee to two decimal places
        let mut fee = asset_metadata.amount_from_sat(fee_sat);
        fee = (fee * 100.0).ceil() / 100.0;

        debug!("Estimated payjoin server fee: {fee} ({fee_sat} satoshi units)");

        Ok(fee)
    }

    async fn build_payjoin_tx(
        &self,
        recipient_address: &str,
        asset_id: &str,
        amount_sat: u64,
    ) -> PayjoinResult<(Transaction, u64)> {
        let fee_asset = AssetId::from_str(asset_id)?;
        let wallet_utxos = self
            .onchain_wallet
            .asset_utxos(&fee_asset)
            .await?
            .iter()
            .map(Utxo::from)
            .collect::<Vec<_>>();
        ensure_sdk!(!wallet_utxos.is_empty(), PayjoinError::InsufficientFunds);

        let address = Address::from_str(recipient_address).map_err(|e| {
            PayjoinError::generic(format!(
                "Recipient address {recipient_address} is not a valid ElementsAddress: {e:?}"
            ))
        })?;
        let recipients = vec![Recipient {
            address,
            asset_id: fee_asset,
            amount: amount_sat,
        }];

        let start_request = Request::Start(StartRequest {
            asset_id: asset_id.to_string(),
            user_agent: "breezsdk".to_string(),
            api_key: self.config.sideswap_api_key.clone(),
        });
        let response: Response = self.post_request(&start_request).await?;
        let Response::Start(start_response) = response else {
            return Err(PayjoinError::service_connectivity(
                "Failed to start payjoin",
            ));
        };
        ensure_sdk!(
            start_response.fee_address.is_blinded(),
            PayjoinError::generic("Server fee address is not blinded")
        );
        ensure_sdk!(
            start_response.change_address.is_blinded(),
            PayjoinError::generic("Server change address is not blinded")
        );
        ensure_sdk!(
            !start_response.utxos.is_empty(),
            PayjoinError::generic("Server utxos are empty")
        );

        let policy_asset = utils::lbtc_asset_id(self.config.network);
        let utxo_select_res = utxo_select(UtxoSelectRequest {
            policy_asset,
            fee_asset,
            price: start_response.price,
            fixed_fee: start_response.fixed_fee,
            wallet_utxos: wallet_utxos.iter().map(Into::into).collect(),
            server_utxos: start_response.utxos.iter().map(Into::into).collect(),
            user_outputs: recipients
                .iter()
                .map(|recipient| InOut {
                    asset_id: recipient.asset_id,
                    value: recipient.amount,
                })
                .collect(),
        })?;
        ensure_sdk!(
            utxo_select_res.user_outputs.len() == recipients.len(),
            PayjoinError::generic("Output/recipient lengths mismatch")
        );

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        // Set the wallet and server inputs
        inputs.append(&mut select_utxos(
            wallet_utxos,
            utxo_select_res
                .user_inputs
                .into_iter()
                .chain(utxo_select_res.client_inputs.into_iter())
                .collect(),
        )?);
        inputs.append(&mut select_utxos(
            start_response.utxos,
            utxo_select_res.server_inputs,
        )?);

        // Set the outputs
        let server_fee = utxo_select_res.server_fee;

        // Recipient outputs
        for (output, recipient) in utxo_select_res
            .user_outputs
            .iter()
            .zip(recipients.into_iter())
        {
            debug!(
                "Payjoin recipent output: {} value: {}",
                recipient.address, output.value
            );
            outputs.push(PsetOutput {
                asset_id: output.asset_id,
                amount: output.value,
                address: recipient.address,
            });
        }

        // Change outputs
        for output in utxo_select_res
            .change_outputs
            .iter()
            .chain(utxo_select_res.fee_change.iter())
        {
            let address = self.onchain_wallet.next_unused_change_address().await?;
            debug!("Payjoin change output: {address} value: {}", output.value);
            outputs.push(PsetOutput {
                asset_id: output.asset_id,
                amount: output.value,
                address,
            });
        }

        // Server fee output
        debug!(
            "Payjoin server fee output: {} value: {}",
            start_response.fee_address, server_fee.value
        );
        outputs.push(PsetOutput {
            asset_id: server_fee.asset_id,
            amount: server_fee.value,
            address: start_response.fee_address,
        });

        // Server change output
        if let Some(output) = utxo_select_res.server_change {
            debug!(
                "Payjoin server change output: {} value: {}",
                start_response.change_address, output.value
            );
            outputs.push(PsetOutput {
                asset_id: output.asset_id,
                amount: output.value,
                address: start_response.change_address,
            });
        }

        // Construct the PSET
        let blinded_pset = construct_pset(ConstructPsetRequest {
            policy_asset,
            inputs,
            outputs,
            network_fee: utxo_select_res.network_fee.value,
        })?;

        let mut pset = blinded_pset.clone();
        remove_explicit_values(&mut pset);
        let server_pset = elements::encode::serialize(&pset);

        // Send the signing request
        let sign_request = Request::Sign(SignRequest {
            order_id: start_response.order_id,
            pset: base64::engine::general_purpose::STANDARD.encode(&server_pset),
        });
        let response: Response = self.post_request(&sign_request).await?;
        let Response::Sign(sign_response) = response else {
            return Err(PayjoinError::service_connectivity("Failed to sign payjoin"));
        };

        // Copy the signed inputs to the blinded PSET
        let server_signed_pset = elements::encode::deserialize::<PartiallySignedTransaction>(
            &base64::engine::general_purpose::STANDARD.decode(&sign_response.pset)?,
        )?;
        let server_signed_blinded_pset = copy_signatures(blinded_pset, server_signed_pset)?;

        let tx = self
            .onchain_wallet
            .sign_pset(server_signed_blinded_pset)
            .await?;
        Ok((tx, server_fee.value))
    }
}

impl From<&Utxo> for InOut {
    fn from(utxo: &Utxo) -> Self {
        Self {
            asset_id: utxo.asset_id,
            value: utxo.value,
        }
    }
}

fn copy_signatures(
    mut dst_pset: PartiallySignedTransaction,
    src_pset: PartiallySignedTransaction,
) -> PayjoinResult<PartiallySignedTransaction> {
    ensure_sdk!(
        dst_pset.inputs().len() == src_pset.inputs().len(),
        PayjoinError::generic("Input lengths mismatch")
    );
    ensure_sdk!(
        dst_pset.outputs().len() == src_pset.outputs().len(),
        PayjoinError::generic("Output lengths mismatch")
    );
    for (dst_input, src_input) in dst_pset
        .inputs_mut()
        .iter_mut()
        .zip(src_pset.inputs().iter())
    {
        if src_input.final_script_witness.is_some() {
            dst_input.final_script_sig = src_input.final_script_sig.clone();
            dst_input.final_script_witness = src_input.final_script_witness.clone();
        }
    }
    Ok(dst_pset)
}

fn select_utxos(mut utxos: Vec<Utxo>, in_outs: Vec<InOut>) -> PayjoinResult<Vec<PsetInput>> {
    let secp = Secp256k1::new();
    let mut selected = Vec::new();
    for in_out in in_outs {
        let index = utxos
            .iter()
            .position(|utxo| utxo.asset_id == in_out.asset_id && utxo.value == in_out.value)
            .ok_or(PayjoinError::generic("Failed to find utxo"))?;
        let utxo = utxos.remove(index);

        let (asset_commitment, value_commitment) = if utxo.asset_bf == AssetBlindingFactor::zero()
            || utxo.value_bf == ValueBlindingFactor::zero()
        {
            (
                confidential::Asset::Explicit(utxo.asset_id),
                confidential::Value::Explicit(utxo.value),
            )
        } else {
            let gen =
                Generator::new_blinded(&secp, utxo.asset_id.into_tag(), utxo.asset_bf.into_inner());
            (
                confidential::Asset::Confidential(gen),
                confidential::Value::new_confidential(&secp, utxo.value, gen, utxo.value_bf),
            )
        };

        let input = PsetInput {
            txid: utxo.txid,
            vout: utxo.vout,
            script_pub_key: utxo.script_pub_key,
            asset_commitment,
            value_commitment,
            tx_out_sec: TxOutSecrets {
                asset: utxo.asset_id,
                asset_bf: utxo.asset_bf,
                value: utxo.value,
                value_bf: utxo.value_bf,
            },
        };
        debug!("Payjoin input: {} vout: {}", input.txid, input.vout);
        selected.push(input);
    }
    Ok(selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use lwk_wollet::{
        elements::{OutPoint, Script, Txid},
        Chain, WalletTxOut,
    };
    use sdk_common::prelude::{BreezServer, MockResponse, MockRestClient, STAGING_BREEZSERVER_URL};
    use serde_json::json;

    use crate::{
        model::Signer,
        test_utils::{
            persist::create_persister,
            wallet::{MockSigner, MockWallet},
        },
    };

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    fn create_sideswap_payjoin_service(
        persister: std::sync::Arc<Persister>,
    ) -> Result<(Arc<MockWallet>, Arc<MockRestClient>, SideSwapPayjoinService)> {
        let config = Config::testnet_esplora(None);
        let breez_server = Arc::new(BreezServer::new(STAGING_BREEZSERVER_URL.to_string(), None)?);
        let signer: Arc<Box<dyn Signer>> = Arc::new(Box::new(MockSigner::new()?));
        let onchain_wallet = Arc::new(MockWallet::new(signer.clone())?);
        let rest_client = Arc::new(MockRestClient::new());

        Ok((
            onchain_wallet.clone(),
            rest_client.clone(),
            SideSwapPayjoinService::new(
                config,
                breez_server,
                persister,
                onchain_wallet,
                rest_client,
            ),
        ))
    }

    fn create_utxos(asset: AssetId, values: Vec<u64>) -> Vec<WalletTxOut> {
        let txid =
            Txid::from_str("0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();
        let script_pubkey =
            Script::from_str("76a914000000000000000000000000000000000000000088ac").unwrap();

        values.into_iter().map(|value| {
            WalletTxOut {
                outpoint: OutPoint::new(txid, 0),
                script_pubkey: script_pubkey.clone(),
                height: Some(10),
                unblinded: TxOutSecrets {
                    asset,
                    value,
                    asset_bf: AssetBlindingFactor::zero(),
                    value_bf: ValueBlindingFactor::zero(),
                },
                wildcard_index: 0,
                ext_int: Chain::Internal,
                is_spent: false,
                address: Address::from_str("lq1pqw8ct25kd47dejyesyvk3g2kaf8s9uhq4se7r2kj9y9hhvu9ug5thxlpn9y63s78kc2mcp6nujavckvr42q7hwkhqq9hfz46nth22hfp3em0ulm4nsuf").unwrap(),
            }
        }).collect()
    }

    #[sdk_macros::async_test_all]
    async fn test_fetch_accepted_assets_error() -> Result<()> {
        create_persister!(persister);
        let (_, mock_rest_client, payjoin_service) =
            create_sideswap_payjoin_service(persister).unwrap();

        mock_rest_client.add_response(MockResponse::new(400, "".to_string()));

        let res = payjoin_service.fetch_accepted_assets().await;
        assert!(res.is_err());

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_fetch_accepted_assets() -> Result<()> {
        create_persister!(persister);
        let (_, mock_rest_client, payjoin_service) =
            create_sideswap_payjoin_service(persister).unwrap();
        let asset_id = AssetId::from_slice(&[2; 32]).unwrap().to_string();

        let response_body =
            json!({"accepted_assets": {"accepted_asset":[{"asset_id": asset_id}]}}).to_string();
        mock_rest_client.add_response(MockResponse::new(200, response_body));

        let res = payjoin_service.fetch_accepted_assets().await;
        assert!(res.is_ok());
        let accepted_assets = res.unwrap();
        assert_eq!(accepted_assets.len(), 1);
        assert_eq!(accepted_assets[0].asset_id, asset_id);

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_estimate_payjoin_tx_fee_error() -> Result<()> {
        create_persister!(persister);
        let (_, mock_rest_client, payjoin_service) =
            create_sideswap_payjoin_service(persister).unwrap();
        let asset_id = AssetId::from_slice(&[2; 32]).unwrap().to_string();

        mock_rest_client.add_response(MockResponse::new(400, "".to_string()));

        let amount_sat = 500_000;
        let res = payjoin_service
            .estimate_payjoin_tx_fee(&asset_id, amount_sat)
            .await;
        assert!(res.is_err());

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_estimate_payjoin_tx_fee_no_utxos() -> Result<()> {
        create_persister!(persister);
        let (_, mock_rest_client, payjoin_service) =
            create_sideswap_payjoin_service(persister).unwrap();
        let asset_id = AssetId::from_slice(&[2; 32]).unwrap().to_string();

        let response_body =
            json!({"accepted_assets": {"accepted_asset":[{"asset_id": asset_id}]}}).to_string();
        mock_rest_client.add_response(MockResponse::new(200, response_body));

        let amount_sat = 500_000;
        let res = payjoin_service
            .estimate_payjoin_tx_fee(&asset_id, amount_sat)
            .await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Cannot pay: not enough funds");

        Ok(())
    }

    #[sdk_macros::async_test_all]
    async fn test_estimate_payjoin_tx_fee_no_asset_metadata() -> Result<()> {
        create_persister!(persister);
        let (mock_wallet, mock_rest_client, payjoin_service) =
            create_sideswap_payjoin_service(persister).unwrap();
        let asset_id = AssetId::from_slice(&[2; 32]).unwrap();
        let asset_id_str = asset_id.to_string();

        // Mock the accepted assets response
        let accepted_assets_response = json!({
            "accepted_assets": {
                "accepted_asset":[{"asset_id": asset_id_str}]
            }
        })
        .to_string();
        mock_rest_client.add_response(MockResponse::new(200, accepted_assets_response));

        // Set up the mock wallet to return some UTXOs for the test asset
        let utxos = create_utxos(asset_id, vec![1_000_000]);
        mock_wallet.set_utxos(utxos);

        let amount_sat = 500_000;
        let res = payjoin_service
            .estimate_payjoin_tx_fee(&asset_id_str, amount_sat)
            .await;

        assert_eq!(res.unwrap_err().to_string(), "No asset metadata available for 0202020202020202020202020202020202020202020202020202020202020202");

        Ok(())
    }

    #[sdk_macros::async_test_all]
    #[ignore = "Requires a mockable FiatAPI"]

    async fn test_estimate_payjoin_tx_fee() -> Result<()> {
        create_persister!(persister);
        let (mock_wallet, mock_rest_client, payjoin_service) =
            create_sideswap_payjoin_service(persister).unwrap();
        let asset_id =
            AssetId::from_str("b612eb46313a2cd6ebabd8b7a8eed5696e29898b87a43bff41c94f51acef9d73")
                .unwrap();
        let asset_id_str = asset_id.to_string();

        // Mock the accepted assets response
        let accepted_assets_response = json!({
            "accepted_assets": {
                "accepted_asset":[{"asset_id": asset_id_str}]
            }
        })
        .to_string();
        mock_rest_client.add_response(MockResponse::new(200, accepted_assets_response));

        // TODO: Mock the FiatAPI response as the staging BreezServer currently times out

        // Set up the mock wallet to return some UTXOs for the test asset
        let utxos = create_utxos(asset_id, vec![1_000_000]);
        mock_wallet.set_utxos(utxos);

        let amount_sat = 500_000;
        let res = payjoin_service
            .estimate_payjoin_tx_fee(&asset_id_str, amount_sat)
            .await;

        assert_eq!(res.unwrap_err().to_string(), "Cannot pay: not enough funds");

        Ok(())
    }
}
