use dotenvy_macro::dotenv;
use eyre::Result;
use gloo::net::http;
use yew::AttrValue;

use crate::types::{DtoCreatePostSuccess, DtoPost, DtoPostForm};

const API_URL: &str = dotenv!("API_URL");

pub async fn create_post(text: AttrValue) -> Result<DtoPost> {
    let data = DtoPostForm {
        text: text.to_string(),
    };

    let post = http::Request::post(&format!("{}/posts", API_URL))
        .json(&data)?
        .send()
        .await?
        .json::<DtoCreatePostSuccess>()
        .await?;

    Ok(DtoPost {
        id: post.id,
        text: data.text,
        likes: 0,
        reply_count: 0,
    })
}

pub async fn get_all_posts() -> Result<Vec<DtoPost>> {
    let res = http::Request::get(&format!("{}/posts", API_URL))
        .send()
        .await?
        .json::<Vec<DtoPost>>()
        .await?;

    Ok(res)
}
