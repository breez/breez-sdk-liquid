#[cfg(test)]
mod test {
    use crate::{
        bitcoin, elements,
        model::{BtcHistory, ChainSwap, LBtcHistory, PaymentState, SwapMetadata},
        recover::{
            handlers::{tests::create_mock_lbtc_wallet_tx, ChainSendSwapHandler},
            model::{ChainSwapRecoveryContext, TxMap},
        },
    };
    use bitcoin::OutPoint;
    use bitcoin::{transaction::Version, ScriptBuf, Sequence};
    use boltz_client::{Amount, LockTime};
    use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;

    use std::{collections::HashMap, str::FromStr};

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_all]
    async fn test_recover_with_lbtc_lockup_and_btc_claim() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Add LBTC lockup tx to history
        let lbtc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_liquid_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let lbtc_lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_lbtc_outgoing_tx_to_context(
            recovery_context,
            &lbtc_lockup_script,
            lbtc_lockup_tx_id,
            100, // Confirmed height
        );

        // Add BTC claim tx to history
        let btc_claim_script = chain_swap
            .get_claim_swap_script()
            .unwrap()
            .as_bitcoin_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let btc_lockup_tx_id = "2222222222222222222222222222222222222222222222222222222222222222";
        let btc_claim_tx_id = "3333333333333333333333333333333333333333333333333333333333333333";

        let recovery_context = add_btc_history_to_context(
            recovery_context,
            &btc_claim_script,
            &[
                (btc_lockup_tx_id, 102), // Server lockup tx
                (btc_claim_tx_id, 101),  // Claim tx
            ],
            &[create_btc_transaction(
                btc_lockup_tx_id,
                &btc_claim_script,
                100000,
            )],
        );

        // Test recover swap
        let result = ChainSendSwapHandler::recover_swap(
            &mut chain_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(chain_swap.state, PaymentState::Complete);
        assert_eq!(
            chain_swap.user_lockup_tx_id,
            Some(lbtc_lockup_tx_id.to_string())
        );
        assert_eq!(
            chain_swap.server_lockup_tx_id,
            Some(btc_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.claim_tx_id, Some(btc_claim_tx_id.to_string()));
        assert_eq!(chain_swap.refund_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_with_lbtc_lockup_only() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Add LBTC lockup tx to history
        let lbtc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_liquid_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let lbtc_lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_lbtc_outgoing_tx_to_context(
            recovery_context,
            &lbtc_lockup_script,
            lbtc_lockup_tx_id,
            100, // Confirmed height
        );

        // Test recover swap
        let result = ChainSendSwapHandler::recover_swap(
            &mut chain_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(chain_swap.state, PaymentState::Pending); // Not expired -> Pending
        assert_eq!(
            chain_swap.user_lockup_tx_id,
            Some(lbtc_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.server_lockup_tx_id, None);
        assert_eq!(chain_swap.claim_tx_id, None);
        assert_eq!(chain_swap.refund_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_with_lbtc_lockup_and_refund() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Add LBTC lockup tx to history
        let lbtc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_liquid_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let lbtc_lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_lbtc_outgoing_tx_to_context(
            recovery_context,
            &lbtc_lockup_script,
            lbtc_lockup_tx_id,
            100, // Confirmed height
        );

        // Add refund tx to history
        let lbtc_refund_tx_id = "4444444444444444444444444444444444444444444444444444444444444444";
        let recovery_context = add_lbtc_incoming_tx_to_context(
            recovery_context,
            &lbtc_lockup_script,
            lbtc_refund_tx_id,
            102, // Confirmed
            50000,
        );

        // Test recover swap
        let result = ChainSendSwapHandler::recover_swap(
            &mut chain_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(chain_swap.state, PaymentState::Failed);
        assert_eq!(
            chain_swap.user_lockup_tx_id,
            Some(lbtc_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.refund_tx_id, Some(lbtc_refund_tx_id.to_string()));
        assert_eq!(chain_swap.server_lockup_tx_id, None);
        assert_eq!(chain_swap.claim_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_expired_swap() {
        // Setup mock data
        let (chain_swap, mut recovery_context) = setup_test_data();

        // Make the swap expired
        recovery_context.liquid_tip_height = chain_swap.timeout_block_height;

        test_recover_expired_swap_common(chain_swap, recovery_context).await;
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_expired_swap_claim() {
        // Setup mock data
        let (chain_swap, mut recovery_context) = setup_test_data();

        // Make the swap expired
        recovery_context.bitcoin_tip_height = chain_swap.claim_timeout_block_height;

        test_recover_expired_swap_common(chain_swap, recovery_context).await;
    }

    async fn test_recover_expired_swap_common(
        mut chain_swap: ChainSwap,
        recovery_context: ChainSwapRecoveryContext,
    ) {
        // Add LBTC lockup tx to history
        let lbtc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_liquid_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let lbtc_lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_lbtc_outgoing_tx_to_context(
            recovery_context,
            &lbtc_lockup_script,
            lbtc_lockup_tx_id,
            100, // Confirmed height
        );

        // Test recover swap
        let result = ChainSendSwapHandler::recover_swap(
            &mut chain_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(chain_swap.state, PaymentState::RefundPending); // Expired -> RefundPending
        assert_eq!(
            chain_swap.user_lockup_tx_id,
            Some(lbtc_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.refund_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_with_no_transactions() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Test recover swap (no transactions in history)
        let result = ChainSendSwapHandler::recover_swap(
            &mut chain_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(chain_swap.state, PaymentState::Created); // No change since no tx info
        assert_eq!(chain_swap.user_lockup_tx_id, None);
        assert_eq!(chain_swap.refund_tx_id, None);
        assert_eq!(chain_swap.server_lockup_tx_id, None);
        assert_eq!(chain_swap.claim_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recovery_within_grace_period() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Set existing tx IDs
        chain_swap.user_lockup_tx_id = Some("existing-lockup-tx-id".to_string());
        chain_swap.refund_tx_id = Some("existing-refund-tx-id".to_string());
        chain_swap.server_lockup_tx_id = Some("existing-server-lockup-tx-id".to_string());
        chain_swap.claim_tx_id = Some("existing-claim-tx-id".to_string());

        // Test recover swap (with grace period, but empty transaction history)
        let result = ChainSendSwapHandler::recover_swap(
            &mut chain_swap,
            &recovery_context,
            true, // Within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        // Original txids should be preserved due to grace period
        assert_eq!(
            chain_swap.user_lockup_tx_id,
            Some("existing-lockup-tx-id".to_string())
        );
        assert_eq!(
            chain_swap.refund_tx_id,
            Some("existing-refund-tx-id".to_string())
        );
        assert_eq!(
            chain_swap.server_lockup_tx_id,
            Some("existing-server-lockup-tx-id".to_string())
        );
        assert_eq!(
            chain_swap.claim_tx_id,
            Some("existing-claim-tx-id".to_string())
        );
    }

    // Helper function to setup test data
    fn setup_test_data() -> (ChainSwap, ChainSwapRecoveryContext) {
        // Create a test chain swap
        let chain_swap = ChainSwap {
            id: "7aSRLEvFAJX3".to_string(),                     
            description: Some("Test swap".to_string()),
            payer_amount_sat: 100000,
            receiver_amount_sat: 95000,
            actual_payer_amount_sat: None,
            pair_fees_json: r#"{"id":"L-BTC/BTC","rate":0.997,"limits":{"maximal":2000000,"minimal":10000,"maximalZeroConf":50000},"fees":{"percentage":0.5,"miner":200}}"#.to_string(),
            create_response_json: r#"{"claim_details":{"swapTree":{"claimLeaf":{"output":"82012088a9149667195b60a10b31c967bddb5e27018ae8b6a0cc882003638ddfdc02e0fef1af18a484a0488c1b36270edc95fe90546cb149c16d97f8ac","version":192},"refundLeaf":{"output":"209c1c861c296fc94eea3ba76e225d7d38755874a9ba3b2d74869512c5736852d5ad033c5b0db1","version":192}},"lockupAddress":"bc1p545knq6m5p5xuqswuux9kvf37vzpalr2xjp738zlp6cd23yjuz9sf5a6d3","serverPublicKey":"039c1c861c296fc94eea3ba76e225d7d38755874a9ba3b2d74869512c5736852d5","timeoutBlockHeight":875324,"amount":52221},"lockup_details":{"swapTree":{"claimLeaf":{"output":"82012088a9149667195b60a10b31c967bddb5e27018ae8b6a0cc88204ad74f69865a09bcff4699deb0b35c6784fb59ded212eedb9c520cb6e6c6d3d1ac","version":196},"refundLeaf":{"output":"20016278e04cfd8721ad8a87cf7ffa255ebdd22b3bb21a160dd8470a2aea44de81ad03a05a30b1","version":196}},"lockupAddress":"lq1pqgpec4sq2mav432r8ukrr80d59rfpjv6qlqc5jrmwl5u5l2qazsh0astupq4jfr7r58sxp4kfvxy3nm49e9x4ecs9jurmwp45xmmavhejmcdcajegnpt","serverPublicKey":"034ad74f69865a09bcff4699deb0b35c6784fb59ded212eedb9c520cb6e6c6d3d1","timeoutBlockHeight":3168928,"amount":54292,"blindingKey":"bbbfd86767009007a58862706f32f709bcd060422ed47dc375ef76afa8e0e478","bip21":"liquidnetwork:lq1pqgpec4sq2mav432r8ukrr80d59rfpjv6qlqc5jrmwl5u5l2qazsh0astupq4jfr7r58sxp4kfvxy3nm49e9x4ecs9jurmwp45xmmavhejmcdcajegnpt?amount=0.00054292&label=Send%20to%20BTC%20address&assetid=6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d"}}"#.to_string(),
            claim_private_key: "ba2e8fc169022b9eb11ff9312b8bc6a6187af70c83a86afc780211a222fca194".to_string(),
            refund_private_key: "3efcaa05843a536cc028360c98af8fa57b8703a824cf0973213bec0f033c499a".to_string(),
            user_lockup_tx_id: None,
            server_lockup_tx_id: None,
            claim_tx_id: None,
            refund_address: None,
            refund_tx_id: None,
            claim_address: None,
            created_at: 1000,
            timeout_block_height: 10000,
            claim_timeout_block_height: 1000,
            state: PaymentState::Created,
            metadata: SwapMetadata {
                version: 1,
                last_updated_at: 1000,
                is_local: true,
            },
            direction: crate::model::Direction::Outgoing,
            lockup_address: "lq1pqgpec4sq2mav432r8ukrr80d59rfpjv6qlqc5jrmwl5u5l2qazsh0astupq4jfr7r58sxp4kfvxy3nm49e9x4ecs9jurmwp45xmmavhejmcdcajegnpt".to_string(),
            preimage: "72a639c5c09e0c68a15e7765f8eaa0efb0cb064c7c1e753bb9d5da907edbc427".to_string(),
            accepted_receiver_amount_sat: None,
            claim_fees_sat: 1221,
            accept_zero_conf: true,
            auto_accepted_fees: true,
        };

        // Create empty recovery context
        let recovery_context = ChainSwapRecoveryContext {
            lbtc_script_to_history_map: HashMap::new(),
            btc_script_to_history_map: HashMap::new(),
            btc_script_to_txs_map: HashMap::new(),
            btc_script_to_balance_map: HashMap::new(),
            tx_map: TxMap {
                outgoing_tx_map: HashMap::new(),
                incoming_tx_map: HashMap::new(),
            },
            liquid_tip_height: 900, // Below timeout height
            bitcoin_tip_height: 900,
            master_blinding_key: MasterBlindingKey::from_seed(&[]),
        };

        (chain_swap, recovery_context)
    }

    // Helper to add an LBTC outgoing transaction (lockup) to the recovery context
    fn add_lbtc_outgoing_tx_to_context(
        mut context: ChainSwapRecoveryContext,
        script: &elements::Script,
        tx_id_hex: &str,
        height: u32,
    ) -> ChainSwapRecoveryContext {
        let tx_id = elements::Txid::from_str(tx_id_hex).unwrap();

        // Create history tx
        let history_tx = LBtcHistory {
            txid: tx_id,
            height: height as i32,
        };

        // Add to script history map
        let mut script_history = context
            .lbtc_script_to_history_map
            .get(script)
            .cloned()
            .unwrap_or_default();
        script_history.push(history_tx);
        context
            .lbtc_script_to_history_map
            .insert(script.clone(), script_history);

        // Create wallet tx
        let wallet_tx = create_mock_lbtc_wallet_tx(tx_id_hex, height, -100000); // Negative amount for outgoing

        // Add to outgoing tx map
        context.tx_map.outgoing_tx_map.insert(tx_id, wallet_tx);

        context
    }

    // Helper to add an LBTC incoming transaction (refund) to the recovery context
    fn add_lbtc_incoming_tx_to_context(
        mut context: ChainSwapRecoveryContext,
        script: &elements::Script,
        tx_id_hex: &str,
        height: u32,
        amount: u64,
    ) -> ChainSwapRecoveryContext {
        let tx_id = elements::Txid::from_str(tx_id_hex).unwrap();

        // Create history tx
        let history_tx = LBtcHistory {
            txid: tx_id,
            height: height as i32,
        };

        // Add to script history map
        let mut script_history = context
            .lbtc_script_to_history_map
            .get(script)
            .cloned()
            .unwrap_or_default();
        script_history.push(history_tx);
        context
            .lbtc_script_to_history_map
            .insert(script.clone(), script_history);

        // Create wallet tx
        let wallet_tx = create_mock_lbtc_wallet_tx(tx_id_hex, height, amount as i64); // Positive for incoming

        // Add to incoming tx map
        context.tx_map.incoming_tx_map.insert(tx_id, wallet_tx);

        context
    }

    // Helper to add BTC transactions and history to the recovery context
    fn add_btc_history_to_context(
        mut context: ChainSwapRecoveryContext,
        script: &bitcoin::ScriptBuf,
        tx_ids: &[(/*tx_id*/ &str, /*height*/ u32)],
        txs: &[bitcoin::Transaction],
    ) -> ChainSwapRecoveryContext {
        // Add history txs
        let mut history = Vec::new();
        for (tx_id_hex, height) in tx_ids {
            let tx_id = bitcoin::Txid::from_str(tx_id_hex).unwrap();
            history.push(BtcHistory {
                txid: tx_id.to_string().parse().unwrap(),
                height: *height as i32,
            });
        }

        // Add to history map
        context
            .btc_script_to_history_map
            .insert(script.clone(), history);

        // Add to txs map
        context
            .btc_script_to_txs_map
            .insert(script.clone(), txs.to_vec());

        context
    }

    // Create a simple BTC transaction
    fn create_btc_transaction(
        tx_id_hex: &str,
        claim_script: &bitcoin::ScriptBuf,
        amount: u64,
    ) -> bitcoin::Transaction {
        let prev_tx_id = bitcoin::Txid::from_str(tx_id_hex).unwrap();
        bitcoin::Transaction {
            version: Version::TWO,
            lock_time: LockTime::from_height(0).unwrap(),
            input: vec![bitcoin::TxIn {
                previous_output: OutPoint {
                    txid: prev_tx_id,
                    vout: 0,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence(1),
                witness: bitcoin::Witness::new(),
            }],
            output: vec![bitcoin::TxOut {
                value: Amount::from_sat(amount),
                script_pubkey: claim_script.clone(),
            }],
        }
    }
}
