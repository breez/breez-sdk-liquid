#[cfg(test)]
mod test {
    use crate::{
        model::PaymentState,
        recover::handlers::{
            handle_chain_send_swap::RecoveredOnchainDataChainSend,
            tests::{create_btc_history_txid, create_lbtc_history_txid},
        },
    };

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lbtc_lockup_and_btc_claim() {
        let recovered_data = RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_refund_tx_id: None,
            btc_server_lockup_tx_id: Some(create_btc_history_txid("2222", 101)),
            btc_claim_tx_id: Some(create_btc_history_txid("3333", 102)),
        };

        // When there's a lockup and confirmed claim tx, it should be Complete
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Complete)
        );
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Complete)
        );

        // Test with unconfirmed claim
        let recovered_data = RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_refund_tx_id: None,
            btc_server_lockup_tx_id: Some(create_btc_history_txid("2222", 101)),
            btc_claim_tx_id: Some(create_btc_history_txid("3333", 0)), // Unconfirmed claim
        };

        // When there's a lockup and unconfirmed claim tx, it should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Pending)
        );
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Pending)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_and_refund() {
        // Test with confirmed refund
        let recovered_data = RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_refund_tx_id: Some(create_lbtc_history_txid("4444", 102)),
            btc_server_lockup_tx_id: Some(create_btc_history_txid("2222", 101)),
            btc_claim_tx_id: None,
        };

        // When there's a lockup and confirmed refund tx, it should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Failed)
        );
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Failed)
        );

        // Test with unconfirmed refund
        let recovered_data = RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_refund_tx_id: Some(create_lbtc_history_txid("4444", 0)), // Unconfirmed refund
            btc_server_lockup_tx_id: Some(create_btc_history_txid("2222", 101)),
            btc_claim_tx_id: None,
        };

        // When there's a lockup and unconfirmed refund tx, it should be RefundPending
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::RefundPending)
        );
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::RefundPending)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_only() {
        let recovered_data = RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_refund_tx_id: None,
            btc_server_lockup_tx_id: None,
            btc_claim_tx_id: None,
        };

        // Not expired yet - should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Pending)
        );

        // Expired - should be RefundPending
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::RefundPending)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_no_txs() {
        let recovered_data = RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_id: None,
            lbtc_refund_tx_id: None,
            btc_server_lockup_tx_id: None,
            btc_claim_tx_id: None,
        };

        // Not expired yet - should return None because we can't determine the state
        assert_eq!(recovered_data.derive_partial_state(false), None);

        // Expired - should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Failed)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_claim_refund() {
        // This is an edge case where both claim and refund txs exist
        let recovered_data = RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            lbtc_refund_tx_id: Some(create_lbtc_history_txid("4444", 102)),
            btc_server_lockup_tx_id: Some(create_btc_history_txid("2222", 101)),
            btc_claim_tx_id: Some(create_btc_history_txid("3333", 103)),
        };

        // Complete state should take precedence over refund
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Complete)
        );
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Complete)
        );
    }
}
