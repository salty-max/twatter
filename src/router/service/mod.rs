pub mod status;

use axum::{routing::get, Router};

pub fn create_service_router() -> Router {
    Router::new().route("/status", get(status::get_status))
}
