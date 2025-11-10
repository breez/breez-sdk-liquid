use nostr_sdk::nips::nip47::NostrWalletConnectURI;
use serde::{Deserialize, Serialize};

use crate::DEFAULT_RELAY_URLS;

#[derive(Clone, Serialize, Deserialize)]
pub struct NwcConfig {
    /// A list of default relay urls to add per connection
    pub relay_urls: Option<Vec<String>>,
    /// Custom Nostr secret key (hex-encoded) for the wallet node
    pub secret_key_hex: Option<String>,
}

impl NwcConfig {
    pub fn relays(&self) -> Vec<String> {
        self.relay_urls
            .clone()
            .unwrap_or(DEFAULT_RELAY_URLS.iter().map(|s| s.to_string()).collect())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PeriodicBudget {
    /// The amount of budget used (in satoshi) for the period
    /// Resets once every period ([PeriodicBudget::reset_time_sec])
    pub used_budget_sat: u64,
    /// The maximum budget amount allowed (in satoshi) for the period
    pub max_budget_sat: u64,
    /// The duration of the budget's period
    /// ## Dev Note:
    /// If the reset time is less than [crate::EXPIRY_CHECK_INTERVAL_SEC] seconds,
    /// then it will take at most [crate::EXPIRY_CHECK_INTERVAL_SEC] seconds in order for the
    /// reset to take effect.
    pub reset_time_sec: u32,
    /// The latest budget update time (last reset time)
    pub updated_at: u32,
}

impl PeriodicBudget {
    pub(crate) fn from_budget_request(req: PeriodicBudgetRequest, updated_at: u32) -> Self {
        Self {
            used_budget_sat: 0,
            max_budget_sat: req.max_budget_sat,
            reset_time_sec: req.reset_time_sec,
            updated_at,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PeriodicBudgetRequest {
    /// See [PeriodicBudget::max_budget_sat]
    pub max_budget_sat: u64,
    /// See [PeriodicBudget::reset_time_sec]
    pub reset_time_sec: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NwcConnection {
    /// The NWC uri for the connection
    pub connection_string: String,
    /// The creation time of the connection
    pub created_at: u32,
    /// Specifies whether this is a receive-only connection. Defaults to false.
    pub receive_only: bool,
    /// The expiry time of the connection
    /// ## Dev Note:
    /// If the expiry time is less than [crate::EXPIRY_CHECK_INTERVAL_SEC] seconds,
    /// then it will take at most [crate::EXPIRY_CHECK_INTERVAL_SEC] seconds in order for the
    /// connection to be deleted.
    pub expiry_time_sec: Option<u32>,
    /// An optional [PeriodicBudget] for the connection
    pub periodic_budget: Option<PeriodicBudget>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AddConnectionRequest {
    /// The **unique** name for the new connection
    pub name: String,
    /// See [NwcConnection::expiry_time_sec]
    pub expiry_time_sec: Option<u32>,
    /// See [NwcConnection::receive_only]
    pub receive_only: Option<bool>,
    pub periodic_budget_req: Option<PeriodicBudgetRequest>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AddConnectionResponse {
    pub connection: NwcConnection,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditConnectionRequest {
    /// The **unique** name for the new connection
    pub name: String,
    /// See [NwcConnection::expiry_time_sec]
    pub expiry_time_sec: Option<u32>,
    /// See [NwcConnection::receive_only]
    pub receive_only: Option<bool>,
    pub periodic_budget_req: Option<PeriodicBudgetRequest>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditConnectionResponse {
    pub connection: NwcConnection,
}

pub(crate) struct ActiveConnection {
    pub connection: NwcConnection,
    pub uri: NostrWalletConnectURI,
}
