use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

pub async fn create_post(post: DtoPost) {
    tracing::debug!("{post:?}");
}

#[derive(Debug)]
pub struct DtoPost {
    pub text: String,
}

#[async_trait]
impl<S> FromRequest<S> for DtoPost
where
    Json<DtoCreatePost>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
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

        Ok(Self { text })
    }
}

#[derive(Deserialize, Debug)]
pub struct DtoCreatePost {
    pub text: Option<String>,
}
