use anyhow::Context;
use rqlite_rs::{query, FromRow};

use self::github::UserGithub;
use crate::prelude::*;

pub mod github;

pub struct User {
    id: i64,
    username: String,
    created_at: Datetime,
    github: Option<UserGithub>,
}

impl User {
    pub async fn create_guest(db: Database) -> anyhow::Result<i64> {
        let now = Datetime::now();

        let query = query!(
            "INSERT INTO users(username, created_at) VALUES (?, ?)",
            "guest",
            now
        )
        .context("failed to parse query")?;

        db.exec(query)
            .await
            .map(|r| r.last_insert_id().expect("Row must be inserted"))
    }
}

impl FromRow for User {
    fn from_row(row: rqlite_rs::Row) -> Result<Self, rqlite_rs::IntoTypedError> {
        Ok(Self {
            id: row.get("id")?,
            username: row.get("username")?,
            created_at: row.get("created_at")?,
            github: match row.get_opt::<String>("guthub_token")?.is_some() {
                true => Some(UserGithub::from_row(row)?),
                false => None,
            },
        })
    }
}
