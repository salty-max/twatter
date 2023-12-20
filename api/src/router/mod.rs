pub mod posts;
mod service;

use axum::Router;
use tower_http::trace;
use tracing::Level;

use crate::state::AppState;

use self::{posts::create_posts_router, service::create_service_router};

pub fn create_main_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/service", create_service_router())
        .nest("/api/v1/posts", create_posts_router())
        .with_state(state)
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
