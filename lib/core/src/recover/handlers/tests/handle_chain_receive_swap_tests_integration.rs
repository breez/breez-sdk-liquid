#[cfg(test)]
mod test {
    use crate::{
        bitcoin, elements,
        model::{BtcHistory, BtcScriptBalance, ChainSwap, LBtcHistory, PaymentState, SwapMetadata},
        recover::{
            handlers::{tests::create_mock_lbtc_wallet_tx, ChainReceiveSwapHandler},
            model::{ChainSwapRecoveryContext, TxMap},
        },
    };
    use bitcoin::{transaction::Version, Sequence};
    use boltz_client::{Amount, LockTime};
    use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;
    use std::{collections::HashMap, str::FromStr};

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::async_test_all]
    async fn test_recover_with_btc_lockup_and_lbtc_claim() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Add BTC lockup tx to history
        let btc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_bitcoin_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let (recovery_context, btc_lockup_tx_id) = add_btc_lockup_to_context(
            recovery_context,
            &btc_lockup_script,
            100, // Confirmed height
            chain_swap.payer_amount_sat,
        );

        // Add LBTC claim tx to history
        let lbtc_claim_script = chain_swap
            .get_claim_swap_script()
            .unwrap()
            .as_liquid_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let lbtc_server_lockup_tx_id =
            "2222222222222222222222222222222222222222222222222222222222222222";
        let lbtc_claim_tx_id = "3333333333333333333333333333333333333333333333333333333333333333";

        let recovery_context = add_lbtc_history_to_context(
            recovery_context,
            &lbtc_claim_script,
            &[(lbtc_server_lockup_tx_id, 101), (lbtc_claim_tx_id, 102)],
            lbtc_claim_tx_id,
            chain_swap.receiver_amount_sat,
        );

        // Test recover swap
        let result = ChainReceiveSwapHandler::recover_swap(
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
            Some(btc_lockup_tx_id.to_string())
        );
        assert_eq!(
            chain_swap.server_lockup_tx_id,
            Some(lbtc_server_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.claim_tx_id, Some(lbtc_claim_tx_id.to_string()));
        assert_eq!(chain_swap.refund_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_with_btc_lockup_only() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Add BTC lockup tx to history
        let btc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_bitcoin_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let (recovery_context, btc_lockup_tx_id) = add_btc_lockup_to_context(
            recovery_context,
            &btc_lockup_script,
            100, // Confirmed height
            chain_swap.payer_amount_sat,
        );

        // Test recover swap
        let result = ChainReceiveSwapHandler::recover_swap(
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
            Some(btc_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.server_lockup_tx_id, None);
        assert_eq!(chain_swap.claim_tx_id, None);
        assert_eq!(chain_swap.refund_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_with_btc_lockup_and_refund() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Add BTC lockup tx to history
        let btc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_bitcoin_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let (recovery_context, btc_lockup_tx_id, btc_refund_tx_id) =
            add_btc_lockup_and_refund_to_context(
                recovery_context,
                &btc_lockup_script,
                102, // Confirmed height
                100, // Confirmed height
                chain_swap.payer_amount_sat,
            );

        // Test recover swap
        let result = ChainReceiveSwapHandler::recover_swap(
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
            Some(btc_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.refund_tx_id, Some(btc_refund_tx_id.to_string()));
        assert_eq!(chain_swap.server_lockup_tx_id, None);
        assert_eq!(chain_swap.claim_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_expired_swap() {
        // Setup mock data
        let (chain_swap, mut recovery_context) = setup_test_data();

        // Make the swap expired
        recovery_context.bitcoin_tip_height = chain_swap.timeout_block_height;

        test_recover_expired_swap_common(chain_swap, recovery_context).await;
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_expired_swap_claim() {
        // Setup mock data
        let (chain_swap, mut recovery_context) = setup_test_data();

        // Make the swap expired
        recovery_context.liquid_tip_height = chain_swap.claim_timeout_block_height;

        test_recover_expired_swap_common(chain_swap, recovery_context).await;
    }

    async fn test_recover_expired_swap_common(
        mut chain_swap: ChainSwap,
        recovery_context: ChainSwapRecoveryContext,
    ) {
        // Add BTC lockup tx to history
        let btc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_bitcoin_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let (mut recovery_context, btc_lockup_tx_id) = add_btc_lockup_to_context(
            recovery_context,
            &btc_lockup_script,
            100, // Confirmed height
            chain_swap.payer_amount_sat,
        );

        // Add balance to the lockup address to simulate funds still there
        recovery_context.btc_script_to_balance_map.insert(
            btc_lockup_script.clone(),
            BtcScriptBalance {
                confirmed: chain_swap.payer_amount_sat,
                unconfirmed: 0,
            },
        );

        // Test recover swap
        let result = ChainReceiveSwapHandler::recover_swap(
            &mut chain_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(chain_swap.state, PaymentState::Refundable); // Expired with funds -> Refundable
        assert_eq!(
            chain_swap.user_lockup_tx_id,
            Some(btc_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.refund_tx_id, None);
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_with_incorrect_amount() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Expected amount
        chain_swap.payer_amount_sat = 100000;

        // Add BTC lockup tx to history with wrong amount
        let btc_lockup_script = chain_swap
            .get_lockup_swap_script()
            .unwrap()
            .as_bitcoin_script()
            .unwrap()
            .funding_addrs
            .unwrap()
            .script_pubkey();

        let (recovery_context, btc_lockup_tx_id) = add_btc_lockup_to_context(
            recovery_context,
            &btc_lockup_script,
            100,   // Confirmed height
            50000, // Less than expected
        );

        // Test recover swap
        let result = ChainReceiveSwapHandler::recover_swap(
            &mut chain_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(chain_swap.state, PaymentState::Refundable); // Wrong amount -> Refundable
        assert_eq!(
            chain_swap.user_lockup_tx_id,
            Some(btc_lockup_tx_id.to_string())
        );
        assert_eq!(chain_swap.actual_payer_amount_sat, Some(50000));
    }

    #[sdk_macros::async_test_all]
    async fn test_recover_with_no_transactions() {
        // Setup mock data
        let (mut chain_swap, recovery_context) = setup_test_data();

        // Test recover swap (no transactions in history)
        let result = ChainReceiveSwapHandler::recover_swap(
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
        chain_swap.claim_address = Some("existing-claim-address".to_string());

        // Test recover swap (with grace period, but empty transaction history)
        let result = ChainReceiveSwapHandler::recover_swap(
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
        assert_eq!(
            chain_swap.claim_address,
            Some("existing-claim-address".to_string())
        );
    }

    // Helper function to setup test data
    fn setup_test_data() -> (ChainSwap, ChainSwapRecoveryContext) {
        // Create a test chain swap
        let chain_swap = ChainSwap {
            id: "80MALuoKqcX".to_string(),
            description: Some("Test swap".to_string()),
            payer_amount_sat: 100000,
            receiver_amount_sat: 95000,
            actual_payer_amount_sat: None,
            pair_fees_json: r#"{"id":"BTC/L-BTC","rate":0.997,"limits":{"maximal":2000000,"minimal":10000,"maximalZeroConf":50000},"fees":{"percentage":0.5,"miner":200}}"#.to_string(),
            create_response_json: r#"{"claim_details":{"swapTree":{"claimLeaf":{"output":"82012088a914afc2dd75ff1251e5142f0d2b4c484ae515bea2d88820daa822253d2de1da7728d41a5cdcf429d63e75096a10a03ecf777b26cd5f9bebac","version":196},"refundLeaf":{"output":"208401dec9e0a804f6297a6e0d3a683fdb927ce888a3997c824556504ef74c36c2ad03683d31b1","version":196}},"lockupAddress":"lq1pqg078dfv0880qtxs04mnz85fmj00x0va09y0cfkg2mn764w9pxd5d850e6sy86t6nzdsumuq970d0592f5r4kjjgkt04a3dmy8jr3p04cm993udvn8x9","serverPublicKey":"038401dec9e0a804f6297a6e0d3a683fdb927ce888a3997c824556504ef74c36c2","timeoutBlockHeight":3226984,"amount":98968,"blindingKey":"9dd5dd64cd82c46564d5c361dd632c24d8c0a6c6b86ce62ba0a0d05d1ae158f7"},"lockup_details":{"swapTree":{"claimLeaf":{"output":"82012088a914afc2dd75ff1251e5142f0d2b4c484ae515bea2d88820c460e9ddee0f9762362183457a98ae32fb1f4e7fd5e4f400cd43def3cc40c701ac","version":192},"refundLeaf":{"output":"20d945dfc41ed339ae02aefa0576aea34a4fa2b3adfb990b9f910a93e3163877b7ad035c720db1","version":192}},"lockupAddress":"bc1p7kaqml56kyzw6gwczgmswdnuk3e5dazvjv9arajdndyj76aaafrs5qgzxa","serverPublicKey":"03c460e9ddee0f9762362183457a98ae32fb1f4e7fd5e4f400cd43def3cc40c701","timeoutBlockHeight":881244,"amount":100000,"bip21":"bitcoin:bc1p7kaqml56kyzw6gwczgmswdnuk3e5dazvjv9arajdndyj76aaafrs5qgzxa?amount=0.001&label=Send%20to%20L-BTC%20address"}}"#.to_string(),
            claim_private_key: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            refund_private_key: "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
            user_lockup_tx_id: None,
            server_lockup_tx_id: None,
            claim_tx_id: None,
            refund_address: None,
            refund_tx_id: None,
            claim_address: None,
            created_at: 1000,
            timeout_block_height: 1000,
            claim_timeout_block_height: 10000,
            state: PaymentState::Created,
            metadata: SwapMetadata {
                version: 1,
                last_updated_at: 1000,
                is_local: true,
            },
            direction: crate::model::Direction::Incoming,
            lockup_address: "bc1p7kaqml56kyzw6gwczgmswdnuk3e5dazvjv9arajdndyj76aaafrs5qgzxa".to_string(),
            preimage: "ce28697547391404b544ab8a108c6767fb7ad6c8e0cf133cdbab5c4e23403a62".to_string(),
            accepted_receiver_amount_sat: Some(95000),
            claim_fees_sat: 1000,
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

    // Helper to add a BTC lockup transaction to the recovery context
    fn add_btc_lockup_to_context(
        mut context: ChainSwapRecoveryContext,
        lockup_script: &bitcoin::ScriptBuf,
        height: u32,
        amount: u64,
    ) -> (ChainSwapRecoveryContext, String) {
        // Create a BTC transaction for the lockup
        let tx = bitcoin::Transaction {
            version: Version::TWO,
            lock_time: LockTime::from_height(0).unwrap(),
            input: vec![],
            output: vec![bitcoin::TxOut {
                value: Amount::from_sat(amount),
                script_pubkey: lockup_script.clone(),
            }],
        };

        // Compute the actual txid from the transaction
        let computed_txid = tx.compute_txid();
        let computed_txid_str = computed_txid.to_string();

        // Create history tx with the computed txid
        let history_tx = BtcHistory {
            txid: computed_txid.to_string().parse().unwrap(),
            height: height as i32,
        };

        // Add to script history map
        let mut script_history = context
            .btc_script_to_history_map
            .get(lockup_script)
            .cloned()
            .unwrap_or_default();
        script_history.push(history_tx);
        context
            .btc_script_to_history_map
            .insert(lockup_script.clone(), script_history);

        // Add transaction to txs map
        let mut txs = context
            .btc_script_to_txs_map
            .get(lockup_script)
            .cloned()
            .unwrap_or_default();
        txs.push(tx);
        context
            .btc_script_to_txs_map
            .insert(lockup_script.clone(), txs);

        // Set balance to 0 (funds have been used)
        context.btc_script_to_balance_map.insert(
            lockup_script.clone(),
            BtcScriptBalance {
                confirmed: 0,
                unconfirmed: 0,
            },
        );

        (context, computed_txid_str)
    }

    // Helper to add BTC lockup and refund transactions to the context
    fn add_btc_lockup_and_refund_to_context(
        context: ChainSwapRecoveryContext,
        lockup_script: &bitcoin::ScriptBuf,
        lockup_height: u32,
        refund_height: u32,
        amount: u64,
    ) -> (ChainSwapRecoveryContext, String, String) {
        // First add the lockup tx
        let (mut context, lockup_tx_id) =
            add_btc_lockup_to_context(context, lockup_script, lockup_height, amount);

        // Create a BTC transaction for the refund
        let lockup_bitcon_tx_id = bitcoin::Txid::from_str(&lockup_tx_id).unwrap();
        let refund_tx = bitcoin::Transaction {
            version: Version::TWO,
            lock_time: LockTime::from_height(0).unwrap(),
            input: vec![bitcoin::TxIn {
                previous_output: bitcoin::OutPoint {
                    txid: lockup_bitcon_tx_id,
                    vout: 0,
                },
                script_sig: bitcoin::ScriptBuf::new(),
                sequence: Sequence::default(),
                witness: bitcoin::Witness::new(),
            }],
            output: vec![bitcoin::TxOut {
                value: Amount::from_sat(amount - 1000),   // Subtract fee
                script_pubkey: bitcoin::ScriptBuf::new(), // Destination address
            }],
        };

        // Create history tx for refund
        let refund_history_tx = BtcHistory {
            txid: refund_tx.compute_txid(),
            height: refund_height as i32,
        };
        // Add refund tx to script history
        let mut script_history = context
            .btc_script_to_history_map
            .get(lockup_script)
            .cloned()
            .unwrap_or_default();

        script_history.push(refund_history_tx);
        context
            .btc_script_to_history_map
            .insert(lockup_script.clone(), script_history);

        // Add refund tx to txs map
        let mut txs = context
            .btc_script_to_txs_map
            .get(lockup_script)
            .cloned()
            .unwrap_or_default();
        txs.push(refund_tx.clone());
        context
            .btc_script_to_txs_map
            .insert(lockup_script.clone(), txs);

        (context, lockup_tx_id, refund_tx.compute_txid().to_string())
    }

    // Helper to add LBTC transactions to the history
    fn add_lbtc_history_to_context(
        mut context: ChainSwapRecoveryContext,
        claim_script: &elements::Script,
        tx_ids: &[(/*tx_id*/ &str, /*height*/ u32)],
        claim_tx_id_hex: &str,
        amount: u64,
    ) -> ChainSwapRecoveryContext {
        // Add history txs
        let mut history = Vec::new();
        for (tx_id_hex, height) in tx_ids {
            let tx_id = elements::Txid::from_str(tx_id_hex).unwrap();
            history.push(LBtcHistory {
                txid: tx_id,
                height: *height as i32,
            });

            // If this is the claim tx, add it to the incoming tx map
            if *tx_id_hex == claim_tx_id_hex {
                let wallet_tx = create_mock_lbtc_wallet_tx(tx_id_hex, *height, amount as i64);
                context.tx_map.incoming_tx_map.insert(tx_id, wallet_tx);
            }
        }

        // Add to history map
        context
            .lbtc_script_to_history_map
            .insert(claim_script.clone(), history);

        context
    }
}
