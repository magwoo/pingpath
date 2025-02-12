use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum_cookie::cookie::{Cookie, CookieBuilder};
use axum_cookie::CookieManager;

pub const DEV_TOKEN: &str = "11dc7bac0042b71d690a29b2c2787ccb89f9014471902b5667daff7c9b407c9c";
pub const DEV_GUEST_TOKEN: &str =
    "84983c60f7daadc1cb8698621f802c0d9f9a3c3c295c810748fb048115c186ec";

pub fn get_nest() -> Router {
    Router::new().route("/dev", post(dev_auth))
}

async fn dev_auth(cookie: CookieManager) -> Result<impl IntoResponse, StatusCode> {
    if !cfg!(debug_assertions) {
        return Err(StatusCode::FORBIDDEN);
    }

    cookie.set(CookieBuilder::new("token", DEV_TOKEN).path("/").build());

    Ok(StatusCode::OK)
}

pub async fn auth_middleware<B>(
    cookie: CookieManager,
    req: Request<B>,
) -> Result<Request<B>, StatusCode> {
    if let Some(token) = cookie.get("token") {
        if token.value() == DEV_TOKEN || token.value() == DEV_GUEST_TOKEN {
            return Ok(req);
        }

        Err(StatusCode::UNAUTHORIZED)
    } else {
        cookie.set(Cookie::new("token", DEV_GUEST_TOKEN).with_path("/"));

        Ok(req)
    }
}
