#![allow(unused_imports)]

#[cfg(feature = "browser")]
mod browser;
#[cfg(feature = "browser")]
pub(crate) use browser::{
    create_db_backup_persister, create_onchain_wallet, create_wallet_persister,
};

#[cfg(feature = "node-js")]
mod node_js;
#[cfg(feature = "node-js")]
pub(crate) use node_js::{
    create_db_backup_persister, create_onchain_wallet, create_wallet_persister,
};

#[cfg(all(not(feature = "browser"), not(feature = "node-js")))]
mod default;
#[cfg(all(not(feature = "browser"), not(feature = "node-js")))]
pub(crate) use default::{create_db_backup_persister, create_onchain_wallet};
