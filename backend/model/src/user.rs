use anyhow::Context;
use rqlite_rs::{query, FromRow};
use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::prelude::*;

pub mod profile;

pub trait GenericUser: Sized {
    fn from_id(id: i64, db: &impl Database) -> impl Future<Output = anyhow::Result<Option<Self>>>;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User<Ext = ()> {
    pub id: i64,
    #[serde(flatten)]
    pub ext: Ext,
}

impl User {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub async fn create_guest(db: &impl Database) -> anyhow::Result<Self> {
        let now = Datetime::now();

        let query = query!(
            "INSERT INTO users(username, created_at) VALUES (?, ?)",
            "guest",
            now
        )
        .context("failed to parse query")?;

        let id = db
            .exec(query)
            .await
            .map(|r| r.last_insert_id().expect("Row must be inserted"))?;

        Ok(Self { id, ext: () })
    }
}

impl<EXT: FromRow> FromRow for User<EXT> {
    fn from_row(row: rqlite_rs::Row) -> Result<Self, rqlite_rs::IntoTypedError> {
        Ok(Self {
            id: row.get("id")?,
            ext: EXT::from_row(row)?,
        })
    }
}
