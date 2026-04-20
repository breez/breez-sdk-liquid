use breez_sdk_liquid::model::{
    PayAmount, PaymentDetails, PaymentMethod, PaymentState, PaymentType, PreparePayOnchainRequest,
    PrepareReceiveRequest, PrepareRefundRequest, ReceiveAmount, RefundRequest, SdkEvent,
};
use serial_test::serial;
use std::time::Duration;

use crate::regtest::{utils, ChainBackend, SdkNodeHandle, TIMEOUT};

#[cfg(feature = "browser-tests")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[sdk_macros::async_test_not_wasm]
#[serial]
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn bitcoin_electrum() {
    let handle = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    bitcoin(handle).await;
}

#[sdk_macros::async_test_all]
#[serial]
async fn bitcoin_esplora() {
    let handle = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    bitcoin(handle).await;
}

async fn bitcoin(mut handle: SdkNodeHandle) {
    let indexers = handle.indexers;

    // Derive the one-poll-cycle timeout from the SDK's configured sync period.
    // Add 1s as buffer
    let one_poll_cycle = Duration::from_secs(handle.onchain_sync_period_sec as u64 + 1);
    assert!(
        one_poll_cycle * 2 <= TIMEOUT,
        "TIMEOUT ({TIMEOUT:?}) must be at least 2x one_poll_cycle ({one_poll_cycle:?}) \
         to leave room for the try-then-mine retry pattern",
    );

    handle
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();

    // -------------- RECEIVE (non-zero-conf) --------------
    // Incoming chain swap: BTC user-lockup → L-BTC server-lockup → SDK claims L-BTC.
    // This tests the non-zero-conf path: Boltz requires a confirmed BTC
    // lockup before broadcasting the L-BTC server-lockup, and the SDK
    // requires a confirmed server-lockup before claiming.
    let onchain_limits = handle.sdk.fetch_onchain_limits().await.unwrap();
    let payer_amount_sat = 100_000;
    assert!(
        payer_amount_sat > onchain_limits.receive.max_zero_conf_sat,
        "RECEIVE test assumes non-zero-conf: payer_amount_sat ({payer_amount_sat}) \
         must exceed receive max_zero_conf_sat ({})",
        onchain_limits.receive.max_zero_conf_sat
    );

    let (prepare_response, receive_response) = handle
        .receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::BitcoinAddress,
            amount: Some(ReceiveAmount::Bitcoin { payer_amount_sat }),
        })
        .await
        .unwrap();

    let receiver_amount_sat = payer_amount_sat - prepare_response.fees_sat;

    assert!(matches!(
        prepare_response.amount.unwrap(),
        ReceiveAmount::Bitcoin { payer_amount_sat: amount_sat } if amount_sat == payer_amount_sat
    ));
    assert!(prepare_response.fees_sat > 0);

    let bip21 = receive_response.destination;
    let address = bip21.split(':').nth(1).unwrap().split('?').next().unwrap();

    // BTC user-lockup creation and confirmation
    let user_lockup_txid = utils::send_to_address_bitcoind(&address, payer_amount_sat)
        .await
        .unwrap();
    utils::wait_for_tx_in_mempool(utils::Chain::Bitcoin, &user_lockup_txid, TIMEOUT)
        .await
        .unwrap();
    utils::mine_and_index_blocks(1, utils::Chain::Bitcoin, None)
        .await
        .unwrap();

    // SDK and Boltz should detect the BTC user-lockup. Boltz should issue L-BTC server-lockup
    // -> ensure server-lockup tx is in mempool before mining its confirmation
    let pending_event = handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentPending { .. }), TIMEOUT)
        .await
        .unwrap();
    let SdkEvent::PaymentPending { details } = &pending_event else {
        panic!("Expected PaymentPending, got {pending_event:?}");
    };
    let PaymentDetails::Bitcoin { swap_id, .. } = &details.details else {
        panic!(
            "Expected PaymentDetails::Bitcoin, got {:?}",
            details.details
        );
    };

    let server_lockup_txid = utils::poll_boltz_server_lockup_txid(swap_id, TIMEOUT)
        .await
        .unwrap();
    utils::wait_for_tx_in_mempool(utils::Chain::Liquid, &server_lockup_txid, TIMEOUT)
        .await
        .unwrap();

    // Confirm the L-BTC server-lockup
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();

    // SDK should detect server-lockup confirmation and eventually issue a
    // claim tx on liquid
    //
    // The SDK's background poller is not synched with mining/indexers
    // -> might fire after server-lockup tx was mined but not all indexers have
    //    caught up
    // -> then it "consumes" the new block w/o detecting lockup tx and does
    //    not issue PaymentWaitingConfirmation.
    // -> On next poll(s) it won't update internal state if there is no new
    //    liquid block.
    // If that is detected by timeout, mine one more block and wait with
    // reduced timeout
    let waiting_event = match handle
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            one_poll_cycle,
        )
        .await
    {
        Ok(event) => event,
        Err(_) => {
            // Tip N was consumed with stale waterfalls data.  Mine N+1
            // so the poller gets a fresh tip with consistent data.
            utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
                .await
                .unwrap();
            handle
                .wait_for_event(
                    |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
                    TIMEOUT.saturating_sub(one_poll_cycle),
                )
                .await
                .unwrap()
        }
    };

    assert_eq!(
        handle.get_pending_receive_sat().await.unwrap(),
        receiver_amount_sat
    );
    assert_eq!(handle.get_balance_sat().await.unwrap(), 0);

    let SdkEvent::PaymentWaitingConfirmation { details } = &waiting_event else {
        panic!("Expected PaymentWaitingConfirmation, got {waiting_event:?}");
    };
    let PaymentDetails::Bitcoin { claim_tx_id, .. } = &details.details else {
        panic!(
            "Expected PaymentDetails::Bitcoin, got {:?}",
            details.details
        );
    };
    let claim_tx_id = claim_tx_id
        .as_ref()
        .expect("claim_tx_id should be set in PaymentWaitingConfirmation");
    utils::wait_for_tx_in_mempool(utils::Chain::Liquid, claim_tx_id, TIMEOUT)
        .await
        .unwrap();

    // Confirm the L-BTC claim tx. In theory only a Liquid block is needed —
    // PaymentSucceeded depends solely on the claim confirmation on Liquid.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();

    // TODO
    // Workaround for #XXX (maybe same as #847?): Esplora sometimes has same
    // claim TX twice in its output (once confirmed, once in mempool) which
    // leads to script history >2 which triggers the edge case in
    // determine_incoming_lockup_and_claim_txs() and triggers 120s "grace period"
    // skipping recovery -> test timeout.
    //
    // To stop skipping recovery we need more time for esplora to settle
    // + at least one new liquid block
    utils::mine_bitcoin_then_liquid(1, &indexers).await.unwrap();

    handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    //TODO
    // Workaround for #828: mine an extra Liquid block so that sync() sees a
    // new tip and takes the full scan path, refreshing the LWK wallet state.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();
    handle.sdk.sync(false).await.unwrap();

    assert_eq!(handle.get_pending_receive_sat().await.unwrap(), 0);
    assert_eq!(handle.get_balance_sat().await.unwrap(), receiver_amount_sat);

    let payments = handle.get_payments().await.unwrap();
    assert_eq!(payments.len(), 1);
    let payment = &payments[0];
    assert_eq!(payment.amount_sat, receiver_amount_sat);
    assert_eq!(payment.fees_sat, prepare_response.fees_sat);
    assert_eq!(payment.payment_type, PaymentType::Receive);
    assert_eq!(payment.status, PaymentState::Complete);
    assert!(
        matches!(&payment.details, PaymentDetails::Bitcoin { bitcoin_address, .. } if *bitcoin_address == address)
    );

    // Cleanup: flush mempools, (e.g. Boltz's deferred BTC claim and
    // any other pending transactions, so the SEND section starts
    // with clean mempools.
    utils::drain_mempools(10).await.unwrap();

    // -------------- SEND (zero-conf) --------------
    // Outgoing chain swap: L-BTC user-lockup → BTC server-lockup → SDK claims BTC.
    // This tests the zero-conf path: the send amount is within the Boltz
    // zero-conf threshold, so the SDK accepts the BTC server-lockup from
    // mempool and claims without waiting for a BTC confirmation.

    let initial_balance_sat = handle.get_balance_sat().await.unwrap();
    let address = utils::generate_address_bitcoind().await.unwrap();
    let receiver_amount_sat = 50_000;
    assert!(
        receiver_amount_sat <= onchain_limits.send.max_zero_conf_sat,
        "SEND test assumes zero-conf: receiver_amount_sat ({receiver_amount_sat}) \
         must not exceed send max_zero_conf_sat ({})",
        onchain_limits.send.max_zero_conf_sat
    );

    let (prepare_response, send_response) = handle
        .send_onchain_payment(
            &PreparePayOnchainRequest {
                amount: PayAmount::Bitcoin {
                    receiver_amount_sat,
                },
                fee_rate_sat_per_vbyte: None,
            },
            address.clone(),
        )
        .await
        .unwrap();

    let fees_sat = prepare_response.total_fees_sat;
    let sender_amount_sat = receiver_amount_sat + fees_sat;
    assert!(
        initial_balance_sat >= sender_amount_sat,
        "SEND requires sufficient balance: have {initial_balance_sat} sat, \
         need {sender_amount_sat} sat (receiver: {receiver_amount_sat} + fees: {fees_sat})"
    );

    // SDK should have broadcast L-BTC user-lockup already
    let PaymentDetails::Bitcoin {
        swap_id,
        lockup_tx_id,
        ..
    } = &send_response.payment.details
    else {
        panic!(
            "Expected PaymentDetails::Bitcoin, got {:?}",
            send_response.payment.details
        );
    };
    let user_lockup_txid = lockup_tx_id
        .as_ref()
        .expect("lockup_tx_id should be set after pay_onchain returns");

    utils::wait_for_tx_in_mempool(utils::Chain::Liquid, user_lockup_txid, TIMEOUT)
        .await
        .unwrap();

    // Get Boltz's expected lockup txid as early as possible
    let server_lockup_txid = utils::poll_boltz_server_lockup_txid(swap_id, TIMEOUT)
        .await
        .unwrap();

    assert!(
        utils::poll_boltz_zero_conf_accepted(swap_id, TIMEOUT)
            .await
            .unwrap(),
        "Boltz should accept zero-conf for SEND swap {swap_id} \
        (receiver_amount_sat = {receiver_amount_sat})"
    );

    // Confirm the L-BTC user-lockup on Liquid.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();

    // Wait for Bolt's BTC server-lockup to appear in bitcoind's mempool.
    utils::wait_for_tx_in_mempool(utils::Chain::Bitcoin, &server_lockup_txid, TIMEOUT)
        .await
        .unwrap();

    // Zero-conf: SDK claims from mempool without a BTC confirmation.
    let waiting_event = handle
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    // Ensure the BTC claim tx is in bitcoind's mempool before mining.
    let SdkEvent::PaymentWaitingConfirmation { details } = &waiting_event else {
        panic!("Expected PaymentWaitingConfirmation, got {waiting_event:?}");
    };
    let PaymentDetails::Bitcoin { claim_tx_id, .. } = &details.details else {
        panic!(
            "Expected PaymentDetails::Bitcoin, got {:?}",
            details.details
        );
    };
    let claim_tx_id = claim_tx_id
        .as_ref()
        .expect("claim_tx_id should be set in PaymentWaitingConfirmation");
    utils::wait_for_tx_in_mempool(utils::Chain::Bitcoin, claim_tx_id, TIMEOUT)
        .await
        .unwrap();

    // Confirm lock-up and claim tx (on bitcoin).
    // Additional liquid block is needed to circumvent gating introduced in #978
    // (this liquid block could also confirm Boltz's L-BTC claim tx)
    utils::mine_bitcoin_then_liquid(1, &indexers).await.unwrap();

    // TODO: figure out why on Wasm this event is occasionally skipped
    // https://github.com/breez/breez-sdk-liquid/issues/847
    let _ = handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await;
    // Workaround for #828: mine an extra Liquid block so that sync() sees a
    // new tip and takes the full scan path, refreshing the LWK wallet state.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();
    handle.sdk.sync(false).await.unwrap();

    assert_eq!(
        handle.get_balance_sat().await.unwrap(),
        initial_balance_sat - sender_amount_sat
    );

    let payments = handle.get_payments().await.unwrap();
    assert_eq!(payments.len(), 2);
    let payment = &payments[0];
    assert_eq!(payment.amount_sat, receiver_amount_sat);
    assert_eq!(payment.fees_sat, fees_sat);
    assert_eq!(payment.payment_type, PaymentType::Send);
    assert_eq!(payment.status, PaymentState::Complete);
    assert!(
        matches!(&payment.details, PaymentDetails::Bitcoin { bitcoin_address, .. } if *bitcoin_address == address)
    );

    // Cleanup: flush mempools, (e.g. Boltz's deferred L-BTC claim and
    // any other pending transactions, so the REFUND section starts
    // with clean mempools.
    utils::drain_mempools(10).await.unwrap();

    // ----------------REFUND with RBF--------------
    // Incoming chain swap with incorrect user lockup amount:
    // BTC user-lockup → BTC refund.
    // This tests the cooperative refund path: the refund is
    // confirmed by both SDK and Boltz (not timeout)
    // User fee-bump is also tested
    // TODO: add a non-cooperative test case

    let payer_amount_sat = 100_000;
    let lockup_amount_sat = payer_amount_sat - 1;

    let (_, receive_response) = handle
        .receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::BitcoinAddress,
            amount: Some(ReceiveAmount::Bitcoin { payer_amount_sat }),
        })
        .await
        .unwrap();

    let bip21 = receive_response.destination;
    let address = bip21.split(':').nth(1).unwrap().split('?').next().unwrap();

    // BTC user-lockup creation. Wrong amount will already be flagged by Boltz in mempool w/o confirmation
    let user_lockup_txid = utils::send_to_address_bitcoind(&address, lockup_amount_sat)
        .await
        .unwrap();
    utils::wait_for_tx_in_mempool(utils::Chain::Bitcoin, &user_lockup_txid, TIMEOUT)
        .await
        .unwrap();

    // SDK and Boltz should detect the wrong amount BTC user-lockup and start refund process
    handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentRefundable { .. }), TIMEOUT)
        .await
        .unwrap();

    let refundables = handle.sdk.list_refundables().await.unwrap();

    assert_eq!(refundables.len(), 1);
    let refundable = &refundables[0];
    assert_eq!(refundable.amount_sat, lockup_amount_sat);
    assert_eq!(refundable.swap_address, address);
    assert!(refundable.last_refund_tx_id.is_none());

    // Create refund tx
    let refund_address = utils::generate_address_bitcoind().await.unwrap();
    let refund_fee_rate = 1;

    let prepare_refund_response = handle
        .sdk
        .prepare_refund(&PrepareRefundRequest {
            swap_address: address.to_string(),
            refund_address: refund_address.clone(),
            fee_rate_sat_per_vbyte: refund_fee_rate,
        })
        .await
        .unwrap();
    assert!(
        prepare_refund_response.last_refund_tx_id.is_none(),
        "No refund tx should exist yet"
    );
    // With fee_rate = 1 sat/vB, fee == vsize
    assert_eq!(
        prepare_refund_response.tx_fee_sat,
        prepare_refund_response.tx_vsize as u64
    );
    assert!(
        prepare_refund_response.tx_fee_sat < lockup_amount_sat,
        "Refund fee ({}) must be less than lockup amount ({lockup_amount_sat})",
        prepare_refund_response.tx_fee_sat
    );

    let refund_response = handle
        .sdk
        .refund(&RefundRequest {
            swap_address: address.to_string(),
            refund_address: refund_address.clone(),
            fee_rate_sat_per_vbyte: refund_fee_rate,
        })
        .await
        .unwrap();

    utils::wait_for_tx_in_mempool(
        utils::Chain::Bitcoin,
        &refund_response.refund_tx_id,
        TIMEOUT,
    )
    .await
    .unwrap();

    handle
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentRefundPending { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    // User fee-bump
    let refundables = handle.sdk.list_refundables().await.unwrap();

    assert_eq!(refundables.len(), 1);
    let refundable = &refundables[0];
    assert_eq!(refundable.amount_sat, lockup_amount_sat);
    assert_eq!(refundable.swap_address, address);
    assert_eq!(
        refundable.last_refund_tx_id.as_ref().unwrap(),
        &refund_response.refund_tx_id
    );

    let refund_rbf_response = handle
        .sdk
        .refund(&RefundRequest {
            swap_address: address.to_string(),
            refund_address,
            fee_rate_sat_per_vbyte: refund_fee_rate * 2,
        })
        .await
        .unwrap();

    utils::wait_for_tx_in_mempool(
        utils::Chain::Bitcoin,
        &refund_rbf_response.refund_tx_id,
        TIMEOUT,
    )
    .await
    .unwrap();

    let refundables = handle.sdk.list_refundables().await.unwrap();

    assert_eq!(refundables.len(), 1);
    let refundable = &refundables[0];
    assert_eq!(refundable.amount_sat, lockup_amount_sat);
    assert_eq!(refundable.swap_address, address);
    assert_eq!(
        refundable.last_refund_tx_id.as_ref().unwrap(),
        &refund_rbf_response.refund_tx_id
    );

    // The following fails because the payment's refund_tx_amount_sat is None.
    // Related issue: https://github.com/breez/breez-sdk-liquid/issues/773
    // TODO: uncomment once the issue is fixed
    /*{
        let payments = handle.get_payments().await.unwrap();
        let payment = &payments[0];
        assert!(matches!(
            payment.details,
            PaymentDetails::Bitcoin {
                refund_tx_amount_sat: Some(refund_tx_amount_sat),
                ..
            } if refund_tx_amount_sat == lockup_amount_sat - prepare_refund_response.tx_fee_sat * 2
        ));
    }*/

    // Confirm the BTC refund tx (on bitcoin).
    // Additional liquid block is needed to circumvent gating introduced in #978
    utils::mine_bitcoin_then_liquid(1, &indexers).await.unwrap();

    // TODO: figure out why on Wasm this event is occasionally skipped
    // https://github.com/breez/breez-sdk-liquid/issues/847
    let _ = handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentFailed { .. }), TIMEOUT)
        .await;
    // Workaround for #828: mine an extra Liquid block so that sync() sees a
    // new tip and takes the full scan path, refreshing the LWK wallet state.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();
    handle.sdk.sync(false).await.unwrap();

    let refundables = handle.sdk.list_refundables().await.unwrap();
    assert_eq!(refundables.len(), 0);

    // Cleanup: flush mempools
    utils::drain_mempools(10).await.unwrap();

    // ----------------FINAL ASSERTIONS--------------
    // Verify the complete payment history across all three sections.

    let payments = handle.get_payments().await.unwrap();
    assert_eq!(payments.len(), 3);
    let payment = &payments[0];
    assert_eq!(payment.status, PaymentState::Failed);

    // On node.js, without disconnecting the sdk, the wasm-pack test process fails after the test succeeds
    handle.sdk.disconnect().await.unwrap();
}
