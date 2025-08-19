use anyhow::Context as _;
use serde::{Deserialize, Serialize};
pub(crate) use sideswap_api::http_rpc::{Request as HttpRequest, Response as HttpResponse};
use sideswap_api::{
    AssetId, Empty, LoginClientRequest, LoginClientResponse, StartSwapWebRequest,
    StartSwapWebResponse, SubscribePriceStreamRequest, SubscribePriceStreamResponse,
    SwapDoneNotification, UnsubscribePriceStreamRequest, UnsubscribePriceStreamResponse,
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
    LoginClient(LoginClientRequest),
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
    LoginClient(LoginClientResponse),
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
    /// The asset we are trading from
    pub(crate) from_asset: AssetId,
    /// The asset we are trading to
    pub(crate) to_asset: AssetId,
    /// The exchange rate of the asset (the amount that can be traded for one L-BTC)
    pub(crate) exchange_rate: f64,
    /// The service fees for the swap (in satoshi precision)
    pub(crate) fees_sat: u64,
    /// The asset amount which will be received after swapping (in satoshi precision)
    pub(crate) receiver_amount_sat: u64,
    /// The asset amount required to execute the swap (in satoshi precision)
    pub(crate) payer_amount_sat: u64,
}

impl AssetSwap {
    pub(crate) fn try_from_price_stream_res(
        from_asset: AssetId,
        res: SubscribePriceStreamResponse,
    ) -> anyhow::Result<Self> {
        if let Some(err) = &res.error_msg {
            anyhow::bail!(
                "Could not convert SideSwap price - received error message from stream: {err}"
            );
        }

        Ok(Self {
            from_asset,
            to_asset: res.asset,
            payer_amount_sat: res
                .send_amount
                .context("Expected send amount when creating side swap")?
                as u64,
            receiver_amount_sat: res
                .recv_amount
                .context("Expected receive amount when creating side swap")?
                as u64,
            fees_sat: res
                .fixed_fee
                .context("Expected fees when creating side swap")? as u64,
            exchange_rate: res
                .price
                .context("Expected price when creating side swap")?,
        })
    }
}
