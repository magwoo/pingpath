use anyhow::Context;
use axum::Router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .context("failed to bind addr")?;

    axum::serve(listener, router)
        .await
        .context("failed to serve")?;

    Ok(())
}
