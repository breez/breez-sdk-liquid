use nostr_sdk::{nips::nip47::NostrWalletConnectURI, PublicKey};
use serde::{Deserialize, Serialize};

use crate::{utils, DEFAULT_RELAY_URLS};

#[derive(Clone, Serialize, Deserialize)]
pub struct NwcConfig {
    /// A list of default relay urls to add per connection
    pub relay_urls: Option<Vec<String>>,
    /// Custom Nostr secret key (hex-encoded) for the wallet node
    pub secret_key_hex: Option<String>,
    /// Whether or not to start the notification listener event loop. True by default.
    /// Recommended to set to `Some(false)` if you only need event handling
    pub listen_to_events: Option<bool>,
}

impl NwcConfig {
    pub fn relays(&self) -> Vec<String> {
        self.relay_urls
            .clone()
            .unwrap_or(DEFAULT_RELAY_URLS.iter().map(|s| s.to_string()).collect())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PeriodicBudgetInner {
    pub used_budget_sat: u64,
    pub max_budget_sat: u64,
    /// The duration of the budget's period
    /// ## Dev Note:
    /// If the renewal time is less than [crate::MIN_REFRESH_INTERVAL_SEC] seconds,
    /// then it will take at most [crate::MIN_REFRESH_INTERVAL_SEC] seconds in order for the
    /// renewal to take effect.
    pub renewal_time_sec: Option<u32>,
    pub updated_at: u32,
}

impl PeriodicBudgetInner {
    pub(crate) fn from_budget_request(req: PeriodicBudgetRequest, updated_at: u32) -> Self {
        Self {
            used_budget_sat: 0,
            max_budget_sat: req.max_budget_sat,
            renewal_time_sec: req.renewal_time_mins.map(utils::mins_to_seconds),
            updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodicBudget {
    /// The amount of budget used (in satoshi) for the period
    /// Resets once every period ([PeriodicBudget::renews_at])
    pub used_budget_sat: u64,
    /// The maximum budget amount allowed (in satoshi) for the period
    pub max_budget_sat: u64,
    /// The next timestamp at which the budget will be renewed (reset to 0)
    pub renews_at: Option<u32>,
    /// The latest budget update time (last reset time)
    pub updated_at: u32,
}

impl From<PeriodicBudgetInner> for PeriodicBudget {
    fn from(b: PeriodicBudgetInner) -> Self {
        Self {
            used_budget_sat: b.used_budget_sat,
            max_budget_sat: b.max_budget_sat,
            renews_at: b.renewal_time_sec.map(|t| b.updated_at + t),
            updated_at: b.updated_at,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PeriodicBudgetRequest {
    /// See [PeriodicBudget::max_budget_sat]
    pub max_budget_sat: u64,
    /// The renewal time of the budget, in minutes.
    /// If not provided, the budget will be fixed for the entire lifetime of the connection.
    /// Otherwise, it will be reset after the amount provided
    pub renewal_time_mins: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NwcConnectionInner {
    pub connection_string: String,
    pub created_at: u32,
    pub receive_only: bool,
    pub paid_amount_sat: u64,
    /// The duration of the connection before it expires, in seconds
    /// ## Dev Note:
    /// If the expiry time is less than [crate::MIN_REFRESH_INTERVAL_SEC] seconds,
    /// then it will take at most [crate::MIN_REFRESH_INTERVAL_SEC] seconds in order for the
    /// connection to be deleted.
    pub expiry_time_sec: Option<u32>,
    pub periodic_budget: Option<PeriodicBudgetInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwcConnection {
    /// The NWC uri for the connection
    pub connection_string: String,
    /// The creation time of the connection
    pub created_at: u32,
    /// Specifies whether this is a receive-only connection. Defaults to false.
    pub receive_only: bool,
    /// The _total_ amount of funds sent for this connection, in satoshi
    pub paid_amount_sat: u64,
    /// The timestamp at which the connection expires
    pub expires_at: Option<u32>,
    /// An optional [PeriodicBudget] for the connection
    pub periodic_budget: Option<PeriodicBudget>,
}

impl From<NwcConnectionInner> for NwcConnection {
    fn from(c: NwcConnectionInner) -> Self {
        Self {
            connection_string: c.connection_string,
            created_at: c.created_at,
            receive_only: c.receive_only,
            paid_amount_sat: c.paid_amount_sat,
            expires_at: c.expiry_time_sec.map(|expiry| c.created_at + expiry),
            periodic_budget: c.periodic_budget.map(Into::into),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AddConnectionRequest {
    /// The **unique** name for the new connection
    pub name: String,
    /// The duration of the connection before it expires, in minutes
    pub expiry_time_mins: Option<u32>,
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
    /// The duration of the connection before it expires, in minutes
    pub expiry_time_mins: Option<u32>,
    /// Whether or not to remove the [NwcConnection::expires_at] field
    pub remove_expiry: Option<bool>,
    /// See [NwcConnection::receive_only]
    pub receive_only: Option<bool>,
    pub periodic_budget_req: Option<PeriodicBudgetRequest>,
    /// Whether or not to remove the [NwcConnection::periodic_budget] field
    pub remove_periodic_budget: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditConnectionResponse {
    pub connection: NwcConnection,
}

#[derive(Clone, Debug)]
pub(crate) struct ActiveConnection {
    pub connection: NwcConnectionInner,
    pub uri: NostrWalletConnectURI,
    pub pubkey: PublicKey,
}

#[derive(Default)]
pub(crate) struct RefreshResult {
    pub refreshed: Vec<String>,
    pub deleted: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NostrServiceInfo {
    pub wallet_pubkey: String,
    pub connected_relays: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct TrackedZap {
    pub zap_request: String,
    pub expires_at: u32,
}
