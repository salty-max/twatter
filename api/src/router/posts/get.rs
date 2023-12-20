use axum::{extract::{State, Path}, http::StatusCode, Json};

use crate::{
    database::queries::{fetch_top_level_posts, Post, PostWithReplies, fetch_post},
    state::AppState,
};

pub async fn get_top_level_posts(
    state: State<AppState>,
) -> Result<(StatusCode, Json<Vec<Post>>), (StatusCode, &'static str)> {
    let res = fetch_top_level_posts(state.db.clone())
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

pub async fn get_post(state: State<AppState>, Path(post_id): Path<i32>) -> Result<(StatusCode, Json<PostWithReplies>), (StatusCode, &'static str)> { 
    let post = fetch_post(state.db.clone(), post_id).await.map_err(|err| {
        tracing::error!("Error fetching one post: {err}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not get post, please try again.")
    })?;

    let Some(res) = post else {
        return Err((StatusCode::NOT_FOUND, "Post does not exist."));
    };

    Ok((StatusCode::OK, Json(res)))
}
