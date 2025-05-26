use serde::{Deserialize, Serialize};
// pub(crate) use sideswap_api::RequestId;
// use sideswap_api::{
//     Empty, StartSwapWebRequest, StartSwapWebResponse, SubscribePriceStreamRequest,
//     SubscribePriceStreamResponse, SwapDoneNotification, UnsubscribePriceStreamRequest,
//     UnsubscribePriceStreamResponse,
// };
//

use sideswap_api::{
    Empty, StartSwapWebRequest, SubscribePriceStreamRequest, UnsubscribePriceStreamRequest,
};

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

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// pub(crate) enum Response {
//     Ping(Empty),
//     SubscribePriceStream(SubscribePriceStreamResponse),
//     UnsubscribePriceStream(UnsubscribePriceStreamResponse),
//     StartSwapWeb(StartSwapWebResponse),
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// pub(crate) enum Notification {
//     UpdatePriceStream(SubscribePriceStreamResponse),
//     SwapDone(SwapDoneNotification),
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// pub(crate) enum WrappedResponse {
//     Notification {
//         method: String,
//         params: Notification,
//     },
//     Response {
//         id: RequestId,
//         method: String,
//         result: Response,
//     },
//     Error {
//         id: RequestId,
//         error: sideswap_api::Error,
//     },
// }

pub type RequestIdInt = i64;
pub type RequestIdString = String;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum RequestId {
    String(RequestIdString),
    Int(RequestIdInt),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Response {
    // Ping(Empty),
    SubscribePriceStream(SubscribePriceStreamResponse),
    // UnsubscribePriceStream(UnsubscribePriceStreamResponse),
    // StartSwapWeb(StartSwapWebResponse),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum WrappedResponse {
    Notification {
        method: String,
        params: Notification,
    },
    Response {
        id: RequestId,
        method: String,
        result: Response,
    },
    Error {
        id: RequestId,
        error: String, // sideswap_api::Error,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Notification {
    UpdatePriceStream(SubscribePriceStreamResponse),
    SwapDone(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribePriceStreamResponse {
    pub subscribe_id: Option<String>,
    pub asset: AssetId,
    pub send_bitcoins: bool,
    pub send_amount: Option<i64>,
    pub recv_amount: Option<i64>,
    pub fixed_fee: Option<i64>,
    pub price: Option<f64>,
    pub error_msg: Option<String>,
}

pub type AssetId = crate::elements::AssetId;
