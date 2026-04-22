use std::time::Duration;

use breez_sdk_liquid::model::{
    PayAmount, PaymentDetails, PaymentMethod, PaymentState, PaymentType, PreparePayOnchainRequest,
    PrepareReceiveRequest, PrepareRefundRequest, ReceiveAmount, RefundRequest, SdkEvent,
};
use serial_test::serial;
use tokio_with_wasm::alias as tokio;

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

// ---------------------------------------------------------------------------
// Helper: mine n Bitcoin blocks, then n Liquid blocks.
//
// PR #978 changed the SDK so that the Bitcoin tip is only
// fetched when `is_new_liquid_block` is true.  If we mine the Liquid block
// *after* the Bitcoin block the SDK is guaranteed to see the updated Bitcoin
// tip the moment its sync loop wakes on the new Liquid tip — no race, no
// sleeps, no retry loops needed.  The original `utils::mine_blocks` mines
// both but does not guarantee ordering relative to the SDK event loop.
// ---------------------------------------------------------------------------
async fn mine_chain_blocks(n: u64) {
    utils::mine_bitcoin_blocks(n).await.unwrap();
    utils::mine_liquid_blocks(n).await.unwrap();
}

async fn bitcoin(mut handle: SdkNodeHandle) {
    handle
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();

    // --------------RECEIVE--------------
    let payer_amount_sat = 100_000;

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

    // Confirm user lockup — mine Bitcoin first, then Liquid so the SDK's
    // sync loop sees the Bitcoin tip update on its next Liquid-block wake-up.
    mine_chain_blocks(1).await;

    // Wait for swapper to lock up funds
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Confirm swapper lockup
    mine_chain_blocks(1).await;

    handle
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

    // Confirm claim tx
    mine_chain_blocks(1).await;

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

    // --------------SEND--------------

    let initial_balance_sat = handle.get_balance_sat().await.unwrap();
    let address = utils::generate_address_bitcoind().await.unwrap();
    let receiver_amount_sat = 50_000;

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

    // Confirm user lockup
    mine_chain_blocks(1).await;

    // Wait for swapper to lock up funds
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Confirm swapper lockup
    mine_chain_blocks(1).await;

    // Wait for sdk to broadcast claim tx
    handle
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    // Confirm claim tx
    mine_chain_blocks(1).await;

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

    mine_chain_blocks(1).await;

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

    // On node.js, without disconnecting the sdk, the wasm-pack test process fails after the test succeeds
    handle.sdk.disconnect().await.unwrap();
}

// Additional tests for chain swap state transitions (Issue #996)
// Verifies that a chain swap that receives the correct amount completes
// successfully when the user lockup is confirmed before the swapper acts.
#[sdk_macros::async_test_not_wasm]
#[serial]
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn bitcoin_receive_exact_amount_electrum() {
    let handle = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    bitcoin_receive_exact_amount(handle).await;
}

#[sdk_macros::async_test_all]
#[serial]
async fn bitcoin_receive_exact_amount_esplora() {
    let handle = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    bitcoin_receive_exact_amount(handle).await;
}

async fn bitcoin_receive_exact_amount(mut handle: SdkNodeHandle) {
    handle
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();

    let payer_amount_sat = 50_000;

    let (prepare_response, receive_response) = handle
        .receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::BitcoinAddress,
            amount: Some(ReceiveAmount::Bitcoin { payer_amount_sat }),
        })
        .await
        .unwrap();

    let receiver_amount_sat = payer_amount_sat - prepare_response.fees_sat;

    let bip21 = receive_response.destination;
    let address = bip21.split(':').nth(1).unwrap().split('?').next().unwrap();

    // Send exact requested amount
    utils::send_to_address_bitcoind(&address, payer_amount_sat)
        .await
        .unwrap();

    // Confirm user lockup (Bitcoin first, then Liquid to trigger SDK fetch)
    mine_chain_blocks(1).await;

    tokio::time::sleep(Duration::from_secs(5)).await;

    mine_chain_blocks(1).await;

    handle
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    mine_chain_blocks(1).await;

    handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    assert_eq!(handle.get_balance_sat().await.unwrap(), receiver_amount_sat);
    assert_eq!(handle.get_pending_receive_sat().await.unwrap(), 0);

    handle.sdk.disconnect().await.unwrap();
}

// Verifies that a chain swap where the user sends *more* than requested
// is treated as refundable (overpayment edge case).
#[sdk_macros::async_test_not_wasm]
#[serial]
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn bitcoin_receive_overpayment_refundable_electrum() {
    let handle = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    bitcoin_receive_overpayment_refundable(handle).await;
}

#[sdk_macros::async_test_all]
#[serial]
async fn bitcoin_receive_overpayment_refundable_esplora() {
    let handle = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    bitcoin_receive_overpayment_refundable(handle).await;
}

async fn bitcoin_receive_overpayment_refundable(mut handle: SdkNodeHandle) {
    handle
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();

    let payer_amount_sat = 100_000;
    // Send 1 sat less than requested — triggers the refundable path
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

    // The swap becomes refundable without needing block confirmation
    handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentRefundable { .. }), TIMEOUT)
        .await
        .unwrap();

    let refundables = handle.sdk.list_refundables().await.unwrap();
    assert_eq!(refundables.len(), 1);
    assert_eq!(refundables[0].amount_sat, lockup_amount_sat);
    assert_eq!(refundables[0].swap_address, address);
    assert!(refundables[0].last_refund_tx_id.is_none());

    // Initiate refund
    let refund_address = utils::generate_address_bitcoind().await.unwrap();
    handle
        .sdk
        .refund(&RefundRequest {
            swap_address: address.to_string(),
            refund_address,
            fee_rate_sat_per_vbyte: 1,
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

    // Mine to confirm the refund tx
    mine_chain_blocks(1).await;

    let _ = handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentFailed { .. }), TIMEOUT)
        .await;
    handle.sdk.sync(false).await.unwrap();

    // Swap should no longer be listed as refundable once confirmed
    let refundables = handle.sdk.list_refundables().await.unwrap();
    assert_eq!(refundables.len(), 0);

    handle.sdk.disconnect().await.unwrap();
}
