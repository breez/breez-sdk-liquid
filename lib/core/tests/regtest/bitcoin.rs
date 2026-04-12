use breez_sdk_liquid::model::{
    PayAmount, PaymentDetails, PaymentMethod, PaymentState, PaymentType, PreparePayOnchainRequest,
    PrepareReceiveRequest, PrepareRefundRequest, ReceiveAmount, RefundRequest, SdkEvent,
};
use serial_test::serial;

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

    utils::send_to_address_bitcoind(&address, payer_amount_sat)
        .await
        .unwrap();

    // Confirm the BTC user-lockup first.  Boltz requires at least one
    // confirmation before broadcasting the L-BTC server-lockup in regtest
    // (BTC maxZeroConfAmount = 0).
    utils::mine_and_index_blocks(1, utils::Chain::Bitcoin, true)
        .await
        .unwrap();

    // Ensure the Boltz server-lockup is in the Liquid mempool before
    // mining, so that it gets included and confirmed in this block.
    utils::wait_for_liquid_mempool_tx(TIMEOUT).await.unwrap();

    utils::mine_and_index_blocks(1, utils::Chain::Liquid, true)
        .await
        .unwrap();

    // Wait for the SDK to detect the server lockup and broadcast the
    // claim tx (emitted as PaymentWaitingConfirmation).
    let waiting_event = handle
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    assert_eq!(
        handle.get_pending_receive_sat().await.unwrap(),
        receiver_amount_sat
    );
    assert_eq!(handle.get_balance_sat().await.unwrap(), 0);

    // Extract the claim_tx_id from the event and wait for it to reach
    // elementsd's mempool before mining the confirming block.
    let SdkEvent::PaymentWaitingConfirmation { details } = &waiting_event else {
        panic!("Expected PaymentWaitingConfirmation, got {waiting_event:?}");
    };
    let PaymentDetails::Bitcoin { claim_tx_id, .. } = &details.details else {
        panic!("Expected PaymentDetails::Bitcoin, got {:?}", details.details);
    };
    let claim_tx_id = claim_tx_id.as_ref().expect("claim_tx_id should be set in PaymentWaitingConfirmation");
    utils::wait_for_tx_in_liquid_mempool(claim_tx_id, TIMEOUT)
        .await
        .unwrap();

    // Confirm claim tx.
    utils::mine_bitcoin_then_liquid(1).await.unwrap();

    handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

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

    // -------------- SEND (zero-conf) --------------
    // Outgoing chain swap: L-BTC user-lockup → BTC server-lockup → SDK claims BTC.
    // This tests the zero-conf path: the send amount is within the Boltz
    // zero-conf threshold, so the SDK accepts the BTC server-lockup from
    // mempool and claims without waiting for a BTC confirmation.
    // TODO: add a non-zero-conf SEND test with amount > send.max_zero_conf_sat

    let initial_balance_sat = handle.get_balance_sat().await.unwrap();
    let address = utils::generate_address_bitcoind().await.unwrap();
    let receiver_amount_sat = 50_000;
    assert!(
        receiver_amount_sat <= onchain_limits.send.max_zero_conf_sat,
        "SEND test assumes zero-conf: receiver_amount_sat ({receiver_amount_sat}) \
         must not exceed send max_zero_conf_sat ({})",
        onchain_limits.send.max_zero_conf_sat
    );

    let (prepare_response, _) = handle
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

    // Ensure the L-BTC user-lockup is in the Liquid mempool before mining.
    utils::wait_for_liquid_mempool_tx(TIMEOUT).await.unwrap();

    // Confirm the L-BTC user-lockup on Liquid.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, true)
        .await
        .unwrap();

    // Wait for Boltz to broadcast the BTC server-lockup.
    utils::wait_for_bitcoin_mempool_tx(TIMEOUT).await.unwrap();

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
        panic!("Expected PaymentDetails::Bitcoin, got {:?}", details.details);
    };
    let claim_tx_id = claim_tx_id.as_ref().expect("claim_tx_id should be set in PaymentWaitingConfirmation");
    utils::wait_for_tx_in_bitcoin_mempool(claim_tx_id, TIMEOUT)
        .await
        .unwrap();

    // Confirm claim tx.
    utils::mine_bitcoin_then_liquid(1).await.unwrap();

    // TODO: figure out why on Wasm this event is occasionally skipped
    // https://github.com/breez/breez-sdk-liquid/issues/847
    let _ = handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await;
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

    // ----------------REFUND--------------

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

    utils::send_to_address_bitcoind(&address, lockup_amount_sat)
        .await
        .unwrap();

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

    let refund_address = utils::generate_address_bitcoind().await.unwrap();
    let refund_fee_rate = 1;
    let refund_rbf_fee_rate = 2;

    let refund_response = handle
        .sdk
        .refund(&RefundRequest {
            swap_address: address.to_string(),
            refund_address: refund_address.clone(),
            fee_rate_sat_per_vbyte: refund_fee_rate,
        })
        .await
        .unwrap();

    handle
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentRefundPending { .. }),
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
        &refund_response.refund_tx_id
    );

    let _prepare_refund_rbf_response = handle
        .sdk
        .prepare_refund(&PrepareRefundRequest {
            swap_address: address.to_string(),
            refund_address: refund_address.clone(),
            fee_rate_sat_per_vbyte: refund_rbf_fee_rate,
        })
        .await
        .unwrap();

    let refund_rbf_response = handle
        .sdk
        .refund(&RefundRequest {
            swap_address: address.to_string(),
            refund_address,
            fee_rate_sat_per_vbyte: refund_rbf_fee_rate,
        })
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

    utils::mine_bitcoin_then_liquid(1).await.unwrap();

    // TODO: figure out why on Wasm this event is occasionally skipped
    // https://github.com/breez/breez-sdk-liquid/issues/847
    let _ = handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentFailed { .. }), TIMEOUT)
        .await;
    handle.sdk.sync(false).await.unwrap();

    let refundables = handle.sdk.list_refundables().await.unwrap();
    assert_eq!(refundables.len(), 0);

    let payments = handle.get_payments().await.unwrap();
    assert_eq!(payments.len(), 3);
    let payment = &payments[0];
    assert_eq!(payment.status, PaymentState::Failed);
    // The following fails because the payment's refund_tx_amount_sat is None. Related issue: https://github.com/breez/breez-sdk-liquid/issues/773
    // TODO: uncomment once the issue is fixed
    /*assert!(matches!(
        payment.details,
        PaymentDetails::Bitcoin {
            refund_tx_amount_sat: Some(refund_tx_amount_sat),
            ..
        } if refund_tx_amount_sat == lockup_amount_sat - prepare_refund_rbf_response.tx_fee_sat
    ));*/

    // On node.js, without disconnecting the sdk, the wasm-pack test process fails after the test succeeds
    handle.sdk.disconnect().await.unwrap();
}
