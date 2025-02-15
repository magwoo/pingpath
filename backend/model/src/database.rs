use anyhow::{Context, Result};
use rqlite_rs::{prelude::*, query::RqliteQuery};
use std::sync::Arc;

#[derive(Clone)]
pub struct Database(Arc<RqliteClient>);

impl Database {
    pub fn from_env() -> Result<Self> {
        let host = std::env::var("DATABASE_URL").context("missing database url env var")?;

        let inner = RqliteClientBuilder::new()
            .known_host(host)
            .build()
            .context("failed to build database")?;

        Ok(Self(Arc::new(inner)))
    }

    pub fn inner(&self) -> &RqliteClient {
        &self.0
    }

    pub async fn exec(&self, q: RqliteQuery) -> Result<QueryResult> {
        let query = q.query.clone();

        self.0
            .exec(q)
            .await
            .with_context(|| format!("failed to exec query({query})"))
    }

    pub async fn fetch(&self, q: RqliteQuery) -> Result<Vec<Row>> {
        let query = q.query.clone();

        self.0
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))
    }

    pub async fn fetch_optional(&self, q: RqliteQuery) -> Result<Option<Row>> {
        let query = q.query.clone();

        let row = self
            .0
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_iter()
            .next();

        Ok(row)
    }

    pub async fn fetch_one(&self, q: RqliteQuery) -> Result<Row> {
        let query = q.query.clone();

        self.0
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_iter()
            .next()
            .with_context(|| format!("missing fetch_one row on query({query})"))
    }
}
