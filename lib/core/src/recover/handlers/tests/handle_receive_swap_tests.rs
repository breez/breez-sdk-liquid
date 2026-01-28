#[cfg(test)]
mod test {
    use crate::{
        model::PaymentState,
        recover::handlers::{
            handle_receive_swap::RecoveredOnchainDataReceive, tests::create_lbtc_history_txid,
        },
    };

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_and_claim() {
        // Test with confirmed claim
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: Some(create_lbtc_history_txid("2222", 101)), // Confirmed claim
            mrh_tx_id: None,
            mrh_amount_sat: None,
            lbtc_claim_script_history_len: 2,
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
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: Some(create_lbtc_history_txid("2222", 0)), // Unconfirmed claim
            mrh_tx_id: None,
            mrh_amount_sat: None,
            lbtc_claim_script_history_len: 2,
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
    fn test_derive_partial_state_with_lockup_only_server_refunded() {
        // Server has refunded (history > 1)
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: None,
            mrh_tx_id: None,
            mrh_amount_sat: None,
            lbtc_claim_script_history_len: 2, // Server lockup + server refund
        };

        // Not expired yet - should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Pending)
        );

        // Expired and server refunded - should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Failed)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_lockup_only_still_claimable() {
        // Server has NOT refunded (history = 1), LBTC still claimable
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: Some(create_lbtc_history_txid("1111", 100)),
            claim_tx_id: None,
            mrh_tx_id: None,
            mrh_amount_sat: None,
            lbtc_claim_script_history_len: 1, // Only server lockup
        };

        // Not expired yet - should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Pending)
        );

        // Expired but server hasn't refunded - should be Pending (still claimable)
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Pending)
        );
    }

    #[sdk_macros::test_all]
    fn test_derive_partial_state_with_mrh_tx() {
        // Test with confirmed MRH tx
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: None,
            claim_tx_id: None,
            mrh_tx_id: Some(create_lbtc_history_txid("3333", 103)),
            mrh_amount_sat: Some(100000),
            lbtc_claim_script_history_len: 0,
        };

        // When there's a confirmed MRH tx, it should be Complete
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Complete)
        );
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Complete)
        );

        // Test with unconfirmed MRH tx
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: None,
            claim_tx_id: None,
            mrh_tx_id: Some(create_lbtc_history_txid("3333", 0)), // Unconfirmed MRH tx
            mrh_amount_sat: Some(100000),
            lbtc_claim_script_history_len: 0,
        };

        // When there's an unconfirmed MRH tx, it should be Pending
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
    fn test_derive_partial_state_with_no_txs() {
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: None,
            claim_tx_id: None,
            mrh_tx_id: None,
            mrh_amount_sat: None,
            lbtc_claim_script_history_len: 0,
        };

        // Not expired yet - should return None because we can't determine the state
        assert_eq!(recovered_data.derive_partial_state(false), None);

        // Expired - should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Failed)
        );
    }
}
