use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn get_status() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");

    let response = json!({
        "data": {
            "version": version
        },
        "message": "Service is running..."
    });

    (StatusCode::OK, Json(response))
}
