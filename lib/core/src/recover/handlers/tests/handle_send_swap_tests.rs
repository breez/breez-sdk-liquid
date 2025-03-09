#[cfg(test)]
mod test {
    use crate::{
        model::PaymentState,
        recover::{handlers::handle_send_swap::RecoveredOnchainDataSend, model::HistoryTxId},
    };
    use lwk_wollet::{elements::Txid, hashes::Hash};

    #[test]
    fn test_derive_partial_state_with_lockup_and_claim() {
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_history_txid("1111", 100)),
            claim_tx_id: Some(create_history_txid("2222", 101)),
            refund_tx_id: None,
            preimage: None,
        };

        // When there's a lockup and claim tx, it should always be Complete, regardless of timeout
        assert_eq!(
            recovered_data.derive_partial_state(false),
            Some(PaymentState::Complete)
        );
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Complete)
        );
    }

    #[test]
    fn test_derive_partial_state_with_lockup_and_refund() {
        // Test with confirmed refund
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_history_txid("1111", 100)),
            claim_tx_id: None,
            refund_tx_id: Some(create_history_txid("3333", 102)),
            preimage: None,
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
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_history_txid("1111", 100)),
            claim_tx_id: None,
            refund_tx_id: Some(create_history_txid("3333", 0)), // Unconfirmed tx
            preimage: None,
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

    #[test]
    fn test_derive_partial_state_with_lockup_only() {
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_history_txid("1111", 100)),
            claim_tx_id: None,
            refund_tx_id: None,
            preimage: None,
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

    #[test]
    fn test_derive_partial_state_with_no_txs() {
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            preimage: None,
        };

        // Not expired yet - should return None because we can't determine the state
        assert_eq!(recovered_data.derive_partial_state(false), None);

        // Expired - should be Failed
        assert_eq!(
            recovered_data.derive_partial_state(true),
            Some(PaymentState::Failed)
        );
    }

    #[test]
    fn test_derive_partial_state_with_lockup_claim_refund() {
        // This is an edge case where both claim and refund txs exist
        let recovered_data = RecoveredOnchainDataSend {
            lockup_tx_id: Some(create_history_txid("1111", 100)),
            claim_tx_id: Some(create_history_txid("2222", 101)),
            refund_tx_id: Some(create_history_txid("3333", 102)),
            preimage: None,
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

    // Helper function to create a HistoryTxId for testing
    fn create_history_txid(hex_id: &str, height: i32) -> HistoryTxId {
        let txid_bytes = hex::decode(format!("{:0>64}", hex_id)).unwrap();
        let mut txid_array = [0u8; 32];
        txid_array.copy_from_slice(&txid_bytes);

        HistoryTxId {
            txid: Txid::from_slice(&txid_array).unwrap(),
            height,
        }
    }
}
