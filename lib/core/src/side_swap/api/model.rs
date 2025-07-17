use anyhow::Context as _;
use serde::{Deserialize, Serialize};
pub(crate) use sideswap_api::http_rpc::{Request as HttpRequest, Response as HttpResponse};
use sideswap_api::{
    Empty, StartSwapWebRequest, StartSwapWebResponse, SubscribePriceStreamRequest,
    SubscribePriceStreamResponse, SwapDoneNotification, UnsubscribePriceStreamRequest,
    UnsubscribePriceStreamResponse,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum RequestId {
    String(String),
    Int(i64),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub(crate) enum Request {
    Ping(Empty),
    SubscribePriceStream(SubscribePriceStreamRequest),
    UnsubscribePriceStream(UnsubscribePriceStreamRequest),
    StartSwapWeb(StartSwapWebRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct WrappedRequest {
    pub id: RequestId,
    #[serde(flatten)]
    pub request: Request,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "result", rename_all = "snake_case")]
pub(crate) enum Response {
    Ping(Empty),
    SubscribePriceStream(SubscribePriceStreamResponse),
    UnsubscribePriceStream(UnsubscribePriceStreamResponse),
    StartSwapWeb(StartSwapWebResponse),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum WrappedResponse {
    Notification {
        #[serde(flatten)]
        notification: Notification,
    },
    Response {
        id: RequestId,
        #[serde(flatten)]
        response: Response,
    },
    Error {
        id: Option<RequestId>,
        error: sideswap_api::Error,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub(crate) enum Notification {
    UpdatePriceStream(SubscribePriceStreamResponse),
    SwapDone(SwapDoneNotification),
}

/// The current state of a swap via the SideSwap service
#[derive(Debug, Clone, Serialize)]
pub(crate) struct AssetSwap {
    /// The asset we are currently trading for
    pub(crate) asset_id: String,
    /// The exchange rate of the asset (the amount that can be traded for one L-BTC)
    pub(crate) exchange_rate: f64,
    /// The service fees for the swap (in satoshi)
    pub(crate) fees_sat: u64,
    /// The asset amount which will be received after swapping (in satoshi precision)
    pub(crate) receiver_amount_sat: u64,
    /// The amount of L-BTC (in satoshi) to execute the swap
    pub(crate) payer_amount_sat: u64,
}

impl TryFrom<SubscribePriceStreamResponse> for AssetSwap {
    type Error = anyhow::Error;

    fn try_from(value: SubscribePriceStreamResponse) -> Result<Self, Self::Error> {
        if let Some(err) = &value.error_msg {
            anyhow::bail!(
                "Could not convert SideSwap price - received error message from stream: {err}"
            );
        }

        Ok(Self {
            asset_id: value.asset.to_string(),
            payer_amount_sat: value
                .send_amount
                .context("Expected send amount when creating side swap")?
                as u64,
            receiver_amount_sat: value
                .recv_amount
                .context("Expected receive amount when creating side swap")?
                as u64,
            fees_sat: value
                .fixed_fee
                .context("Expected fees when creating side swap")? as u64,
            exchange_rate: value
                .price
                .context("Expected price when creating side swap")?,
        })
    }
}
