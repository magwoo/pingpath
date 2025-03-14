use anyhow::Context;
use rqlite_rs::prelude::*;
use rqlite_rs::query::RqliteQuery;
use std::future::Future;
use std::sync::Arc;

#[derive(Clone)]
pub struct Rqlite(Arc<RqliteClient>);

pub trait Database: Sized + Clone {
    fn from_env() -> anyhow::Result<Self>;

    fn exec(&self, q: RqliteQuery) -> impl Future<Output = anyhow::Result<QueryResult>>;

    fn fetch<T: FromRow>(&self, q: RqliteQuery) -> impl Future<Output = anyhow::Result<Vec<T>>>;

    fn fetch_optional<T: FromRow>(
        &self,
        q: RqliteQuery,
    ) -> impl Future<Output = anyhow::Result<Option<T>>>;

    fn fetch_one<T: FromRow>(&self, q: RqliteQuery) -> impl Future<Output = anyhow::Result<T>>;
}

impl Rqlite {
    pub fn inner(&self) -> &RqliteClient {
        &self.0
    }
}

impl Database for Rqlite {
    fn from_env() -> anyhow::Result<Self> {
        let host = std::env::var("DATABASE_URL").context("missing database url env var")?;

        let inner = RqliteClientBuilder::new()
            .known_host(host)
            .build()
            .context("failed to build database")?;

        Ok(Self(Arc::new(inner)))
    }

    async fn exec(&self, q: RqliteQuery) -> anyhow::Result<QueryResult> {
        let query = q.query.clone();

        self.inner()
            .exec(q)
            .await
            .with_context(|| format!("failed to exec query({query})"))
    }

    async fn fetch<T: FromRow>(&self, q: RqliteQuery) -> anyhow::Result<Vec<T>> {
        let query = q.query.clone();

        self.inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_typed()
            .context("failed to type")
    }

    async fn fetch_optional<T: FromRow>(&self, q: RqliteQuery) -> anyhow::Result<Option<T>> {
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

    async fn fetch_one<T: FromRow>(&self, q: RqliteQuery) -> anyhow::Result<T> {
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
