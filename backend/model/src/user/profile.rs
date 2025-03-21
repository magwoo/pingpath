use anyhow::Context;
use rqlite_rs::{query, FromRow};
use serde::{Deserialize, Serialize};

use super::{GenericUser, User};
use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct WithProfile {
    username: String,
    img_url: Option<String>,
    #[serde(skip)]
    github_token: Option<String>,
}

impl GenericUser for User<WithProfile> {
    async fn from_id(id: i64, db: impl Database) -> anyhow::Result<Option<Self>> {
        let query =
            query!("SELECT * FROM users WHERE id = ?", id).context("failed to parse query")?;

        db.fetch_optional(query)
            .await
            .with_context(|| format!("id: {id}"))
    }
}

impl FromRow for WithProfile {
    fn from_row(row: rqlite_rs::Row) -> Result<Self, rqlite_rs::IntoTypedError> {
        Ok(Self {
            username: row.get("username")?,
            img_url: row.get_opt("img_url")?,
            github_token: row.get_opt("github_token")?,
        })
    }
}
