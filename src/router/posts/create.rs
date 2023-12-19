use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::queries::{is_post_deleted, save_post},
    state::AppState,
};

pub async fn create_post(
    state: State<AppState>,
    post: DtoPost,
) -> Result<(StatusCode, Json<DtoCreatePostResult>), (StatusCode, &'static str)> {
    let res = save_post(state.db.clone(), &post.text, post.parent_id)
        .await
        .map_err(|err| {
            tracing::error!("Error inserting post into database: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Post could not be created at this time, please try again later.",
            )
        })?;

    Ok((StatusCode::CREATED, Json(DtoCreatePostResult { id: res })))
}

#[derive(Serialize)]
pub struct DtoCreatePostResult {
    pub id: i32,
}

#[derive(Debug)]
pub struct DtoPost {
    pub text: String,
    pub parent_id: Option<i32>,
}

#[async_trait]
impl FromRequest<AppState> for DtoPost
where
    Json<DtoCreatePost>: FromRequest<AppState, Rejection = JsonRejection>,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(post) = Json::<DtoCreatePost>::from_request(req, state)
            .await
            .map_err(|err| {
                tracing::error!("Error extracting body from request: {}", err.body_text());
                err.status().into_response()
            })?;

        let Some(text) = post.text else {
            return Err((StatusCode::BAD_REQUEST).into_response());
        };

        if text.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Text must have at least one character.",
            )
                .into_response());
        }

        if text.len() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Text must be 255 characters or less.",
            )
                .into_response());
        }

        if let Some(parent_id) = post.parent_id {
            if parent_id <= 0 {
                return Err((StatusCode::BAD_REQUEST).into_response());
            }

            let is_deleted = is_post_deleted(state.db.clone(), parent_id)
                .await
                .map_err(|err| {
                    tracing::error!("Error checking if parent is deleted: {err}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "There was a problem creating the new post.",
                    )
                        .into_response()
                })?;

            if is_deleted {
                return Err((
                    StatusCode::NOT_FOUND,
                    "Cannot reply to a non-existing post.",
                )
                    .into_response());
            }
        }

        Ok(Self {
            text,
            parent_id: post.parent_id,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct DtoCreatePost {
    pub text: Option<String>,
    parent_id: Option<i32>,
}
