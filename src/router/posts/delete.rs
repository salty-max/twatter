use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use eyre::Result;

use crate::{database::queries::soft_delete_post, state::AppState};

pub async fn delete_post(
    state: State<AppState>,
    Path(post_id): Path<i32>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    soft_delete_post(state.db.clone(), post_id)
        .await
        .map_err(|err| {
            tracing::error!("Error while soft deleting post: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not delete post, please try again later.",
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
