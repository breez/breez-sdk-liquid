use breez_sdk_liquid::model::{
    PaymentDetails, PaymentState, PaymentType, PrepareReceiveRequest, PrepareSendRequest, SdkEvent,
};
use serial_test::serial;

use crate::regtest::{
    utils::{self},
    ChainBackend, SdkNodeHandle, TIMEOUT,
};

#[cfg(feature = "browser-tests")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[sdk_macros::async_test_not_wasm]
#[serial]
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn bolt11_electrum() {
    let handle_alice = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    let handle_bob = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    bolt11(handle_alice, handle_bob).await;
}

#[sdk_macros::async_test_all]
#[serial]
async fn bolt11_esplora() {
    let handle_alice = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    let handle_bob = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    bolt11(handle_alice, handle_bob).await;
}

#[sdk_macros::async_test_not_wasm]
#[serial]
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn bolt11_mixed() {
    let handle_alice = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    let handle_bob = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    bolt11(handle_alice, handle_bob).await;
}

async fn bolt11(mut handle_alice: SdkNodeHandle, mut handle_bob: SdkNodeHandle) {
    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();
    handle_bob
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();

    let indexers = utils::Indexers::from_handles(&[&handle_alice, &handle_bob]);

    // -------------------RECEIVE SWAP-------------------
    let payer_amount_sat = 200_000;

    let (prepare_response, receive_response) = handle_alice
        .receive_payment(&PrepareReceiveRequest {
            payment_method: breez_sdk_liquid::model::PaymentMethod::Bolt11Invoice,
            amount: Some(breez_sdk_liquid::model::ReceiveAmount::Bitcoin { payer_amount_sat }),
        })
        .await
        .unwrap();
    let invoice = receive_response.destination;
    let receiver_amount_sat = payer_amount_sat - prepare_response.fees_sat;

    let _ = utils::pay_invoice_lnd(&invoice).await;

    handle_alice
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    handle_alice
        .assert_wallet_pending(receiver_amount_sat, 0, 0)
        .await;

    // Confirm the server lockup and wait for swap to complete
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();

    utils::wait_for_event_with_retry(
        &mut handle_alice,
        &indexers,
        |e| matches!(e, SdkEvent::PaymentSucceeded { .. }),
        TIMEOUT,
    )
    .await
    .unwrap();

    // Workaround for #828: mine an extra Liquid block so that sync() sees a
    // new tip and takes the full scan path, refreshing the LWK wallet state.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();
    handle_alice.sdk.sync(false).await.unwrap();

    handle_alice
        .assert_wallet_settled(receiver_amount_sat)
        .await;

    let payments = handle_alice.get_payments().await.unwrap();
    assert_eq!(payments.len(), 1);
    let payment = &payments[0];
    utils::assert_payment(
        payment,
        receiver_amount_sat,
        prepare_response.fees_sat,
        PaymentType::Receive,
        PaymentState::Complete,
    );
    assert!(matches!(payment.details, PaymentDetails::Lightning { .. }));

    // -------------------SEND SWAP-------------------
    let receiver_amount_sat = 100_000;
    let initial_balance = handle_alice.get_balance_sat().await.unwrap();

    let invoice = utils::generate_invoice_lnd(receiver_amount_sat)
        .await
        .unwrap();

    let (prepare_response, _) = handle_alice
        .send_payment(&PrepareSendRequest {
            destination: invoice,
            amount: None,
            disable_mrh: None,
            payment_timeout_sec: None,
        })
        .await
        .unwrap();
    let payer_amount_sat = receiver_amount_sat + prepare_response.fees_sat.unwrap();

    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentPending { .. }), TIMEOUT)
        .await
        .unwrap();

    handle_alice
        .assert_wallet_pending(0, payer_amount_sat, initial_balance - payer_amount_sat)
        .await;

    // Confirm the server lockup and wait for swap to complete
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();

    utils::wait_for_event_with_retry(
        &mut handle_alice,
        &indexers,
        |e| matches!(e, SdkEvent::PaymentSucceeded { .. }),
        TIMEOUT,
    )
    .await
    .unwrap();

    // Workaround for #828: mine an extra Liquid block so that sync() sees a
    // new tip and takes the full scan path, refreshing the LWK wallet state.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();
    handle_alice.sdk.sync(false).await.unwrap();

    handle_alice
        .assert_wallet_settled(initial_balance - payer_amount_sat)
        .await;

    let payments = handle_alice.get_payments().await.unwrap();
    assert_eq!(payments.len(), 2);
    let payment = &payments[0];
    utils::assert_payment(
        payment,
        receiver_amount_sat,
        prepare_response.fees_sat.unwrap(),
        PaymentType::Send,
        PaymentState::Complete,
    );
    assert!(matches!(payment.details, PaymentDetails::Lightning { .. }));

    // -------------------MRH-------------------
    let receiver_amount_sat = 50_000;
    let alice_balance_before_mrh = handle_alice.get_balance_sat().await.unwrap();

    let (_, receive_response) = handle_bob
        .receive_payment(&PrepareReceiveRequest {
            payment_method: breez_sdk_liquid::model::PaymentMethod::Bolt11Invoice,
            amount: Some(breez_sdk_liquid::model::ReceiveAmount::Bitcoin {
                payer_amount_sat: receiver_amount_sat,
            }),
        })
        .await
        .unwrap();
    let invoice = receive_response.destination;

    let (prepare_response_send, _) = handle_alice
        .send_payment(&PrepareSendRequest {
            destination: invoice,
            amount: None,
            disable_mrh: None,
            payment_timeout_sec: None,
        })
        .await
        .unwrap();

    handle_bob
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    // Confirm the liquid tx
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();

    let _ = utils::wait_for_event_with_retry(
        &mut handle_alice,
        &indexers,
        |e| matches!(e, SdkEvent::PaymentSucceeded { .. }),
        TIMEOUT,
    )
    .await;
    utils::wait_for_event_with_retry(
        &mut handle_bob,
        &indexers,
        |e| matches!(e, SdkEvent::PaymentSucceeded { .. }),
        TIMEOUT,
    )
    .await
    .unwrap();

    // Workaround for #828: mine an extra Liquid block so that sync() sees a
    // new tip and takes the full scan path, refreshing the LWK wallet state.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();
    handle_alice.sdk.sync(false).await.unwrap();
    handle_bob.sdk.sync(false).await.unwrap();

    handle_alice
        .assert_wallet_settled(
            alice_balance_before_mrh
                - receiver_amount_sat
                - prepare_response_send.fees_sat.unwrap(),
        )
        .await;
    handle_bob.assert_wallet_settled(receiver_amount_sat).await;

    let alice_payments = handle_alice.get_payments().await.unwrap();
    assert_eq!(alice_payments.len(), 3);
    let alice_payment = &alice_payments[0];
    utils::assert_payment(
        alice_payment,
        receiver_amount_sat,
        prepare_response_send.fees_sat.unwrap(),
        PaymentType::Send,
        PaymentState::Complete,
    );
    assert!(matches!(
        alice_payment.details,
        PaymentDetails::Liquid { .. }
    ));

    let bob_payments = handle_bob.get_payments().await.unwrap();
    assert_eq!(bob_payments.len(), 1);
    let bob_payment = &bob_payments[0];
    utils::assert_payment(
        bob_payment,
        receiver_amount_sat,
        0,
        PaymentType::Receive,
        PaymentState::Complete,
    );
    // TODO: figure out why occasionally this fails (details = Liquid)
    // https://github.com/breez/breez-sdk-liquid/issues/829
    /*assert!(matches!(
        bob_payment.details,
        PaymentDetails::Lightning { .. }
    ));*/

    // On node.js, without disconnecting the sdk, the wasm-pack test process fails after the test succeeds
    handle_alice.sdk.disconnect().await.unwrap();
    handle_bob.sdk.disconnect().await.unwrap();
}
