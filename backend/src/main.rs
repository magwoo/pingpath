use anyhow::Context;
use axum::Router;
use axum_cookie::CookieLayer;

mod auth;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .nest("/auth", auth::get_nest())
        .layer(CookieLayer::strict());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .context("failed to bind addr")?;

    axum::serve(listener, router)
        .await
        .context("failed to serve")?;

    Ok(())
}
