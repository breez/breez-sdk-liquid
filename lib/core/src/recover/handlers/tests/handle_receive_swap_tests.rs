#[cfg(test)]
mod test {
    use crate::{
        model::PaymentState,
        recover::handlers::{
            handle_receive_swap::RecoveredOnchainDataReceive, tests::test::create_history_txid,
        },
    };

    #[test]
    fn test_derive_partial_state_with_lockup_and_claim() {
        // Test with confirmed claim
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: Some(create_history_txid("1111", 100)),
            claim_tx_id: Some(create_history_txid("2222", 101)), // Confirmed claim
            mrh_tx_id: None,
            mrh_amount_sat: None,
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
            lockup_tx_id: Some(create_history_txid("1111", 100)),
            claim_tx_id: Some(create_history_txid("2222", 0)), // Unconfirmed claim
            mrh_tx_id: None,
            mrh_amount_sat: None,
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

    #[test]
    fn test_derive_partial_state_with_lockup_only() {
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: Some(create_history_txid("1111", 100)),
            claim_tx_id: None,
            mrh_tx_id: None,
            mrh_amount_sat: None,
        };

        // Not expired yet - should be Pending
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Pending)
        );

        // Expired - should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Failed)
        );
    }

    #[test]
    fn test_derive_partial_state_with_mrh_tx() {
        // Test with confirmed MRH tx
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: None,
            claim_tx_id: None,
            mrh_tx_id: Some(create_history_txid("3333", 103)),
            mrh_amount_sat: Some(100000),
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
            mrh_tx_id: Some(create_history_txid("3333", 0)), // Unconfirmed MRH tx
            mrh_amount_sat: Some(100000),
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

    #[test]
    fn test_derive_partial_state_with_no_txs() {
        let recovered_data = RecoveredOnchainDataReceive {
            lockup_tx_id: None,
            claim_tx_id: None,
            mrh_tx_id: None,
            mrh_amount_sat: None,
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
