use anyhow::Result;
use boltz_client::{
    boltz::{
        ChainFees, ChainMinerFees, ChainPair, ChainSwapDetails, CreateChainResponse,
        CreateReverseResponse, CreateSubmarineResponse, GetBolt12ParamsResponse, GetNodesResponse,
        Leaf, MagicRoutingHint, Node, PairLimits, PairMinerFees, ReverseFees, ReverseLimits,
        ReversePair, SubmarineClaimTxResponse, SubmarineFees, SubmarinePair, SubmarinePairLimits,
        SwapTree,
    },
    util::secrets::Preimage,
    Amount, PublicKey,
};
use lwk_wollet::secp256k1;
use sdk_common::invoice::parse_invoice;
use std::{collections::HashMap, str::FromStr, sync::Mutex};

use crate::{
    ensure_sdk,
    error::{PaymentError, SdkError},
    model::{Direction, SendSwap, Swap, Transaction as SdkTransaction, Utxo},
    swapper::{ProxyUrlFetcher, Swapper},
    test_utils::generate_random_string,
    utils,
};

#[derive(Default)]
pub struct ZeroAmountSwapMockConfig {
    pub user_lockup_sat: u64,
    pub onchain_fee_increase_sat: u64,
}

#[derive(Default)]
pub struct MockSwapper {
    zero_amount_swap_mock_config: Mutex<ZeroAmountSwapMockConfig>,
}

impl MockSwapper {
    pub(crate) fn new() -> Self {
        MockSwapper::default()
    }

    fn mock_swap_tree() -> SwapTree {
        SwapTree {
            claim_leaf: Leaf {
                output: "".to_string(),
                version: 2,
            },
            refund_leaf: Leaf {
                output: "".to_string(),
                version: 2,
            },
        }
    }

    fn mock_public_key() -> PublicKey {
        utils::generate_keypair().public_key().into()
    }

    fn mock_swap_details() -> ChainSwapDetails {
        ChainSwapDetails {
            swap_tree: Self::mock_swap_tree(),
            lockup_address: "".to_string(),
            server_public_key: Self::mock_public_key(),
            timeout_block_height: 0,
            amount: 0,
            blinding_key: None,
            refund_address: None,
            claim_address: None,
            bip21: None,
        }
    }

    pub(crate) fn set_zero_amount_swap_mock_config(&self, config: ZeroAmountSwapMockConfig) {
        *self.zero_amount_swap_mock_config.lock().unwrap() = config;
    }

    fn new_chain_pair() -> ChainPair {
        ChainPair {
            hash: generate_random_string(10),
            rate: 0.0,
            limits: PairLimits {
                maximal: u64::MAX,
                minimal: 0,
                maximal_zero_conf: 100_000,
            },
            fees: ChainFees {
                percentage: 0.1,
                miner_fees: ChainMinerFees {
                    server: 100,
                    user: PairMinerFees {
                        lockup: 100,
                        claim: 100,
                    },
                },
            },
        }
    }

    async fn get_zero_amount_swap_server_lockup_sat(&self) -> u64 {
        let zero_amount_swap_mock_config = self.zero_amount_swap_mock_config.lock().unwrap();

        let pair = Self::new_chain_pair();
        let fees = pair
            .fees
            .boltz(zero_amount_swap_mock_config.user_lockup_sat)
            + pair.fees.server()
            + zero_amount_swap_mock_config.onchain_fee_increase_sat;

        zero_amount_swap_mock_config.user_lockup_sat - fees
    }
}

#[sdk_macros::async_trait]
impl Swapper for MockSwapper {
    async fn create_chain_swap(
        &self,
        _req: boltz_client::swaps::boltz::CreateChainRequest,
    ) -> Result<CreateChainResponse, PaymentError> {
        Ok(CreateChainResponse {
            id: generate_random_string(4),
            claim_details: Self::mock_swap_details(),
            lockup_details: Self::mock_swap_details(),
        })
    }

