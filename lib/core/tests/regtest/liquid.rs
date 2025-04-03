use breez_sdk_liquid::model::{
    PayAmount, PaymentDetails, PaymentMethod, PaymentState, PaymentType, PrepareReceiveRequest,
    PrepareSendRequest, SdkEvent,
};
use serial_test::serial;

use crate::regtest::{utils, SdkNodeHandle, TIMEOUT};

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[sdk_macros::async_test_all]
#[serial]
async fn liquid() {
    let mut handle = SdkNodeHandle::init_node().await.unwrap();

    // --------------RECEIVE--------------

    let (prepare_response, receive_response) = handle
        .receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::LiquidAddress,
            amount: None,
        })
        .await
        .unwrap();

    assert!(prepare_response.amount.is_none());
    assert_eq!(prepare_response.fees_sat, 0);

    let address = receive_response.destination;
    let amount_sat = 100_000;

    utils::send_to_address_elementsd(&address, amount_sat)
        .await
        .unwrap();

    handle
        .wait_for_event(
            |e| matches!(e, SdkEvent::PaymentWaitingConfirmation { .. }),
            TIMEOUT,
        )
        .await
        .unwrap();

    assert_eq!(handle.get_pending_receive_sat().await.unwrap(), amount_sat);
    assert_eq!(handle.get_balance_sat().await.unwrap(), 0);

    utils::mine_blocks(1).await.unwrap();

    handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

    assert_eq!(handle.get_pending_receive_sat().await.unwrap(), 0);
    assert_eq!(handle.get_balance_sat().await.unwrap(), amount_sat);

    let payments = handle.get_payments().await.unwrap();
    assert_eq!(payments.len(), 1);
    let payment = &payments[0];
    assert_eq!(payment.amount_sat, amount_sat);
    assert_eq!(payment.fees_sat, 0);
    assert_eq!(payment.payment_type, PaymentType::Receive);
    assert_eq!(payment.status, PaymentState::Complete);
    assert!(matches!(payment.details, PaymentDetails::Liquid { .. }));

    // --------------SEND--------------

    let initial_balance_sat = handle.get_balance_sat().await.unwrap();
    let address = utils::generate_address_elementsd().await.unwrap();
    let receiver_amount_sat = 50_000;

    let (prepare_response, _) = handle
        .send_payment(&PrepareSendRequest {
            destination: address,
            amount: Some(PayAmount::Bitcoin {
                receiver_amount_sat,
            }),
        })
        .await
        .unwrap();

    let fees_sat = prepare_response.fees_sat.unwrap();
    let sender_amount_sat = receiver_amount_sat + fees_sat;

    utils::mine_blocks(1).await.unwrap();

    handle
        .wait_for_event(|e| matches!(e, SdkEvent::PaymentSucceeded { .. }), TIMEOUT)
        .await
        .unwrap();

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
    assert!(matches!(payment.details, PaymentDetails::Liquid { .. }));
}
