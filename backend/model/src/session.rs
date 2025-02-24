use anyhow::Context;
use chrono::{Duration, Timelike};
use rqlite_rs::query;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::{future::Future, sync::LazyLock};

use crate::prelude::*;

static HASH_SALT: LazyLock<String> =
    LazyLock::new(|| std::env::var("HASH_SALT").expect("missing `HASH_SALT` env var"));

pub trait GenericSession: Sized {
    fn new(user: impl Into<User>, db: impl Database) -> impl Future<Output = anyhow::Result<Self>>;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session<Ext = ()> {
    id: i64,
    token: String,
    created_at: Datetime,
    expire_date: Datetime,
    #[serde(flatten)]
    ext: Ext,
}

impl GenericSession for Session {
    async fn new(user: impl Into<User>, db: impl Database) -> anyhow::Result<Self> {
        let user_id = user.into().id();
        let now = Datetime::now();
        let expire_date = now + Duration::days(7);
        let token = Self::generate_token(user_id, now);

        let query = query!(
            "INSERT INTO sessions(user_id, token, created_at, expire_date) VALUES (?, ?, ?, ?)",
            user_id,
            token,
            now,
            expire_date
        )
        .context("failed to parse query")?;

        let id = db
            .exec(query)
            .await
            .with_context(|| format!("failed to create session for user({user_id})"))?
            .last_insert_id()
            .expect("Session must be created");

        Ok(Self {
            id,
            token: token.to_string(),
            created_at: now,
            expire_date,
            ext: (),
        })
    }
}

impl Session {
    pub fn generate_token(user_id: i64, now: Datetime) -> String {
        let mut hasher = Sha512::new();

        hasher.update(&*HASH_SALT);
        hasher.update(user_id.to_le_bytes());
        hasher.update(&*HASH_SALT);
        hasher.update(format!("{}", now));
        hasher.update(&*HASH_SALT);
        hasher.update(now.nanosecond().to_le_bytes());
        hasher.update(&*HASH_SALT);

        format!("{:x}", hasher.finalize())
    }
}
