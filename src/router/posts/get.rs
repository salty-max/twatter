use axum::{extract::State, http::StatusCode, Json};

use crate::{
    database::queries::{fetch_all_top_level_posts, Post},
    state::AppState,
};

pub async fn get_all_top_level_posts(
    state: State<AppState>,
) -> Result<(StatusCode, Json<Vec<Post>>), (StatusCode, &'static str)> {
    let res = fetch_all_top_level_posts(state.db.clone())
        .await
        .map_err(|err| {
            tracing::error!("Error fetching posts from database: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not fetch top level posts.",
            )
        })?;

    Ok((StatusCode::OK, Json(res)))
}
