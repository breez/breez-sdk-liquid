use crate::{model::Bolt12Offer, test_utils::generate_random_string, utils};

pub fn new_bolt12_offer(description: Option<String>, webhook_url: Option<String>) -> Bolt12Offer {
    Bolt12Offer {
        id: generate_random_string(32),
        description: description.unwrap_or("default".to_string()),
        private_key: "945affeef55f12227f1d4a3f80a17062a05b229ddc5a01591eb5ddf882df92e3".to_string(),
        webhook_url: webhook_url.unwrap_or("http://localhost:4004/notify".to_string()),
        created_at: utils::now(),
    }
}
