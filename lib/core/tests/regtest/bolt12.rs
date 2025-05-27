use std::time::Duration;

use breez_sdk_liquid::model::{
    PayAmount, PaymentDetails, PaymentMethod, PaymentState, PaymentType, PrepareReceiveRequest,
    PrepareSendRequest, SdkEvent,
};
use serial_test::serial;
use tokio_with_wasm::alias as tokio;

use crate::regtest::{
    utils::{self, mine_blocks},
    ChainBackend, SdkNodeHandle, TIMEOUT,
};

#[cfg(feature = "browser-tests")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[sdk_macros::async_test_not_wasm]
#[serial]
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
async fn bolt12_electrum() {
    bolt12(ChainBackend::Electrum).await;
}

#[sdk_macros::async_test_all]
#[serial]
async fn bolt12_esplora() {
    bolt12(ChainBackend::Esplora).await;
}

async fn bolt12(chain_backend: ChainBackend) {
    let alice_mnemonic = bip39::Mnemonic::generate_in(bip39::Language::English, 12).unwrap();
    let bob_mnemonic = bip39::Mnemonic::generate_in(bip39::Language::English, 12).unwrap();

    let mut handle_alice = SdkNodeHandle::init_node(chain_backend, &alice_mnemonic)
        .await
        .unwrap();
    let mut handle_bob = SdkNodeHandle::init_node(chain_backend, &bob_mnemonic)
        .await
        .unwrap();

    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::Synced), TIMEOUT)
        .await
        .unwrap();
    handle_bob
        .wait_for_event(|e| matches!(e, SdkEvent::Synced), TIMEOUT)
        .await
        .unwrap();

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

    utils::mine_blocks(1).await.unwrap();

    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    // -------------------CREATE BOLT12 OFFER-------------------
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

    let (_, _) = handle_alice
        .send_payment(&PrepareSendRequest {
            destination: offer,
            amount: Some(PayAmount::Bitcoin {
                receiver_amount_sat,
            }),
            comment: None,
        })
        .await
        .unwrap();

    mine_blocks(1).await.unwrap();

    handle_bob
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    // TODO: this shouldn't be needed, but without it, sometimes get_balance_sat isn't updated in time
    // https://github.com/breez/breez-sdk-liquid/issues/828
    tokio::time::sleep(Duration::from_secs(1)).await;
    handle_alice.sdk.sync(false).await.unwrap();

    assert_eq!(handle_bob.get_pending_receive_sat().await.unwrap(), 0);
    assert_eq!(handle_bob.get_pending_send_sat().await.unwrap(), 0);
    assert_eq!(
        handle_bob.get_balance_sat().await.unwrap(),
        receiver_amount_sat
    );

    let alice_payments = handle_alice.get_payments().await.unwrap();
    assert_eq!(alice_payments.len(), 2);
    let alice_payment = &alice_payments[0];
    assert_eq!(alice_payment.amount_sat, receiver_amount_sat);
    // The prepare response gives the fees for a swap, so instead we test the Liquid fee
    assert_eq!(alice_payment.fees_sat, 26);
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
