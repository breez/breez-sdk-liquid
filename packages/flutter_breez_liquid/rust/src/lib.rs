pub mod duplicates;
pub mod errors;
pub mod events;
mod frb_generated;
pub mod logger;
pub mod models;
pub mod nwc;
pub mod plugin;
pub mod sdk;

use flutter_rust_bridge::frb;
pub use sdk::BreezSdkLiquid;

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

#[frb(ignore)]
static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

#[frb(ignore)]
pub(crate) fn rt() -> &'static Runtime {
    &RT
}