    async fn create_send_swap(
        &self,
        req: boltz_client::swaps::boltz::CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError> {
        let invoice = parse_invoice(&req.invoice)
            .map_err(|err| PaymentError::invalid_invoice(err.to_string()))?;
        let Some(amount_msat) = invoice.amount_msat else {
            return Err(PaymentError::invalid_invoice(
                "Invoice does not contain an amount",
            ));
        };

        Ok(CreateSubmarineResponse {
            accept_zero_conf: false,
            address: "".to_string(),
            bip21: "".to_string(),
            claim_public_key: Self::mock_public_key(),
            expected_amount: amount_msat / 1000,
            id: generate_random_string(4),
            referral_id: None,
            swap_tree: Self::mock_swap_tree(),
            timeout_block_height: 1459611,
            blinding_key: None,
        })
    }

    async fn get_chain_pair(
        &self,
        _direction: Direction,
    ) -> anyhow::Result<Option<ChainPair>, PaymentError> {
        Ok(Some(Self::new_chain_pair()))
    }

    async fn get_chain_pairs(
        &self,
    ) -> Result<(Option<ChainPair>, Option<ChainPair>), PaymentError> {
        let test_pair = Some(ChainPair {
            hash: generate_random_string(10),
            rate: 0.0,
            limits: PairLimits {
                maximal: u64::MAX,
                minimal: 0,
                maximal_zero_conf: 100_000,
            },
            fees: ChainFees {
                percentage: 0.1,
                miner_fees: ChainMinerFees {
                    server: 100,
                    user: PairMinerFees {
                        lockup: 100,
                        claim: 100,
                    },
                },
            },
        });
        Ok((test_pair.clone(), test_pair))
    }

    async fn get_submarine_preimage(&self, _swap_id: &str) -> Result<String, PaymentError> {
        Ok(Preimage::new().to_string().unwrap())
    }

    async fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError> {
        Ok(Some(SubmarinePair {
            hash: generate_random_string(10),
            rate: 0.0,
            limits: SubmarinePairLimits {
                maximal: 25_000_000,
                minimal: 1_000,
                maximal_zero_conf: 250_000,
                minimal_batched: Some(21),
            },
            fees: SubmarineFees {
                percentage: 0.1,
                miner_fees: 14,
            },
        }))
    }

    async fn get_send_claim_tx_details(
        &self,
        _swap: &SendSwap,
    ) -> Result<SubmarineClaimTxResponse, PaymentError> {
        Ok(SubmarineClaimTxResponse {
            preimage: Preimage::new()
                .to_string()
                .expect("Expected valid preimage"),
            pub_nonce: "".to_string(),
            public_key: Self::mock_public_key(),
            transaction_hash: "".to_string(),
        })
    }

    async fn create_claim_tx(
        &self,
        swap: Swap,
        _claim_address: Option<String>,
    ) -> Result<SdkTransaction, PaymentError> {
        let btc_tx = SdkTransaction::Bitcoin(boltz_client::bitcoin::Transaction {
            version: lwk_wollet::bitcoin::transaction::Version::TWO,
            lock_time: boltz_client::LockTime::ZERO,
            input: vec![],
            output: vec![],
        });
        let lbtc_tx = SdkTransaction::Liquid(boltz_client::elements::Transaction {
            version: 2,
            lock_time: boltz_client::ElementsLockTime::ZERO,
            input: vec![],
            output: vec![],
        });

        Ok(match &swap {
            Swap::Chain(swap) => match swap.direction {
                Direction::Incoming => lbtc_tx,
                Direction::Outgoing => btc_tx,
            },
            Swap::Receive(_) => lbtc_tx,
            Swap::Send(_) => unimplemented!(),
        })
    }

    async fn estimate_refund_broadcast(
        &self,
        _swap: Swap,
        _refund_address: &str,
        _fee_rate_sat_per_vb: Option<f64>,
        _is_cooperative: bool,
    ) -> Result<(u32, u64), SdkError> {
        Ok((0, 0))
    }

    async fn create_refund_tx(
        &self,
        swap: Swap,
        _refund_address: &str,
        _utxos: Vec<Utxo>,
        _broadcast_fee_rate_sat_per_vb: Option<f64>,
        _is_cooperative: bool,
    ) -> Result<SdkTransaction, PaymentError> {
        let btc_tx = SdkTransaction::Bitcoin(boltz_client::bitcoin::Transaction {
            version: lwk_wollet::bitcoin::transaction::Version::TWO,
            lock_time: boltz_client::LockTime::ZERO,
            input: vec![],
            output: vec![],
        });
        let lbtc_tx = SdkTransaction::Liquid(boltz_client::elements::Transaction {
            version: 2,
            lock_time: boltz_client::ElementsLockTime::ZERO,
            input: vec![],
            output: vec![],
        });

        Ok(match &swap {
            Swap::Chain(swap) => match swap.direction {
                Direction::Incoming => btc_tx,
                Direction::Outgoing => lbtc_tx,
            },
            Swap::Send(_) => lbtc_tx,
            Swap::Receive(_) => unimplemented!(),
        })
    }

    async fn claim_send_swap_cooperative(
        &self,
        _swap: &SendSwap,
        _claim_tx_response: boltz_client::swaps::boltz::SubmarineClaimTxResponse,
        _refund_address: &str,
    ) -> Result<(), PaymentError> {
        Ok(())
    }

    async fn create_receive_swap(
        &self,
        req: boltz_client::swaps::boltz::CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError> {
        Ok(CreateReverseResponse {
            id: generate_random_string(4),
            invoice: req.invoice.map_or(Some("".to_string()), |_| None),
            swap_tree: Self::mock_swap_tree(),
            lockup_address: "".to_string(),
            refund_public_key: Self::mock_public_key(),
            timeout_block_height: 0,
            onchain_amount: 0,
            blinding_key: None,
        })
    }

    async fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError> {
        Ok(Some(ReversePair {
            hash: "".to_string(),
            rate: 0.0,
            limits: ReverseLimits {
                maximal: 25_000_000,
                minimal: 1_000,
            },
            fees: ReverseFees {
                percentage: 0.25,
                miner_fees: PairMinerFees {
                    lockup: 14,
                    claim: 26,
                },
            },
        }))
    }

    async fn broadcast_tx(
        &self,
        _chain: boltz_client::network::Chain,
        tx_hex: &str,
    ) -> Result<String, PaymentError> {
        let tx = utils::deserialize_tx_hex(tx_hex)?;
        Ok(tx.txid().to_string())
    }

    async fn check_for_mrh(
        &self,
        _invoice: &str,
    ) -> Result<Option<(String, boltz_client::bitcoin::Amount)>, PaymentError> {
        // Ok(Some(("".to_string(), 0.0)))
        unimplemented!()
    }

    async fn get_bolt12_invoice(
        &self,
        _offer: &str,
        _amount_sat: u64,
    ) -> Result<String, PaymentError> {
        unimplemented!()
    }

    async fn create_bolt12_offer(&self, _offer: &str, _url: &str) -> Result<(), SdkError> {
        Ok(())
    }

    async fn update_bolt12_offer(
        &self,
        _offer: &str,
        _url: &str,
        _signature: &str,
    ) -> Result<(), SdkError> {
        unimplemented!()
    }

    async fn delete_bolt12_offer(&self, _offer: &str, _signature: &str) -> Result<(), SdkError> {
        unimplemented!()
    }

    async fn get_bolt12_params(&self) -> Result<GetBolt12ParamsResponse, SdkError> {
        Ok(GetBolt12ParamsResponse {
            min_cltv: 180,
            magic_routing_hint: MagicRoutingHint {
                channel_id: "596385002596073472".to_string(),
            },
        })
    }

    async fn get_nodes(&self) -> Result<GetNodesResponse, SdkError> {
        Ok(GetNodesResponse {
            btc: HashMap::from([(
                "CLN".to_string(),
                Node {
                    public_key: secp256k1::PublicKey::from_str(
                        "02d96eadea3d780104449aca5c93461ce67c1564e2e1d73225fa67dd3b997a6018",
                    )
                    .unwrap(),
                    uris: vec![
                        "02d96eadea3d780104449aca5c93461ce67c1564e2e1d73225fa67dd3b997a6018@143.202.162.204:9736".to_string(),
                        "02d96eadea3d780104449aca5c93461ce67c1564e2e1d73225fa67dd3b997a6018@2803:6900:581::1:c175:f0ad:9736".to_string()],
                },
            )]),
        })
    }

    async fn get_zero_amount_chain_swap_quote(&self, _swap_id: &str) -> Result<Amount, SdkError> {
        let server_lockup_amount_sat = self.get_zero_amount_swap_server_lockup_sat().await;
        Ok(Amount::from_sat(server_lockup_amount_sat))
    }

    async fn accept_zero_amount_chain_swap_quote(
        &self,
        _swap_id: &str,
        server_lockup_sat: u64,
    ) -> Result<(), PaymentError> {
        ensure_sdk!(
            server_lockup_sat == self.get_zero_amount_swap_server_lockup_sat().await,
            PaymentError::InvalidOrExpiredFees
        );
        Ok(())
    }
}

pub(crate) struct MockProxyUrlFetcher {}

impl MockProxyUrlFetcher {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[sdk_macros::async_trait]
impl ProxyUrlFetcher for MockProxyUrlFetcher {
    async fn fetch(&self) -> Result<&Option<String>> {
        Ok(&None)
    }
}
