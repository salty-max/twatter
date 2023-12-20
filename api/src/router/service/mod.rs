pub mod status;

use axum::{routing::get, Router};

use crate::state::AppState;

pub fn create_service_router() -> Router<AppState> {
    Router::new().route("/status", get(status::get_status))
}
