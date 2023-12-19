use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{database::queries::update_post_text, state::AppState};

pub async fn update_post(
    state: State<AppState>,
    Path(post_id): Path<i32>,
    Json(body): Json<DtoUpdatePostText>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    update_post_text(state.db.clone(), post_id, &body.text)
        .await
        .map_err(|err| {
            tracing::error!("Error while updating post text: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not update post. Please try again later",
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize, Deserialize)]
pub struct DtoUpdatePostText {
    pub text: String,
}
