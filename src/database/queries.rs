use eyre::Result;
use serde::{Deserialize, Serialize};

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
        "SELECT p.post_id as id, p.text, p.likes, (SELECT COUNT(*) FROM posts p2 WHERE p2.parent_id = p.post_id) as replies FROM posts p WHERE p.parent_id IS NULL GROUP BY post_id;"
    )
    .fetch_all(&db)
    .await?)
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: i32,
    text: String,
    likes: i32,
    replies: Option<i64>,
}
