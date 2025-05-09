use std::{collections::HashMap, time::Duration};

use log::debug;
use sideswap_api::{
    mkt::{Notification as MarketNotification, QuoteNotif, QuoteStatus, QuoteSubId},
    Notification,
};
use tokio::sync::RwLock;

pub(crate) struct SideSwapNotificationsHandler {
    quotes: RwLock<HashMap<QuoteSubId, QuoteNotif>>,
}

impl SideSwapNotificationsHandler {
    pub(crate) fn new() -> Self {
        Self {
            quotes: Default::default(),
        }
    }

    pub(crate) async fn handle_notification(&self, notif: Notification) {
        match notif {
            Notification::Market(MarketNotification::Quote(new_quote)) => {
                let mut quotes = self.quotes.write().await;
                quotes.insert(new_quote.quote_sub_id, new_quote);
            }
            notif => debug!("Received unhandled notification from SideSwap service: {notif:?}"),
        }
    }

    pub(crate) async fn wait_for_quote(
        &self,
        quote_sub_id: QuoteSubId,
        interval: Duration,
        mut max_retries: u64,
        successful_only: bool,
    ) -> Option<QuoteNotif> {
        while max_retries > 0 {
            if let Some(quote) = self.quotes.read().await.get(&quote_sub_id) {
                if matches!(quote.status, QuoteStatus::Success { .. }) || !successful_only {
                    return Some(quote.clone());
                }
            }
            tokio::time::sleep(interval).await;
            max_retries -= 1;
        }
        None
    }
}
