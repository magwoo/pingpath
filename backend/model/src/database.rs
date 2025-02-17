use anyhow::{Context, Result};
use rqlite_rs::{prelude::*, query::RqliteQuery};

static mut INSTANCE: Option<RqliteClient> = None;

#[derive(Clone, Copy, Default)]
pub struct Rqlite;

pub trait Database: Sized + Default {
    fn from_env() -> Result<Self>;

    fn inner<'a>() -> &'a RqliteClient;

    fn exec(q: RqliteQuery) -> impl std::future::Future<Output = Result<QueryResult>>;

    fn fetch<T: FromRow>(q: RqliteQuery) -> impl std::future::Future<Output = Result<Vec<T>>>;

    fn fetch_optional<T: FromRow>(
        q: RqliteQuery,
    ) -> impl std::future::Future<Output = Result<Option<T>>>;

    fn fetch_one<T: FromRow>(q: RqliteQuery) -> impl std::future::Future<Output = Result<T>>;
}

impl Database for Rqlite {
    fn from_env() -> Result<Self> {
        let host = std::env::var("DATABASE_URL").context("missing database url env var")?;

        let inner = RqliteClientBuilder::new()
            .known_host(host)
            .build()
            .context("failed to build database")?;

        unsafe { INSTANCE = Some(inner) }

        Ok(Self)
    }

    #[inline]
    fn inner<'a>() -> &'a RqliteClient {
        inner()
    }

    async fn exec(q: RqliteQuery) -> Result<QueryResult> {
        let query = q.query.clone();

        Self::inner()
            .exec(q)
            .await
            .with_context(|| format!("failed to exec query({query})"))
    }

    async fn fetch<T: FromRow>(q: RqliteQuery) -> Result<Vec<T>> {
        let query = q.query.clone();

        Self::inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_typed()
            .context("failed to type")
    }

    async fn fetch_optional<T: FromRow>(q: RqliteQuery) -> Result<Option<T>> {
        let query = q.query.clone();

        let row = Self::inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_typed::<T>()
            .context("failed to type")?
            .into_iter()
            .next();

        Ok(row)
    }

    async fn fetch_one<T: FromRow>(q: RqliteQuery) -> Result<T> {
        let query = q.query.clone();

        let row = Self::inner()
            .fetch(q)
            .await
            .with_context(|| format!("failed to fetch query({query})"))?
            .into_iter()
            .next()
            .with_context(|| format!("missing fetch_one row on query({query})"))?;

        row.into_typed().context("failed to type")
    }
}

#[inline]
fn inner<'a>() -> &'a RqliteClient {
    #[allow(static_mut_refs)]
    unsafe {
        INSTANCE.as_ref().expect("Database instance uninitialized")
    }
}
