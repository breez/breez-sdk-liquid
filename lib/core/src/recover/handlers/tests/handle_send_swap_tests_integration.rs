#[cfg(test)]
mod test {
    use crate::chain::liquid::MockLiquidChainService;
    use crate::error::PaymentError;
    use crate::prelude::*;
    use crate::recover::handlers::tests::{
        create_empty_lbtc_transaction, create_mock_lbtc_wallet_tx,
    };
    use crate::recover::handlers::SendSwapHandler;
    use crate::recover::model::*;
    use crate::swapper::MockSwapper;
    use lwk_wollet::elements::script::Script;
    use lwk_wollet::elements::{AssetId, Txid};
    use lwk_wollet::WalletTx;
    use mockall::predicate::*;
    use sdk_common::utils::Arc;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    /// Test recovery with a claim transaction and preimage
    #[sdk_macros::async_test_all]
    async fn test_recover_with_claim_tx_and_preimage() {
        // Setup mock data
        let (mut send_swap, recovery_context) = setup_test_data();

        // Create a lockup tx
        let swap_script = send_swap.get_swap_script().unwrap();
        let lockup_script = swap_script.funding_addrs.unwrap().script_pubkey();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let mut recovery_context = add_outgoing_tx_to_context(
            recovery_context,
            &lockup_script,
            lockup_tx_id,
            100, // Confirmed height
        );

        // Setup claim tx with preimage
        let claim_tx_id = "2222222222222222222222222222222222222222222222222222222222222222";
        let preimage = "49666c97f6cea07fa5780c22ece1f0c9957caf1e3c37b9037b4f64dc6d09be7f"; // base64 of "somepreimage1234567890"
        let claim_tx = create_empty_lbtc_transaction();

        // Setup the mock chain service to return our claim tx
        let mut mock_liquid_chain_service = MockLiquidChainService::new();
        mock_liquid_chain_service
            .expect_get_transactions()
            .returning(move |_| Ok(vec![claim_tx.clone()]));

        recovery_context.liquid_chain_service = Arc::new(mock_liquid_chain_service);

        let mut swapper = MockSwapper::new();
        swapper
            .expect_get_submarine_preimage()
            .returning(move |_| Ok(preimage.to_string()));
        recovery_context.swapper = Arc::new(swapper);

        // Add the claim tx to history
        let recovery_context = add_claim_tx_to_context(
            recovery_context,
            &lockup_script,
            claim_tx_id,
            101, // Confirmed
        );

        // Test recover swap
        let result = SendSwapHandler::recover_swap(
            &mut send_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(send_swap.state, PaymentState::Complete);
        assert_eq!(send_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
        assert_eq!(send_swap.preimage, Some(preimage.to_string()));
    }

    /// Test recovery with a lockup and refund transaction
    #[sdk_macros::async_test_all]
    async fn test_recover_with_refund_tx() {
        // Setup mock data
        let (mut send_swap, mut recovery_context) = setup_test_data();

        // Setup mock swapper
        let mut swapper = MockSwapper::new();
        swapper
            .expect_get_submarine_preimage()
            .returning(move |_| Err(PaymentError::generic("No preimage available")));
        recovery_context.swapper = Arc::new(swapper);

        // Create a lockup tx
        let swap_script = send_swap.get_swap_script().unwrap();
        let lockup_script = swap_script.funding_addrs.unwrap().script_pubkey();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_outgoing_tx_to_context(
            recovery_context,
            &lockup_script,
            lockup_tx_id,
            100, // Confirmed height
        );

        // Add a refund tx to history
        let refund_tx_id = "3333333333333333333333333333333333333333333333333333333333333333";
        let (recovery_context, _) = add_incoming_tx_to_context(
            recovery_context,
            &lockup_script,
            refund_tx_id,
            102,   // Confirmed
            50000, // Refund amount
        );

        // Test recover swap
        let result = SendSwapHandler::recover_swap(
            &mut send_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(send_swap.state, PaymentState::Failed); // Confirmed refund -> Failed
        assert_eq!(send_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
        assert_eq!(send_swap.refund_tx_id, Some(refund_tx_id.to_string()));
    }

    /// Test recovery with only a lockup tx (not expired)
    #[sdk_macros::async_test_all]
    async fn test_recover_with_lockup_only() {
        // Setup mock data
        let (mut send_swap, mut recovery_context) = setup_test_data();

        // Setup mock swapper
        let preimage = "49666c97f6cea07fa5780c22ece1f0c9957caf1e3c37b9037b4f64dc6d09be7f"; // base64 of "somepreimage1234567890"
        let mut swapper = MockSwapper::new();
        swapper
            .expect_get_submarine_preimage()
            .returning(move |_| Ok(preimage.to_string()));
        recovery_context.swapper = Arc::new(swapper);

        // Create a lockup tx
        let swap_script = send_swap.get_swap_script().unwrap();
        let lockup_script = swap_script.funding_addrs.unwrap().script_pubkey();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_outgoing_tx_to_context(
            recovery_context,
            &lockup_script,
            lockup_tx_id,
            100, // Confirmed height
        );

        // Test recover swap
        let result = SendSwapHandler::recover_swap(
            &mut send_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(send_swap.state, PaymentState::Complete); // Not expired -> Complete
        assert_eq!(send_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
        assert_eq!(send_swap.refund_tx_id, None);
    }

    /// Test recovery with only a lockup tx (expired)
    #[sdk_macros::async_test_all]
    async fn test_recover_with_lockup_expired() {
        // Setup mock data
        let (mut send_swap, mut recovery_context) = setup_test_data();

        // Setup mock swapper
        let mut swapper = MockSwapper::new();
        swapper
            .expect_get_submarine_preimage()
            .returning(move |_| Err(PaymentError::generic("Swap expired")));
        recovery_context.swapper = Arc::new(swapper);

        // Set tip height to make swap expired
        recovery_context.liquid_tip_height = send_swap.timeout_block_height as u32 + 10;

        // Create a lockup tx
        let swap_script = send_swap.get_swap_script().unwrap();
        let lockup_script = swap_script.funding_addrs.unwrap().script_pubkey();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_outgoing_tx_to_context(
            recovery_context,
            &lockup_script,
            lockup_tx_id,
            100, // Confirmed height
        );

        // Test recover swap
        let result = SendSwapHandler::recover_swap(
            &mut send_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(send_swap.state, PaymentState::RefundPending); // Expired -> RefundPending
        assert_eq!(send_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
        assert_eq!(send_swap.refund_tx_id, None);
    }

    /// Test recovery with unconfirmed refund tx
    #[sdk_macros::async_test_all]
    async fn test_recover_with_unconfirmed_refund() {
        // Setup mock data
        let (mut send_swap, mut recovery_context) = setup_test_data();

        // Setup mock swapper
        let mut swapper = MockSwapper::new();
        swapper
            .expect_get_submarine_preimage()
            .returning(move |_| Err(PaymentError::generic("No preimage available")));
        recovery_context.swapper = Arc::new(swapper);

        // Create a lockup tx
        let swap_script = send_swap.get_swap_script().unwrap();
        let lockup_script = swap_script.funding_addrs.unwrap().script_pubkey();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_outgoing_tx_to_context(
            recovery_context,
            &lockup_script,
            lockup_tx_id,
            100, // Confirmed height
        );

        // Add an unconfirmed refund tx
        let refund_tx_id = "3333333333333333333333333333333333333333333333333333333333333333";
        let (recovery_context, _) = add_incoming_tx_to_context(
            recovery_context,
            &lockup_script,
            refund_tx_id,
            0,     // Unconfirmed
            50000, // Refund amount
        );

        // Test recover swap
        let result = SendSwapHandler::recover_swap(
            &mut send_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(send_swap.state, PaymentState::RefundPending); // Unconfirmed refund -> RefundPending
        assert_eq!(send_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
        assert_eq!(send_swap.refund_tx_id, Some(refund_tx_id.to_string()));
    }

    /// Test recovery with no transactions
    #[sdk_macros::async_test_all]
    async fn test_recover_with_no_transactions() {
        // Setup mock data
        let (mut send_swap, recovery_context) = setup_test_data();

        // Test recover swap (no transactions in history)
        let result = SendSwapHandler::recover_swap(
            &mut send_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(send_swap.state, PaymentState::Created); // No change since no tx info
        assert_eq!(send_swap.lockup_tx_id, None);
        assert_eq!(send_swap.refund_tx_id, None);
    }

    /// Test recovery when swap is within grace period
    #[sdk_macros::async_test_all]
    async fn test_recovery_within_grace_period() {
        // Setup mock data
        let (mut send_swap, recovery_context) = setup_test_data();

        // Set existing lockup and refund tx IDs
        send_swap.lockup_tx_id = Some("existing-lockup-tx-id".to_string());
        send_swap.refund_tx_id = Some("existing-refund-tx-id".to_string());

        // Test recover swap (with grace period, but empty transaction history)
        let result = SendSwapHandler::recover_swap(
            &mut send_swap,
            &recovery_context,
            true, // Within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        // Original txids should be preserved due to grace period
        assert_eq!(
            send_swap.lockup_tx_id,
            Some("existing-lockup-tx-id".to_string())
        );
        assert_eq!(
            send_swap.refund_tx_id,
            Some("existing-refund-tx-id".to_string())
        );
    }

    // Helper function to setup test data
    fn setup_test_data() -> (SendSwap, ReceiveOrSendSwapRecoveryContext) {
        // Create a test send swap
        let send_swap = SendSwap {
            id: "test-swap-id".to_string(),
            invoice: "lnbc10u1pnuv29epp5gzznge42jmaypq98xalte29hg8mddq7577uxvlv7alqtus2xgzjqhp52q6dyyex57klg3xhvl8tme05uwd5vghuuhm4av0gwgalx8vglr3qcqzzsxqyz5vqsp5l9r3lccpc258wq783c97n58wkk0ft5kqhdpp63ds75t93e7zu3ps9qxpqysgqmg2t6ry7mpr35jx6pq2ha960y2vwnl7050w87022g5qm00nyw83j47f8f6npr2yjmw2hennl8ewe87dj6nvg750zhcr3e94ypcxchdcp8q95j7".to_string(),
            bolt12_offer: None,
            payment_hash: Some("40853466aa96fa4080a7377ebca8b741f6d683d4f7b8667d9eefc0be414640a4".to_string()),
            destination_pubkey: Some("03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".to_string()),
            description: Some("Test payment".to_string()),
            preimage: None,
            payer_amount_sat: 100000,
            receiver_amount_sat: 95000,
            pair_fees_json: r#"{"hash":"BTC/BTC","rate":0.997,"limits":{"maximal":2000000,"minimal":10000,"maximalZeroConf":50000,"minimalBatched":21},"fees":{"percentage":0.5,"minerFees":200}}"#.to_string(),
            create_response_json: r#"{"accept_zero_conf":true,"address":"lq1pqg8hsjkptr8u7l35ctx5yn4dpwufkjxt7d24zuj5ddahnn7jaduh8r6celry8kn9xrkgwchrrx2madlemf0u27pnmjar4d4k5wvtem8kfl7ru56w94sv","bip21":"liquidnetwork:lq1pqg8hsjkptr8u7l35ctx5yn4dpwufkjxt7d24zuj5ddahnn7jaduh8r6celry8kn9xrkgwchrrx2madlemf0u27pnmjar4d4k5wvtem8kfl7ru56w94sv?amount=0.00001015&label=Send%20to%20BTC%20lightning&assetid=6f0279e9ed041c3d710a9f57d0c02928416460c4b722ae3457a11eec381c526d","claim_public_key":"0381b8583fe95488b961d12836102b1869b241972e571bd44a933d273b12a0d123","expected_amount":1015,"referral_id":"breez-sdk","swap_tree":{"claim_leaf":{"output":"a914cea2d1aa5af00fb688727b0054de58ecf45e948f882081b8583fe95488b961d12836102b1869b241972e571bd44a933d273b12a0d123ac","version":196},"refund_leaf":{"output":"20a668381222ff9076ca6d5f5b098b501331f07d5065f1dc0e0f217cc493359e69ad03b12432b1","version":196}},"timeout_block_height":3286193,"blinding_key":"73332603e5d438ddb3b12c16c7271c9f98658c77257cbb06639d05773aa1fec3"}"#.to_string(),
            lockup_tx_id: None,
            refund_address: None,
            refund_tx_id: None,
            created_at: 1000,
            timeout_block_height: 1000,
            state: PaymentState::Created,
            refund_private_key: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            metadata: SwapMetadata {
                version: 1,
                last_updated_at: 1000,
                is_local: true,
            },
        };

        // Create empty recovery context
        let recovery_context = ReceiveOrSendSwapRecoveryContext {
            lbtc_script_to_history_map: HashMap::new(),
            tx_map: TxMap {
                outgoing_tx_map: HashMap::new(),
                incoming_tx_map: HashMap::new(),
            },
            liquid_tip_height: 900, // Below timeout height
            swapper: Arc::new(MockSwapper::new()),
            liquid_chain_service: Arc::new(MockLiquidChainService::new()),
            lbtc_asset_id: AssetId::LIQUID_BTC,
        };

        (send_swap, recovery_context)
    }

    // Helper to add an outgoing transaction (lockup) to the recovery context
    fn add_outgoing_tx_to_context(
        mut context: ReceiveOrSendSwapRecoveryContext,
        script: &Script,
        tx_id_hex: &str,
        height: u32,
    ) -> ReceiveOrSendSwapRecoveryContext {
        let tx_id = Txid::from_str(tx_id_hex).unwrap();
        let asset_id = AssetId::from_slice(&[0; 32]).unwrap();

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
        script_history.push(history_tx.clone());
        context
            .lbtc_script_to_history_map
            .insert(script.clone(), script_history);

        // Create wallet tx
        let wallet_tx = create_mock_lbtc_wallet_tx(tx_id_hex, height, -100000, asset_id); // Negative amount for outgoing

        // Add to outgoing tx map
        context.tx_map.outgoing_tx_map.insert(tx_id, wallet_tx);

        context
    }

    // Helper to add a claim tx to the history without adding it to the wallet tx map
    fn add_claim_tx_to_context(
        mut context: ReceiveOrSendSwapRecoveryContext,
        script: &Script,
        tx_id_hex: &str,
        height: u32,
    ) -> ReceiveOrSendSwapRecoveryContext {
        let tx_id = Txid::from_str(tx_id_hex).unwrap();

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
        script_history.push(history_tx.clone());
        context
            .lbtc_script_to_history_map
            .insert(script.clone(), script_history);

        context
    }

    // Helper to add an incoming transaction (refund) to the recovery context
    fn add_incoming_tx_to_context(
        mut context: ReceiveOrSendSwapRecoveryContext,
        script: &Script,
        tx_id_hex: &str,
        height: u32,
        amount: u64,
    ) -> (ReceiveOrSendSwapRecoveryContext, WalletTx) {
        let tx_id = Txid::from_str(tx_id_hex).unwrap();
        let asset_id = AssetId::from_slice(&[0; 32]).unwrap();

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
        script_history.push(history_tx.clone());
        context
            .lbtc_script_to_history_map
            .insert(script.clone(), script_history);

        // Create wallet tx
        let wallet_tx = create_mock_lbtc_wallet_tx(tx_id_hex, height, amount as i64, asset_id); // Positive amount for incoming

        // Add to incoming tx map
        context
            .tx_map
            .incoming_tx_map
            .insert(tx_id, wallet_tx.clone());

        (context, wallet_tx)
    }
}
