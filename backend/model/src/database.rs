use anyhow::{Context, Result};
use rqlite_rs::{prelude::*, query::RqliteQuery};

static mut INSTANCE: Option<RqliteClient> = None;

#[derive(Clone, Copy)]
pub struct Database;

impl Database {
    pub fn from_env() -> Result<Self> {
        let host = std::env::var("DATABASE_URL").context("missing database url env var")?;

        let inner = RqliteClientBuilder::new()
            .known_host(host)
            .build()
            .context("failed to build database")?;

        unsafe { INSTANCE = Some(inner) }

        Ok(Self)
    }

    #[inline]
    pub fn inner<'a>() -> &'a RqliteClient {
        inner()
    }

    pub async fn exec(q: RqliteQuery) -> Result<QueryResult> {
        let query = q.query.clone();

        Self::inner()
            .exec(q)
            .await
            .with_context(|| format!("failed to exec query({query})"))
    }

    pub async fn fetch(&self, q: RqliteQuery) -> Result<Vec<Row>> {
        let query = q.query.clone();

        Self::inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))
    }

    pub async fn fetch_optional(&self, q: RqliteQuery) -> Result<Option<Row>> {
        let query = q.query.clone();

        let row = Self::inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_iter()
            .next();

        Ok(row)
    }

    pub async fn fetch_one(&self, q: RqliteQuery) -> Result<Row> {
        let query = q.query.clone();

        Self::inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_iter()
            .next()
            .with_context(|| format!("missing fetch_one row on query({query})"))
    }
}

#[inline]
fn inner<'a>() -> &'a RqliteClient {
    unsafe { INSTANCE.as_ref().expect("Database instance uninitialized") }
}
