use axum::{routing::post, Router};

use crate::state::AppState;

pub mod create;

pub fn create_posts_router() -> Router<AppState> {
    Router::new().route("/", post(create::create_post))
}
