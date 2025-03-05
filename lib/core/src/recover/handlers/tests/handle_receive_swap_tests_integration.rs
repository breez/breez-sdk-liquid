#[cfg(test)]
mod test {
    use crate::{
        chain::liquid::MockLiquidChainService,
        model::{PaymentState, ReceiveSwap, SwapMetadata},
        recover::{
            handlers::ReceiveSwapHandler,
            model::{HistoryTxId, RecoveryContext, TxMap},
        },
        swapper::MockSwapper,
    };
    use boltz_client::ElementsAddress;
    use lwk_wollet::elements::{
        AssetId, LockTime, Script, Sequence, Transaction, TxIn, TxInWitness, Txid,
    };
    use lwk_wollet::elements_miniscript::slip77::MasterBlindingKey;
    use lwk_wollet::WalletTx;
    use std::{
        collections::{BTreeMap, HashMap},
        str::FromStr,
        sync::Arc,
    };

    #[tokio::test]
    async fn test_recover_with_claim_tx() {
        // Setup mock data
        let (mut receive_swap, recovery_context) = setup_test_data();

        // Setup a claim tx in the history
        let claim_script = receive_swap.claim_script().unwrap();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_lockup_tx_to_context(
            recovery_context,
            &claim_script,
            lockup_tx_id,
            100, // Confirmed
        );

        let claim_tx_id = "2222222222222222222222222222222222222222222222222222222222222222";
        let (recovery_context, _) = add_claim_tx_to_context(
            recovery_context,
            &claim_script,
            claim_tx_id,
            101,    // Confirmed
            100000, // Amount
        );

        // Test recover swap
        let result = ReceiveSwapHandler::recover_swap(
            &mut receive_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(receive_swap.state, PaymentState::Complete);
        assert_eq!(receive_swap.claim_tx_id, Some(claim_tx_id.to_string()));
        assert_eq!(receive_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
    }

    #[tokio::test]
    async fn test_recover_with_mrh_tx() {
        // Setup mock data
        let (mut receive_swap, recovery_context) = setup_test_data();

        // Setup an MRH tx in the history
        let mrh_script = ElementsAddress::from_str(&receive_swap.mrh_address)
            .unwrap()
            .script_pubkey();
        let mrh_tx_id = "3333333333333333333333333333333333333333333333333333333333333333";
        let (recovery_context, _) = add_mrh_tx_to_context(
            recovery_context,
            &mrh_script,
            mrh_tx_id,
            102,    // Confirmed
            150000, // Amount
        );

        // Test recover swap
        let result = ReceiveSwapHandler::recover_swap(
            &mut receive_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(receive_swap.state, PaymentState::Complete);
        assert_eq!(receive_swap.mrh_tx_id, Some(mrh_tx_id.to_string()));
    }

    #[tokio::test]
    async fn test_recover_expired_swap() {
        // Setup mock data
        let (mut receive_swap, mut recovery_context) = setup_test_data();

        // Set tip height to make swap expired
        recovery_context.liquid_tip_height = receive_swap.timeout_block_height + 10;

        // Test recover swap
        let result = ReceiveSwapHandler::recover_swap(
            &mut receive_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(receive_swap.state, PaymentState::Failed);
        assert_eq!(receive_swap.claim_tx_id, None);
        assert_eq!(receive_swap.mrh_tx_id, None);
    }

    #[tokio::test]
    async fn test_recover_with_lockup_no_claim() {
        // Setup mock data
        let (mut receive_swap, recovery_context) = setup_test_data();

        // Setup only a lockup tx in the history
        let claim_script = receive_swap.claim_script().unwrap();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_lockup_tx_to_context(
            recovery_context,
            &claim_script,
            lockup_tx_id,
            100, // Confirmed
        );

        // Test recover swap
        let result = ReceiveSwapHandler::recover_swap(
            &mut receive_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(receive_swap.state, PaymentState::Pending); // Should be pending
        assert_eq!(receive_swap.claim_tx_id, None); // No claim tx
        assert_eq!(receive_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
    }

    #[tokio::test]
    async fn test_recover_with_lockup_expired() {
        // Setup mock data
        let (mut receive_swap, mut recovery_context) = setup_test_data();

        // Make the swap expired
        recovery_context.liquid_tip_height = receive_swap.timeout_block_height + 10;

        // Setup only a lockup tx in the history
        let claim_script = receive_swap.claim_script().unwrap();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_lockup_tx_to_context(
            recovery_context,
            &claim_script,
            lockup_tx_id,
            100, // Confirmed
        );

        // Test recover swap
        let result = ReceiveSwapHandler::recover_swap(
            &mut receive_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(receive_swap.state, PaymentState::Failed); // Should be failed since expired
        assert_eq!(receive_swap.claim_tx_id, None); // No claim tx
        assert_eq!(receive_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
    }

    #[tokio::test]
    async fn test_recover_with_no_transactions() {
        // Setup mock data
        let (mut receive_swap, recovery_context) = setup_test_data();

        // Test recover swap (no transactions in history)
        let result = ReceiveSwapHandler::recover_swap(
            &mut receive_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(receive_swap.state, PaymentState::Created); // Should remain as created
        assert_eq!(receive_swap.claim_tx_id, None); // No claim tx
        assert_eq!(receive_swap.lockup_tx_id, None); // No lockup tx
    }

    #[tokio::test]
    async fn test_recovery_within_grace_period_claim() {
        // Setup mock data
        let (mut receive_swap, recovery_context) = setup_test_data();

        // Set existing claim tx ID in the swap
        receive_swap.claim_tx_id = Some("existing-claim-tx-id".to_string());

        // Test recover swap (with grace period, but no transactions in the chain)
        let result = ReceiveSwapHandler::recover_swap(
            &mut receive_swap,
            &recovery_context,
            true, // Within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        // Should still have the original claim_tx_id since we're in the grace period
        assert_eq!(
            receive_swap.claim_tx_id,
            Some("existing-claim-tx-id".to_string())
        );
    }

    #[tokio::test]
    async fn test_recover_with_unconfirmed_tx() {
        // Setup mock data
        let (mut receive_swap, recovery_context) = setup_test_data();

        // Setup an unconfirmed claim tx in the history
        let claim_script = receive_swap.claim_script().unwrap();

        let lockup_tx_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let recovery_context = add_lockup_tx_to_context(
            recovery_context,
            &claim_script,
            lockup_tx_id,
            100, // Confirmed
        );

        let claim_tx_id = "2222222222222222222222222222222222222222222222222222222222222222";
        let (recovery_context, _) = add_claim_tx_to_context(
            recovery_context,
            &claim_script,
            claim_tx_id,
            0,      // Unconfirmed (height = 0)
            100000, // Amount
        );

        // Test recover swap
        let result = ReceiveSwapHandler::recover_swap(
            &mut receive_swap,
            &recovery_context,
            false, // Not within grace period
        )
        .await;

        // Verify results
        assert!(result.is_ok());
        assert_eq!(receive_swap.state, PaymentState::Pending); // Should be pending
        assert_eq!(receive_swap.claim_tx_id, Some(claim_tx_id.to_string()));
        assert_eq!(receive_swap.lockup_tx_id, Some(lockup_tx_id.to_string()));
    }

    // Helper function to setup test data
    fn setup_test_data() -> (ReceiveSwap, RecoveryContext) {
        // Create a test receive swap
        let receive_swap = ReceiveSwap {
            id: "test-swap-id".to_string(),
            preimage: "5747ef5affdf79f693ea56e6e65bb68718f57f160a92197bcac2fd456cb06edd".to_string(),
            create_response_json: r#"{"swap_tree":{"claim_leaf":{"output":"82012088a91460bac83421a184c3cf912ae231df8e3f0ce6ac5488204c9f9e348b27b1257c51f3ad2a05589ac8f3af72246ff3094950441cdf826b47ac","version":196},"refund_leaf":{"output":"209916729fe59068c8544b8070a32f653ed9cb550e76a5caaeda557aadf9e2cc2fad03e48c31b1","version":196}},"lockup_address":"lq1pqw632yu95t23pa7jr4s746g68nwl0ukkfvncs3q7t66f9gjqj7ccj2nwx8verw57l2zn029vlnwjuvrpm4yxnz3tccfks9e8rdy2r9tu586g8fya887j","refund_public_key":"029916729fe59068c8544b8070a32f653ed9cb550e76a5caaeda557aadf9e2cc2f","timeout_block_height":3247332,"onchain_amount":1071,"blinding_key":"605f50d0c0516c800594e1d44b9ceaeb7fa7a4258d6357043cc2daaa13e48895"}"#.to_string(),
            claim_private_key: "2f23dbb3c13e30ac8df594369b62ef1eb34a50197d7acc15db413961d90810e5".to_string(),
            invoice: "lnbc11u1pn65lr9sp5xfmwgmaddn2acwc7rr4xhj3k5dy4tyfhma57tpfp0z7eyp90fdjspp5a48w03jc5dtzqnyyqw727naffpcdvhj7s9hen45zh9m3auhfmx2qdpz2djkuepqw3hjqnpdgf2yxgrpv3j8yetnwvxqyp2xqcqz95rzjqfxfl8353vnmzftu28e662s9tzdv3ua0wgjxlucff9gyg8xlsf45wzzxeyqq28qqqqqqqqqqqqqqq9gq2y9qyysgq37h36xnz7khazpus03846hml4q8y8qekrzwh5ql36fy6l7dmgyuq3d9jyvnmm3h8tmxn7ae20wgte2elq4akpu3mqnyj626zy69drmqq95tqch".to_string(),
            payment_hash: Some("ed4ee7c658a356204c8403bcaf4fa94870d65e5e816f99d682b9771ef2e9d994".to_string()),
            destination_pubkey: Some("03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".to_string()),
            description: Some("Test payment".to_string()),
            payer_amount_sat: 100000,
            receiver_amount_sat: 95000,
            pair_fees_json: r#"{"id":"BTC/BTC","rate":0.997,"limits":{"maximal":2000000,"minimal":10000,"maximalZeroConf":50000},"fees":{"percentage":0.5,"miner":200}}"#.to_string(),
            claim_fees_sat: 500,
            claim_tx_id: None,
            lockup_tx_id: None,
            mrh_address: "lq1qqvynd50t4tajashdguell7nu9gycuqqd869w8vqww9ys9dsz7szdfeu7pwe4yzzme28qsluyfyrtqmq9scl5ydw4lesx3c5qu".to_string(),
            mrh_tx_id: None,
            created_at: 1000,
            timeout_block_height: 1000,
            state: PaymentState::Created,
            metadata: SwapMetadata {
                version: 1,
                last_updated_at: 1000,
                is_local: true,
            },
        };

        // Create empty recovery context
        let recovery_context = RecoveryContext {
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
            swapper: Arc::new(MockSwapper::new()),
            liquid_chain_service: Arc::new(MockLiquidChainService::new()),
        };

        (receive_swap, recovery_context)
    }

    // Helper to add a claim transaction to the recovery context
    fn add_claim_tx_to_context(
        mut context: RecoveryContext,
        claim_script: &Script,
        tx_id_hex: &str,
        height: u32,
        amount: u64,
    ) -> (RecoveryContext, WalletTx) {
        let tx_id = Txid::from_str(tx_id_hex).unwrap();

        // Create history tx
        let history_tx = HistoryTxId {
            txid: tx_id,
            height: height as i32,
        };

        // Add to script history map
        let mut script_history = context
            .lbtc_script_to_history_map
            .get(claim_script)
            .cloned()
            .unwrap_or_default();
        script_history.push(history_tx.clone());
        context
            .lbtc_script_to_history_map
            .insert(claim_script.clone(), script_history);

        // Create wallet tx
        let wallet_tx = create_mock_wallet_tx(tx_id_hex, height, amount);

        // Add to incoming tx map
        context
            .tx_map
            .incoming_tx_map
            .insert(tx_id, wallet_tx.clone());

        (context, wallet_tx)
    }

    fn add_lockup_tx_to_context(
        mut context: RecoveryContext,
        lockup_script: &Script,
        tx_id_hex: &str,
        height: u32,
    ) -> RecoveryContext {
        let tx_id = Txid::from_str(tx_id_hex).unwrap();

        // Create history tx
        let history_tx = HistoryTxId {
            txid: tx_id,
            height: height as i32,
        };

        // Add to script history map
        let mut script_history = context
            .lbtc_script_to_history_map
            .get(lockup_script)
            .cloned()
            .unwrap_or_default();
        script_history.push(history_tx.clone());
        context
            .lbtc_script_to_history_map
            .insert(lockup_script.clone(), script_history);

        context
    }

    // Helper to add an MRH transaction to the recovery context
    fn add_mrh_tx_to_context(
        mut context: RecoveryContext,
        mrh_script: &Script,
        tx_id_hex: &str,
        height: u32,
        amount: u64,
    ) -> (RecoveryContext, WalletTx) {
        let tx_id = Txid::from_str(tx_id_hex).unwrap();

        // Create history tx
        let history_tx = HistoryTxId {
            txid: tx_id,
            height: height as i32,
        };

        // Add to script history map
        let mut script_history = context
            .lbtc_script_to_history_map
            .get(mrh_script)
            .cloned()
            .unwrap_or_default();
        script_history.push(history_tx.clone());
        context
            .lbtc_script_to_history_map
            .insert(mrh_script.clone(), script_history);

        // Create wallet tx
        let wallet_tx = create_mock_wallet_tx(tx_id_hex, height, amount);

        // Add to incoming tx map
        context
            .tx_map
            .incoming_tx_map
            .insert(tx_id, wallet_tx.clone());

        (context, wallet_tx)
    }

    // Create a mock wallet transaction
    fn create_mock_wallet_tx(tx_id_hex: &str, height: u32, amount: u64) -> WalletTx {
        let tx_id = Txid::from_str(tx_id_hex).unwrap();

        // Create balance for the transaction
        let mut balance = BTreeMap::new();
        balance.insert(AssetId::default(), amount as i64);

        WalletTx {
            txid: tx_id,
            tx: create_empty_transaction(),
            height: Some(height),
            fee: 1000,
            timestamp: Some(1001), // Just after swap creation time
            balance,
            outputs: Vec::new(),
            inputs: Vec::new(),
            type_: "".to_string(),
        }
    }

    fn create_empty_transaction() -> Transaction {
        Transaction {
            version: 2,
            lock_time: LockTime::from_height(0).unwrap(),
            input: vec![TxIn {
                previous_output: Default::default(),
                is_pegin: false,
                script_sig: Script::new(),
                sequence: Sequence::default(),
                asset_issuance: Default::default(),
                witness: TxInWitness::empty(),
            }],
            output: vec![],
        }
    }
}
