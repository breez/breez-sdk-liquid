use std::sync::Arc;
use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use breez_sdk_liquid::model::{
    PayAmount, PaymentDetails, PaymentMethod, PaymentState, PaymentType, PrepareReceiveRequest,
    PrepareSendRequest, ReceivePaymentRequest, SdkEvent,
};
use breez_sdk_liquid::sdk::LiquidSdk;
use serde::{Deserialize, Serialize};
use serial_test::serial;

use crate::regtest::{
    utils::{self, mine_blocks},
    SdkNodeHandle, TIMEOUT,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceRequestData {
    pub offer: String,
    #[serde(rename = "invoiceRequest")]
    pub invoice_request: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookRequest {
    pub data: InvoiceRequestData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookResponse {
    pub invoice: String,
}

#[derive(Clone)]
pub struct State {
    pub sdk: Arc<LiquidSdk>,
}

pub async fn handle_webhook(
    Extension(state): Extension<State>,
    Json(req): Json<WebhookRequest>,
) -> impl IntoResponse {
    let prepare_response = state
        .sdk
        .prepare_receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::Bolt12Invoice,
            amount: None,
            offer: Some(req.data.offer),
            invoice_request: Some(req.data.invoice_request),
        })
        .await
        .unwrap();
    let response = state
        .sdk
        .receive_payment(&ReceivePaymentRequest {
            prepare_response,
            description: None,
            use_description_hash: None,
        })
        .await
        .unwrap();
    let invoice = response.destination;
    (StatusCode::OK, Json(WebhookResponse { invoice })).into_response()
}

#[cfg(feature = "browser-tests")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[sdk_macros::async_test_not_wasm]
#[serial]
async fn bolt12() {
    let mut handle_alice = SdkNodeHandle::init_node().await.unwrap();
    let mut handle_bob = SdkNodeHandle::init_node().await.unwrap();

    handle_alice
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();
    handle_bob
        .wait_for_event(|e| matches!(e, SdkEvent::Synced { .. }), TIMEOUT)
        .await
        .unwrap();

    // -------------------SETUP-------------------
    // Setup Alice with some funds
    let (_, receive_response) = handle_alice
        .receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::LiquidAddress,
            amount: None,
            offer: None,
            invoice_request: None,
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

    // Setup a webhook server for Bob
    let state = State {
        sdk: handle_bob.sdk.clone(),
    };
    let router = Router::new()
        .route("/notify", post(handle_webhook))
        .layer(Extension(state));
    let addr = "localhost:7678";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tokio::spawn(async {
        axum::serve(listener, router.into_make_service())
            .await
            .unwrap();
    });

    // -------------------CREATE BOLT12 OFFER-------------------
    handle_bob
        .sdk
        .register_webhook(format!("http://host.docker.internal:7678/notify"))
        .await
        .unwrap();

    let (_, receive_response) = handle_bob
        .receive_payment(&PrepareReceiveRequest {
            payment_method: PaymentMethod::Bolt12Offer,
            amount: None,
            offer: None,
            invoice_request: None,
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
}
