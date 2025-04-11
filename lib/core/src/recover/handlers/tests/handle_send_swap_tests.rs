#[cfg(test)]
mod test {
    use crate::{
        model::PaymentState,
        recover::handlers::{
            handle_send_swap::RecoveredOnchainDataSend, tests::create_lbtc_history_txid,
        },
    };

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_and_claim() {
        let swap_preimage = Some("cccc".to_string());
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: Some(create_lbtc_history_txid("2222", 101)),
            refund_tx_id: None,
            preimage: None,
        };

        // When there's a lockup and claim tx, it should always be Complete, regardless of timeout
        assert_eq!(
            recovered_data.derive_partial_state(swap_preimage.clone(), false),
            Some(PaymentState::Complete)
        );
        assert_eq!(
            recovered_data.derive_partial_state(swap_preimage, true),
            Some(PaymentState::Complete)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_and_refund() {
        // Test with confirmed refund
        let no_swap_preimage = None;
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: None,
            refund_tx_id: Some(create_lbtc_history_txid("3333", 102)),
            preimage: None,
        };

        // When there's a lockup and confirmed refund tx, it should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(no_swap_preimage.clone(), false),
            Some(PaymentState::Failed)
        );
        assert_eq!(
            recovered_data.derive_partial_state(no_swap_preimage.clone(), true),
            Some(PaymentState::Failed)
        );

        // Test with unconfirmed refund
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: None,
            refund_tx_id: Some(create_lbtc_history_txid("3333", 0)), // Unconfirmed tx
            preimage: None,
        };

        // When there's a lockup and unconfirmed refund tx, it should be RefundPending
        assert_eq!(
            recovered_data.derive_partial_state(no_swap_preimage.clone(), false),
            Some(PaymentState::RefundPending)
        );
        assert_eq!(
            recovered_data.derive_partial_state(no_swap_preimage, true),
            Some(PaymentState::RefundPending)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_only() {
        let no_swap_preimage = None;
        let swap_preimage = Some("cccc".to_string());
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: None,
            refund_tx_id: None,
            preimage: None,
        };

        // Not expired yet - should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(no_swap_preimage.clone(), false),
            Some(PaymentState::Pending)
        );
        // Not expired yet - should be Complete
        assert_eq!(
            recovered_data.derive_partial_state(swap_preimage.clone(), false),
            Some(PaymentState::Complete)
        );

        // Expired - should be RefundPending
        assert_eq!(
            recovered_data.derive_partial_state(no_swap_preimage, true),
            Some(PaymentState::RefundPending)
        );
        // Expired - should be Complete
        assert_eq!(
            recovered_data.derive_partial_state(swap_preimage, true),
            Some(PaymentState::Complete)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_no_txs() {
        let no_swap_preimage = None;
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            preimage: None,
        };

        // Not expired yet - should return None because we can't determine the state
        assert_eq!(
            recovered_data.derive_partial_state(no_swap_preimage.clone(), false),
            None
        );

        // Expired - should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(no_swap_preimage, true),
            Some(PaymentState::Failed)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_claim_refund() {
        // This is an edge case where both claim and refund txs exist
        let swap_preimage = Some("cccc".to_string());
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: Some(create_lbtc_history_txid("2222", 101)),
            refund_tx_id: Some(create_lbtc_history_txid("3333", 102)),
            preimage: None,
        };

        // Complete state should take precedence over refund
        assert_eq!(
            recovered_data.derive_partial_state(swap_preimage.clone(), false),
            Some(PaymentState::Complete)
        );
        assert_eq!(
            recovered_data.derive_partial_state(swap_preimage, true),
            Some(PaymentState::Complete)
        );
    }
}
