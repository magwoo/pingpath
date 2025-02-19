use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use axum_cookie::CookieManager;
use model::prelude::*;

use crate::auth::DEV_TOKEN;

pub fn get_nest() -> Router<Rqlite> {
    Router::new()
        .route("/", get(get_profile))
        .route("/summary", get(get_profile_summary))
}

async fn get_profile(cookie: CookieManager) -> Result<impl IntoResponse, StatusCode> {
    let token = cookie.get("token").unwrap();

    let mock_data = if token.value() == DEV_TOKEN {
        serde_json::json!({
            "username": "boris2001",
            "imgUrl": "https://avatars.githubusercontent.com/u/114882188?v=4",
            "type": "Full",
            "addressAmount": 12
        })
    } else {
        serde_json::json!({
            "username": "guest",
            "type": "Trial"
        })
    };

    Ok(Json(mock_data.to_string()))
}

async fn get_profile_summary(cookie: CookieManager) -> Result<impl IntoResponse, StatusCode> {
    let token = cookie.get("token").unwrap();

    let mock_data = if token.value() == DEV_TOKEN {
        serde_json::json!({
            "username": "boris2001",
            "imgUrl": "https://avatars.githubusercontent.com/u/114882188?v=4"
        })
    } else {
        serde_json::json!({
            "username": "guest"
        })
    };

    Ok(Json(mock_data.to_string()))
}
