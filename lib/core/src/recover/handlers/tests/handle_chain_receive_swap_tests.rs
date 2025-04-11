#[cfg(test)]
mod test {
    use crate::{
        model::PaymentState,
        recover::handlers::{
            handle_chain_receive_swap::RecoveredOnchainDataChainReceive,
            tests::{create_btc_history_txid, create_lbtc_history_txid},
        },
    };
    use boltz_client::boltz::PairLimits;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_btc_lockup_and_lbtc_claim() {
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_claim_tx_id: Some(create_lbtc_history_txid("2222", 101)),
            lbtc_claim_address: Some("lq1qqvynd50t4tajashdguell7nu9gycuqqd869w8vqww9ys9dsz7szdfeu7pwe4yzzme28qsluyfyrtqmq9scl5ydw4lesx3c5qu".to_string()),
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 0,
            btc_user_lockup_amount_sat: 100000,
            btc_refund_tx_id: None,
        };

        // When there's a lockup and confirmed claim tx, it should be Complete
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, false),
            Some(PaymentState::Complete)
        );
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, true, false),
            Some(PaymentState::Complete)
        );

        // Test with unconfirmed claim
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_claim_tx_id: Some(create_lbtc_history_txid("2222", 0)), // Unconfirmed claim
            lbtc_claim_address: Some("lq1qqvynd50t4tajashdguell7nu9gycuqqd869w8vqww9ys9dsz7szdfeu7pwe4yzzme28qsluyfyrtqmq9scl5ydw4lesx3c5qu".to_string()),
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 0,
            btc_user_lockup_amount_sat: 100000,
            btc_refund_tx_id: None,
        };

        // When there's a lockup and unconfirmed claim tx, it should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, false),
            Some(PaymentState::Pending)
        );
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, true, false),
            Some(PaymentState::Pending)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_btc_lockup_and_btc_refund() {
        // Test with confirmed refund
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_claim_tx_id: None,
            lbtc_claim_address: None,
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 0,
            btc_user_lockup_amount_sat: 100000,
            btc_refund_tx_id: Some(create_btc_history_txid("4444", 103)), // Confirmed refund
        };

        // When there's a lockup and confirmed refund tx, it should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, false),
            Some(PaymentState::Failed)
        );
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, true, false),
            Some(PaymentState::Failed)
        );

        // Test with unconfirmed refund
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_claim_tx_id: None,
            lbtc_claim_address: None,
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 0,
            btc_user_lockup_amount_sat: 100000,
            btc_refund_tx_id: Some(create_btc_history_txid("4444", 0)), // Unconfirmed refund
        };

        // When there's a lockup and unconfirmed refund tx, it should be RefundPending
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, false),
            Some(PaymentState::RefundPending)
        );
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, true, false),
            Some(PaymentState::RefundPending)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_btc_lockup_only() {
        // Test with correct lockup amount
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: None,
            lbtc_claim_tx_id: None,
            lbtc_claim_address: None,
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 0,
            btc_user_lockup_amount_sat: 100000,
            btc_refund_tx_id: None,
        };

        // Not expired yet - should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, false),
            Some(PaymentState::Pending)
        );
        // Not expired, waiting for fee acceptance
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, true),
            Some(PaymentState::WaitingFeeAcceptance)
        );
        // Expired without balance - should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, true, false),
            Some(PaymentState::Pending)
        );

        // Test with funds still in the address
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: None,
            lbtc_claim_tx_id: None,
            lbtc_claim_address: None,
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 100000, // Funds still in address
            btc_user_lockup_amount_sat: 100000,
            btc_refund_tx_id: None,
        };

        // Expired with funds still in address - should be Refundable
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, true, false),
            Some(PaymentState::Refundable)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_incorrect_lockup_amount() {
        let limits = PairLimits {
            minimal: 10000,
            maximal: 2000000,
            maximal_zero_conf: 0,
        };

        // Test with amount below minimum
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: None,
            lbtc_claim_tx_id: None,
            lbtc_claim_address: None,
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 5000,
            btc_user_lockup_amount_sat: 5000, // Below minimum
            btc_refund_tx_id: None,
        };

        // Should be Refundable due to amount below minimum
        assert_eq!(
            recovered_data.derive_partial_state(None, Some(limits.clone()), false, false),
            Some(PaymentState::Refundable)
        );

        // Test with amount above maximum
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: None,
            lbtc_claim_tx_id: None,
            lbtc_claim_address: None,
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 3000000,
            btc_user_lockup_amount_sat: 3000000, // Above maximum
            btc_refund_tx_id: None,
        };

        // Should be Refundable due to amount above maximum
        assert_eq!(
            recovered_data.derive_partial_state(None, Some(limits), false, false),
            Some(PaymentState::Refundable)
        );

        // Test with unexpected amount
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: None,
            lbtc_claim_tx_id: None,
            lbtc_claim_address: None,
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 150000,
            btc_user_lockup_amount_sat: 150000, // Different from expected
            btc_refund_tx_id: None,
        };

        // Should be Refundable due to unexpected amount
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, false),
            Some(PaymentState::Refundable)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_no_txs() {
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: None,
            lbtc_claim_tx_id: None,
            lbtc_claim_address: None,
            btc_user_lockup_tx_id: None,
            btc_user_lockup_address_balance_sat: 0,
            btc_user_lockup_amount_sat: 0,
            btc_refund_tx_id: None,
        };

        // Not expired yet - should return None because we can't determine the state
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, false),
            None
        );

        // Expired - should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, true, false),
            Some(PaymentState::Failed)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_claim_refund() {
        // This is an edge case where both claim and refund txs exist
        let recovered_data = RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_claim_tx_id: Some(create_lbtc_history_txid("2222", 101)),
            lbtc_claim_address: Some("lq1qqvynd50t4tajashdguell7nu9gycuqqd869w8vqww9ys9dsz7szdfeu7pwe4yzzme28qsluyfyrtqmq9scl5ydw4lesx3c5qu".to_string()),
            btc_user_lockup_tx_id: Some(create_btc_history_txid("3333", 102)),
            btc_user_lockup_address_balance_sat: 0,
            btc_user_lockup_amount_sat: 100000,
            btc_refund_tx_id: Some(create_btc_history_txid("4444", 103)),
        };

        // Complete state should take precedence over refund
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, false, false),
            Some(PaymentState::Complete)
        );
        assert_eq!(
            recovered_data.derive_partial_state(Some(100000), None, true, false),
            Some(PaymentState::Complete)
        );
    }
}
