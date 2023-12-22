use dotenvy_macro::dotenv;
use eyre::Result;
use gloo::net::http;
use serde::{Deserialize, Serialize};
use yew::AttrValue;

const API_URL: &str = dotenv!("API_URL");

pub async fn create_post(text: AttrValue) -> Result<DtoPost> {
    let data = DtoPostForm {
        text: text.to_string(),
    };

    let result = http::Request::post(&format!("{}/posts", API_URL))
        .json(&data)?
        .send()
        .await?
        .json::<DtoCreatePostSuccess>()
        .await?;

    Ok(DtoPost {
        id: result.id,
        text,
    })
}

#[derive(Serialize)]
pub struct DtoPostForm {
    text: String,
}

#[derive(Deserialize)]
pub struct DtoCreatePostSuccess {
    id: i32,
}

#[derive(Debug)]
pub struct DtoPost {
    pub id: i32,
    pub text: AttrValue,
}
