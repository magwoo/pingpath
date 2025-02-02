use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

pub fn get_nest() -> Router {
    Router::new()
        .route("/", get(get_profile))
        .route("/username", get(get_profile_username))
}

async fn get_profile() -> Result<impl IntoResponse, StatusCode> {
    let mock_data = serde_json::json!({
        "username": "boris2001",
        "iconUrl": "https://avatars.githubusercontent.com/u/114882188?v=4",
        "type": "Full",
        "addressAmount": 12
    });

    Ok(Json(mock_data.to_string()))
}

async fn get_profile_username() -> Result<impl IntoResponse, StatusCode> {
    let mock_data = serde_json::json!({
        "username": "boris2001"
    });

    Ok(Json(mock_data.to_string()))
}
