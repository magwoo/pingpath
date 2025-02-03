use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum_cookie::CookieManager;

pub const DEV_TOKEN: &str = "11dc7bac0042b71d690a29b2c2787ccb89f9014471902b5667daff7c9b407c9c";

pub fn get_nest() -> Router {
    Router::new().route("/dev", post(dev_auth))
}

async fn dev_auth(cookie: CookieManager) -> Result<impl IntoResponse, StatusCode> {
    if !cfg!(debug_assertions) {
        return Err(StatusCode::FORBIDDEN);
    }

    cookie.set(("token", DEV_TOKEN));

    Ok(StatusCode::OK)
}

pub async fn auth_middleware<B>(
    cookie: CookieManager,
    req: Request<B>,
) -> Result<Request<B>, StatusCode> {
    let token = cookie.get("token").ok_or(StatusCode::UNAUTHORIZED)?;

    if token.value() == DEV_TOKEN {
        return Ok(req);
    }

    Err(StatusCode::UNAUTHORIZED)
}
