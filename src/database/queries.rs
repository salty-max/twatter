use eyre::Result;
use serde::Serialize;

use super::connect::DB;

pub async fn save_post(db: DB, text: &str, parent_id: Option<i32>) -> Result<i32> {
    let result = sqlx::query!(
        "INSERT INTO posts (text, parent_id) VALUES ($1, $2) RETURNING post_id",
        text,
        parent_id
    )
    .fetch_one(&db)
    .await?;

    Ok(result.post_id)
}

pub async fn fetch_all_top_level_posts(db: DB) -> Result<Vec<Post>> {
    Ok(sqlx::query_as!(
        Post,
        "SELECT post_id as id, text, likes FROM posts WHERE parent_id IS NULL ORDER BY post_id"
    )
    .fetch_all(&db)
    .await?)
}

#[derive(Serialize)]
pub struct Post {
    id: i32,
    text: String,
    likes: i32,
}
