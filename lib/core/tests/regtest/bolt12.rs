use breez_sdk_liquid::model::{
    PayAmount, PaymentDetails, PaymentMethod, PaymentState, PaymentType, PrepareReceiveRequest,
    PrepareSendRequest, SdkEvent,
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
async fn bolt12_electrum() {
    let handle_alice = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    let handle_bob = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    bolt12(handle_alice, handle_bob).await;
}

#[sdk_macros::async_test_all]
#[serial]
async fn bolt12_esplora() {
    let handle_alice = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    let handle_bob = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    bolt12(handle_alice, handle_bob).await;
}

#[sdk_macros::async_test_not_wasm]
#[serial]
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn bolt12_mixed() {
    let handle_alice = SdkNodeHandle::init_node(ChainBackend::Esplora)
        .await
        .unwrap();
    let handle_bob = SdkNodeHandle::init_node(ChainBackend::Electrum)
        .await
        .unwrap();
    bolt12(handle_alice, handle_bob).await;
}

async fn bolt12(mut handle_alice: SdkNodeHandle, mut handle_bob: SdkNodeHandle) {
    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();
    handle_bob
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();

    let indexers = utils::Indexers::from_handles(&[&handle_alice, &handle_bob]);

    // -------------------SETUP-------------------
    // Setup Alice with some funds
    let (_, receive_response) = handle_alice
        .receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::LiquidAddress,
            amount: None,
        })
        .await
        .unwrap();

    let address = receive_response.destination;
    let amount_sat = 200_000;

    utils::send_to_address_elementsd(&address, amount_sat)
        .await
        .unwrap();

    handle_alice
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    handle_alice.assert_wallet_pending(amount_sat, 0, 0).await;

    // Confirm the server lockup and wait for swap to complete
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();

    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    handle_alice.assert_wallet_settled(amount_sat).await;

    // -------------------RECEIVE SWAP-------------------
    // TODO: Receive to an offer using the CLN node

    let (_, receive_response) = handle_bob
        .receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::Bolt12Offer,
            amount: None,
        })
        .await
        .unwrap();
    let offer = receive_response.destination;

    // -------------------SEND SWAP-------------------
    // TODO: Pay an offer using the CLN node

    // -------------------MRH-------------------
    let receiver_amount_sat = 50_000;

    let (prepare_response_send, _) = handle_alice
        .send_payment(&PrepareSendRequest {
            destination: offer,
            amount: Some(PayAmount::Bitcoin {
                receiver_amount_sat,
            }),
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
    handle_bob
        .assert_wallet_pending(receiver_amount_sat, 0, 0)
        .await;

    // Confirm the send TX and wait for swap to complete
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();

    handle_bob
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    // Workaround for #828: mine an extra Liquid block so that sync() sees a
    // new tip and takes the full scan path, refreshing the LWK wallet state.
    utils::mine_and_index_blocks(1, utils::Chain::Liquid, Some(&indexers))
        .await
        .unwrap();
    handle_alice.sdk.sync(false).await.unwrap();
    handle_bob.sdk.sync(false).await.unwrap();

    // For BOLT12 MRH payments, prepare_send_payment estimates a submarine swap fee.
    // At send_payment time the SDK detects the MRH and routes via a direct Liquid tx,
    // so the actual fee is lower. Read back from the Liquid Esplora indexer.
    let prepare_fees_sat = prepare_response_send.fees_sat.unwrap();
    let alice_payments = handle_alice.get_payments().await.unwrap();
    assert_eq!(alice_payments.len(), 2);
    let alice_payment = &alice_payments[0];
    let alice_tx_id = alice_payment.tx_id.as_ref().unwrap();
    let actual_fees_sat = utils::get_lbtc_tx_fee_sat(alice_tx_id).await.unwrap();
    assert!(actual_fees_sat <= prepare_fees_sat);

    handle_alice
        .assert_wallet_settled(amount_sat - receiver_amount_sat - actual_fees_sat)
        .await;

    utils::assert_payment(
        alice_payment,
        receiver_amount_sat,
        actual_fees_sat,
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
    handle_bob.assert_wallet_settled(receiver_amount_sat).await;

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
