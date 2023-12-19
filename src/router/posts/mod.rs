use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::state::AppState;

pub mod create;
pub mod get;
pub mod update;

pub fn create_posts_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get::get_top_level_posts))
        .route("/:id", get(get::get_post))
        .route("/", post(create::create_post))
        .route("/:id", patch(update::update_post))
}
