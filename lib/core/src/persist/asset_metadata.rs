use anyhow::Result;
use rusqlite::{Connection, Row};

use super::{AssetMetadata, Persister};

impl Persister {
    pub(crate) fn replace_asset_metadata(
        &self,
        asset_metadata: Option<Vec<AssetMetadata>>,
    ) -> Result<()> {
        let con = self.get_connection()?;
        con.execute("DELETE FROM asset_metadata WHERE is_default = 0", [])?;
        if let Some(asset_metadata) = asset_metadata {
            for am in asset_metadata {
                con.execute(
                    "INSERT INTO asset_metadata (asset_id, name, ticker, precision, fiat_id) VALUES (?, ?, ?, ?, ?)",
                    (am.asset_id, am.name, am.ticker, am.precision, am.fiat_id),
                )?;
            }
        }

        Ok(())
    }

    pub(crate) fn list_asset_metadata(&self) -> Result<Vec<AssetMetadata>> {
        let con = self.get_connection()?;
        let mut stmt = con.prepare(
            "SELECT asset_id, 
            name, 
            ticker, 
            precision, 
            fiat_id
        FROM asset_metadata",
        )?;
        let asset_metadata: Vec<AssetMetadata> = stmt
            .query_map([], Self::sql_row_to_asset_metadata)?
            .map(|i| i.unwrap())
            .collect();
        Ok(asset_metadata)
    }

    pub(crate) fn get_asset_metadata(&self, asset_id: &str) -> Result<Option<AssetMetadata>> {
        let con: Connection = self.get_connection()?;
        let mut stmt = con.prepare(
            "SELECT asset_id, 
            name, 
            ticker, 
            precision, 
            fiat_id 
        FROM asset_metadata
        WHERE asset_id = ?",
        )?;
        let res = stmt.query_row([asset_id], Self::sql_row_to_asset_metadata);

        Ok(res.ok())
    }

    fn sql_row_to_asset_metadata(row: &Row) -> rusqlite::Result<AssetMetadata> {
        Ok(AssetMetadata {
            asset_id: row.get(0)?,
            name: row.get(1)?,
            ticker: row.get(2)?,
            precision: row.get(3)?,
            fiat_id: row.get(4)?,
        })
    }
}
