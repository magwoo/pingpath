use anyhow::{Context, Result};
use rqlite_rs::prelude::*;
use rqlite_rs::query::RqliteQuery;
use std::sync::Arc;

#[derive(Clone)]
pub struct Rqlite(Arc<RqliteClient>);

pub trait Database: Sized {
    fn from_env() -> Result<Self>;

    fn exec(&self, q: RqliteQuery) -> impl std::future::Future<Output = Result<QueryResult>>;

    fn fetch<T: FromRow>(
        &self,
        q: RqliteQuery,
    ) -> impl std::future::Future<Output = Result<Vec<T>>>;

    fn fetch_optional<T: FromRow>(
        &self,
        q: RqliteQuery,
    ) -> impl std::future::Future<Output = Result<Option<T>>>;

    fn fetch_one<T: FromRow>(&self, q: RqliteQuery)
        -> impl std::future::Future<Output = Result<T>>;
}

impl Rqlite {
    pub fn inner(&self) -> &RqliteClient {
        &self.0
    }
}

impl Database for Rqlite {
    fn from_env() -> Result<Self> {
        let host = std::env::var("DATABASE_URL").context("missing database url env var")?;

        let inner = RqliteClientBuilder::new()
            .known_host(host)
            .build()
            .context("failed to build database")?;

        Ok(Self(Arc::new(inner)))
    }

    async fn exec(&self, q: RqliteQuery) -> Result<QueryResult> {
        let query = q.query.clone();

        self.inner()
            .exec(q)
            .await
            .with_context(|| format!("failed to exec query({query})"))
    }

    async fn fetch<T: FromRow>(&self, q: RqliteQuery) -> Result<Vec<T>> {
        let query = q.query.clone();

        self.inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_typed()
            .context("failed to type")
    }

    async fn fetch_optional<T: FromRow>(&self, q: RqliteQuery) -> Result<Option<T>> {
        let query = q.query.clone();

        let row = self
            .inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_typed::<T>()
            .context("failed to type")?
            .into_iter()
            .next();

        Ok(row)
    }

    async fn fetch_one<T: FromRow>(&self, q: RqliteQuery) -> Result<T> {
        let query = q.query.clone();

        let row = self
            .inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_iter()
            .next()
            .with_context(|| format!("missing fetch_one row on query({query})"))?;

        row.into_typed().context("failed to type")
    }
}
