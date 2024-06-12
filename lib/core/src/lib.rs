#[cfg(feature = "frb")]
pub(crate) mod bindings;
pub mod error;
pub(crate) mod event;
#[cfg(feature = "frb")]
pub(crate) mod frb_generated;
pub mod logger;
pub mod model;
pub mod persist;
pub(crate) mod receive_swap;
pub mod sdk;
pub(crate) mod send_swap;
pub(crate) mod swapper;
pub(crate) mod utils;
pub(crate) mod wallet;

pub use sdk_common::prelude::*;

// === FRB mirroring
//
// This section contains frb "mirroring" structs and enums.
// These are needed by the flutter bridge in order to use structs defined in an external crate.
// See <https://cjycode.com/flutter_rust_bridge/v1/feature/lang_external.html#types-in-other-crates>

use flutter_rust_bridge::frb;

#[frb(mirror(Network))]
pub enum _Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

#[frb(mirror(LNInvoice))]
pub struct _LNInvoice {
    pub bolt11: String,
    pub network: Network,
    pub payee_pubkey: String,
    pub payment_hash: String,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub amount_msat: Option<u64>,
    pub timestamp: u64,
    pub expiry: u64,
    pub routing_hints: Vec<RouteHint>,
    pub payment_secret: Vec<u8>,
    pub min_final_cltv_expiry_delta: u64,
}

#[frb(mirror(RouteHint))]
pub struct _RouteHint {
    pub hops: Vec<RouteHintHop>,
}

#[frb(mirror(RouteHintHop))]
pub struct _RouteHintHop {
    pub src_node_id: String,
    pub short_channel_id: u64,
    pub fees_base_msat: u32,
    pub fees_proportional_millionths: u32,
    pub cltv_expiry_delta: u64,
    pub htlc_minimum_msat: Option<u64>,
    pub htlc_maximum_msat: Option<u64>,
}
