#![cfg(test)]

use boltz_client::{
    boltz::{
        ChainFees, ChainMinerFees, ChainPair, ChainSwapDetails, CreateChainResponse,
        CreateReverseResponse, CreateSubmarineResponse, Leaf, PairLimits, PairMinerFees,
        ReverseFees, ReverseLimits, ReversePair, SubmarineClaimTxResponse, SubmarineFees,
        SubmarinePair, SwapTree,
    },
    util::secrets::Preimage,
    PublicKey,
};
use sdk_common::invoice::parse_invoice;

use crate::{
    error::{PaymentError, SdkError},
    model::{Direction, SendSwap, Swap, Transaction as SdkTransaction, Utxo},
    swapper::Swapper,
    test_utils::generate_random_string,
    utils,
};

use super::status_stream::MockStatusStream;

#[derive(Default)]
pub struct MockSwapper {}

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
}

impl Swapper for MockSwapper {
    fn create_chain_swap(
        &self,
        _req: boltz_client::swaps::boltz::CreateChainRequest,
    ) -> Result<CreateChainResponse, PaymentError> {
        Ok(CreateChainResponse {
            id: generate_random_string(4),
            claim_details: Self::mock_swap_details(),
            lockup_details: Self::mock_swap_details(),
        })
    }

    fn create_send_swap(
        &self,
        req: boltz_client::swaps::boltz::CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError> {
        let invoice = parse_invoice(&req.invoice)
            .map_err(|err| PaymentError::invalid_invoice(&err.to_string()))?;
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

    fn get_chain_pair(
        &self,
        _direction: Direction,
    ) -> anyhow::Result<Option<ChainPair>, PaymentError> {
        Ok(Some(ChainPair {
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
        }))
    }

    fn get_chain_pairs(&self) -> Result<(Option<ChainPair>, Option<ChainPair>), PaymentError> {
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

    fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError> {
        Ok(Some(SubmarinePair {
            hash: generate_random_string(10),
            rate: 0.0,
            limits: PairLimits {
                maximal: u64::MAX,
                minimal: 0,
                maximal_zero_conf: 100_000,
            },
            fees: SubmarineFees {
                percentage: 0.1,
                miner_fees: 100,
            },
        }))
    }

    fn get_send_claim_tx_details(
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

    fn create_claim_tx(
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

    fn estimate_refund_broadcast(
        &self,
        _swap: Swap,
        _refund_address: &str,
        _fee_rate_sat_per_vb: Option<f64>,
    ) -> Result<(u32, u64), SdkError> {
        Ok((0, 0))
    }

    fn create_refund_tx(
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

    fn claim_send_swap_cooperative(
        &self,
        _swap: &SendSwap,
        _claim_tx_response: boltz_client::swaps::boltz::SubmarineClaimTxResponse,
        _refund_address: &str,
    ) -> Result<(), PaymentError> {
        Ok(())
    }

    fn create_receive_swap(
        &self,
        _req: boltz_client::swaps::boltz::CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError> {
        Ok(CreateReverseResponse {
            id: generate_random_string(4),
            invoice: "".to_string(),
            swap_tree: Self::mock_swap_tree(),
            lockup_address: "".to_string(),
            refund_public_key: Self::mock_public_key(),
            timeout_block_height: 0,
            onchain_amount: 0,
            blinding_key: None,
        })
    }

    fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError> {
        Ok(Some(ReversePair {
            hash: "".to_string(),
            rate: 0.0,
            limits: ReverseLimits {
                maximal: u64::MAX,
                minimal: 0,
            },
            fees: ReverseFees {
                percentage: 0.1,
                miner_fees: PairMinerFees {
                    lockup: 14,
                    claim: 100,
                },
            },
        }))
    }

    fn broadcast_tx(
        &self,
        _chain: boltz_client::network::Chain,
        tx_hex: &str,
    ) -> Result<String, PaymentError> {
        let tx = utils::deserialize_tx_hex(tx_hex)?;
        Ok(tx.txid().to_string())
    }

    fn create_status_stream(&self) -> Box<dyn crate::swapper::SwapperStatusStream> {
        Box::new(MockStatusStream::new())
    }

    fn check_for_mrh(
        &self,
        _invoice: &str,
    ) -> Result<Option<(String, boltz_client::bitcoin::Amount)>, PaymentError> {
        // Ok(Some(("".to_string(), 0.0)))
        unimplemented!()
    }

    fn get_bolt12_invoice(&self, _offer: &str, _amount_sat: u64) -> Result<String, PaymentError> {
        unimplemented!()
    }
}
