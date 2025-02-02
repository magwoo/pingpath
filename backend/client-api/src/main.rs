use anyhow::Context;
use axum::middleware::map_request;
use axum::Router;
use axum_cookie::CookieLayer;

mod auth;
mod profile;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let without_auth = Router::new().nest("/auth", auth::get_nest());

    let with_auth = Router::new()
        .nest("/profile", profile::get_nest())
        .layer(map_request(auth::auth_middleware));

    let router = Router::new()
        .merge(without_auth)
        .merge(with_auth)
        .layer(CookieLayer::strict());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .context("failed to bind addr")?;

    axum::serve(listener, router)
        .await
        .context("failed to serve")?;

    Ok(())
}
