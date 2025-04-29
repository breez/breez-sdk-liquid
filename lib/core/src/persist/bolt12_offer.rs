use anyhow::Result;
use rusqlite::{params, Connection, Params, Row};

use crate::model::*;
use crate::persist::Persister;
use crate::sync::model::data::Bolt12OfferSyncData;
use crate::sync::model::RecordType;

impl Persister {
    pub(crate) fn insert_or_update_bolt12_offer_inner(
        con: &Connection,
        bolt12_offer: &Bolt12Offer,
    ) -> Result<()> {
        con.execute(
            "
            INSERT INTO bolt12_offers (
                id,
                description,
                private_key,
                webhook_url,
                created_at
            )
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT (id)
            DO UPDATE SET webhook_url = excluded.webhook_url
            ",
            (
                &bolt12_offer.id,
                &bolt12_offer.description,
                &bolt12_offer.private_key,
                &bolt12_offer.webhook_url,
                &bolt12_offer.created_at,
            ),
        )?;

        Ok(())
    }

    pub(crate) fn insert_or_update_bolt12_offer(&self, bolt12_offer: &Bolt12Offer) -> Result<()> {
        let maybe_bolt12_offer = self.fetch_bolt12_offer_by_id(&bolt12_offer.id)?;
        let updated_fields = Bolt12OfferSyncData::updated_fields(maybe_bolt12_offer, bolt12_offer);

        let mut con = self.get_connection()?;
        let tx = con.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

        Self::insert_or_update_bolt12_offer_inner(&tx, bolt12_offer)?;

        // Trigger a sync if:
        // - updated_fields is None (swap is inserted, not updated)
        // - updated_fields in a non empty list of updated fields
        if updated_fields.as_ref().is_none_or(|u| !u.is_empty()) {
            self.commit_outgoing(
                &tx,
                &bolt12_offer.id,
                RecordType::Bolt12Offer,
                updated_fields,
            )?;
            tx.commit()?;
            self.trigger_sync();
        } else {
            tx.commit()?;
        }

        Ok(())
    }

    fn list_bolt12_offers_query(where_clauses: Vec<String>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT
                id,
                description,
                private_key,
                webhook_url,
                created_at
            FROM bolt12_offers
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_bolt12_offer_by_id(&self, id: &str) -> Result<Option<Bolt12Offer>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_bolt12_offers_query(vec!["id = ?".to_string()]);
        let res = con.query_row(&query, [id], Self::sql_row_to_bolt12_offer);

        Ok(res.ok())
    }

    pub(crate) fn fetch_bolt12_offer_by_description(
        &self,
        description: &str,
    ) -> Result<Option<Bolt12Offer>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_bolt12_offers_query(vec!["description = ?".to_string()]);
        let res = con.query_row(&query, [description], Self::sql_row_to_bolt12_offer);

        Ok(res.ok())
    }

    fn sql_row_to_bolt12_offer(row: &Row) -> rusqlite::Result<Bolt12Offer> {
        Ok(Bolt12Offer {
            id: row.get(0)?,
            description: row.get(1)?,
            private_key: row.get(2)?,
            webhook_url: row.get(3)?,
            created_at: row.get(4)?,
        })
    }

    pub(crate) fn list_bolt12_offers(&self) -> Result<Vec<Bolt12Offer>> {
        let con: Connection = self.get_connection()?;
        self.list_bolt12_offers_where(&con, vec![], params![])
    }

    pub(crate) fn list_bolt12_offers_where<P>(
        &self,
        con: &Connection,
        where_clauses: Vec<String>,
        params: P,
    ) -> Result<Vec<Bolt12Offer>>
    where
        P: Params,
    {
        let query = Self::list_bolt12_offers_query(where_clauses);
        let offers = con
            .prepare(&query)?
            .query_map(params, Self::sql_row_to_bolt12_offer)?
            .map(|i| i.unwrap())
            .collect();
        Ok(offers)
    }

    pub(crate) fn list_bolt12_offers_by_webhook_url(
        &self,
        webhook_url: &str,
    ) -> Result<Vec<Bolt12Offer>> {
        let con = self.get_connection()?;
        let where_clause = vec!["webhook_url = ?".to_string()];
        self.list_bolt12_offers_where(&con, where_clause, params![webhook_url])
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{bolt12_offer::new_bolt12_offer, persist::create_persister};
    use anyhow::{anyhow, Result};
    use rusqlite::params;

    #[cfg(feature = "browser-tests")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[sdk_macros::test_all]
    fn test_fetch_bolt12_offer() -> Result<()> {
        create_persister!(storage);
        let bolt12_offer = new_bolt12_offer(None, Some("http://localhost:4004/notify".to_string()));

        storage.insert_or_update_bolt12_offer(&bolt12_offer)?;
        // Fetch bolt12 offer by id
        assert!(storage.fetch_send_swap_by_id(&bolt12_offer.id).is_ok());
        // Fetch bolt12 offer by description
        assert!(storage
            .fetch_bolt12_offer_by_description(&bolt12_offer.description)
            .is_ok());

        Ok(())
    }

    #[sdk_macros::test_all]
    fn test_list_bolt12_offers() -> Result<()> {
        create_persister!(storage);
        storage.insert_or_update_bolt12_offer(&new_bolt12_offer(
            None,
            Some("http://localhost:4004/notify".to_string()),
        ))?;
        storage.insert_or_update_bolt12_offer(&new_bolt12_offer(
            Some("another".to_string()),
            Some("http://other.local:4004/notify".to_string()),
        ))?;
        storage.insert_or_update_bolt12_offer(&new_bolt12_offer(
            Some("another-other".to_string()),
            Some("http://localhost:4004/notify".to_string()),
        ))?;

        let con = storage.get_connection()?;
        let offers = storage.list_bolt12_offers_where(&con, vec![], params![])?;
        assert_eq!(offers.len(), 3);

        let offers = storage.list_bolt12_offers_by_webhook_url("http://localhost:4004/notify")?;
        assert_eq!(offers.len(), 2);

        Ok(())
    }

    #[sdk_macros::test_all]
    fn test_update_bolt12_offer() -> Result<()> {
        create_persister!(storage);

        let mut bolt12_offer =
            new_bolt12_offer(None, Some("http://localhost:4004/notify".to_string()));
        storage.insert_or_update_bolt12_offer(&bolt12_offer)?;

        let offers = storage.list_bolt12_offers_by_webhook_url("http://localhost:4004/notify")?;
        assert_eq!(offers.len(), 1);

        bolt12_offer.webhook_url = Some("http://other.local:4004/notify".to_string());
        storage.insert_or_update_bolt12_offer(&bolt12_offer)?;

        let offers = storage.list_bolt12_offers_by_webhook_url("http://localhost:4004/notify")?;
        assert_eq!(offers.len(), 0);

        let offers = storage.list_bolt12_offers_by_webhook_url("http://other.local:4004/notify")?;
        assert_eq!(offers.len(), 1);

        let mut offer = storage
            .fetch_bolt12_offer_by_id(&bolt12_offer.id)?
            .ok_or(anyhow!("Could not find BOLT12 offer in database"))?;

        assert_eq!(offer.id, bolt12_offer.id);
        assert_eq!(
            offer.webhook_url,
            Some("http://other.local:4004/notify".to_string())
        );

        offer.webhook_url = None;
        storage.insert_or_update_bolt12_offer(&offer)?;

        let offers = storage.list_bolt12_offers_by_webhook_url("http://other.local:4004/notify")?;
        assert_eq!(offers.len(), 0);

        Ok(())
    }
}
