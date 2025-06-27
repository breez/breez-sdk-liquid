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
