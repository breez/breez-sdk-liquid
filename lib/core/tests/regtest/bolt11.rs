use std::time::Duration;

use breez_sdk_liquid::model::{
    PaymentDetails, PaymentState, PaymentType, PrepareReceiveRequest, PrepareSendRequest, SdkEvent,
};
use serial_test::serial;
use tokio_with_wasm::alias as tokio;

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

async fn bolt11(mut handle_alice: SdkNodeHandle, mut handle_bob: SdkNodeHandle) {
    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();
    handle_bob
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();

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

    assert_eq!(
        handle_alice.get_pending_receive_sat().await.unwrap(),
        receiver_amount_sat
    );

    utils::mine_blocks(1).await.unwrap();

    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    // TODO: this shouldn't be needed, but without it, sometimes get_balance_sat isn't updated in time
    // https://github.com/breez/breez-sdk-liquid/issues/828
    tokio::time::sleep(Duration::from_secs(1)).await;
    handle_alice.sdk.sync(false).await.unwrap();

    assert_eq!(handle_alice.get_pending_receive_sat().await.unwrap(), 0);
    assert_eq!(handle_alice.get_pending_send_sat().await.unwrap(), 0);
    assert_eq!(
        handle_alice.get_balance_sat().await.unwrap(),
        receiver_amount_sat
    );

    let payments = handle_alice.get_payments().await.unwrap();
    assert_eq!(payments.len(), 1);
    let payment = &payments[0];
    assert_eq!(payment.amount_sat, receiver_amount_sat);
    assert_eq!(payment.fees_sat, prepare_response.fees_sat);
    assert_eq!(payment.payment_type, PaymentType::Receive);
    assert_eq!(payment.status, PaymentState::Complete);
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
            payer_note: None,
        })
        .await
        .unwrap();
    let payer_amount_sat = receiver_amount_sat + prepare_response.fees_sat.unwrap();

    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentPending { .. }), TIMEOUT)
        .await
        .unwrap();

    assert_eq!(
        handle_alice.get_pending_send_sat().await.unwrap(),
        payer_amount_sat
    );
    assert_eq!(
        handle_alice.get_balance_sat().await.unwrap(),
        initial_balance - payer_amount_sat
    );

    utils::mine_blocks(1).await.unwrap();

    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    // TODO: this shouldn't be needed, but without it, sometimes get_balance_sat isn't updated in time
    // https://github.com/breez/breez-sdk-liquid/issues/828
    tokio::time::sleep(Duration::from_secs(10)).await;
    handle_alice.sdk.sync(false).await.unwrap();

    assert_eq!(handle_alice.get_pending_receive_sat().await.unwrap(), 0);
    assert_eq!(handle_alice.get_pending_send_sat().await.unwrap(), 0);
    assert_eq!(
        handle_alice.get_balance_sat().await.unwrap(),
        initial_balance - payer_amount_sat
    );

    let payments = handle_alice.get_payments().await.unwrap();
    assert_eq!(payments.len(), 2);
    let payment = &payments[0];
    assert_eq!(payment.amount_sat, receiver_amount_sat);
    assert_eq!(payment.fees_sat, prepare_response.fees_sat.unwrap());
    assert_eq!(payment.payment_type, PaymentType::Send);
    assert_eq!(payment.status, PaymentState::Complete);
    assert!(matches!(payment.details, PaymentDetails::Lightning { .. }));

    // -------------------MRH-------------------
    let receiver_amount_sat = 50_000;

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
            payer_note: None,
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

    utils::mine_blocks(1).await.unwrap();

    // TODO: figure out why on Wasm this event is occasionally skipped
    // https://github.com/breez/breez-sdk-liquid/issues/847
    let _ = handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await;
    handle_bob
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    // TODO: this shouldn't be needed, but without it, sometimes get_balance_sat isn't updated in time
    // https://github.com/breez/breez-sdk-liquid/issues/828
    tokio::time::sleep(Duration::from_secs(10)).await;
    handle_alice.sdk.sync(false).await.unwrap();
    handle_bob.sdk.sync(false).await.unwrap();

    assert_eq!(handle_bob.get_pending_receive_sat().await.unwrap(), 0);
    assert_eq!(handle_bob.get_pending_send_sat().await.unwrap(), 0);
    assert_eq!(
        handle_bob.get_balance_sat().await.unwrap(),
        receiver_amount_sat
    );

    let alice_payments = handle_alice.get_payments().await.unwrap();
    assert_eq!(alice_payments.len(), 3);
    let alice_payment = &alice_payments[0];
    assert_eq!(alice_payment.amount_sat, receiver_amount_sat);
    assert_eq!(
        alice_payment.fees_sat,
        prepare_response_send.fees_sat.unwrap()
    );
    assert_eq!(alice_payment.payment_type, PaymentType::Send);
    assert_eq!(alice_payment.status, PaymentState::Complete);
    assert!(matches!(
        alice_payment.details,
        PaymentDetails::Liquid { .. }
    ));

    let bob_payments = handle_bob.get_payments().await.unwrap();
    assert_eq!(bob_payments.len(), 1);
    let bob_payment = &bob_payments[0];
    assert_eq!(bob_payment.amount_sat, receiver_amount_sat);
    assert_eq!(bob_payment.fees_sat, 0);
    assert_eq!(bob_payment.payment_type, PaymentType::Receive);
    assert_eq!(bob_payment.status, PaymentState::Complete);
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
