use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub mod create;
pub mod get;

pub fn create_posts_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get::get_all_top_level_posts))
        .route("/", post(create::create_post))
}
