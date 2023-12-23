use eyre::Result;
use serde::{Deserialize, Serialize};

use super::connect::DB;

pub async fn save_post(db: DB, text: &str, parent_id: Option<i32>) -> Result<i32> {
    let result = sqlx::query!(
        r#"
            INSERT INTO posts (text, parent_id) 
            VALUES ($1, $2)
            RETURNING post_id;
        "#,
        text,
        parent_id
    )
    .fetch_one(&db)
    .await?;

    Ok(result.post_id)
}

pub async fn fetch_top_level_posts(db: DB) -> Result<Vec<Post>> {
    Ok(sqlx::query_as!(
        Post,
        r#"
            SELECT 
                p.post_id AS id,
                p.text,
                p.likes,
                (
                    SELECT COUNT(*) 
                    FROM posts r 
                    WHERE r.parent_id = p.post_id
                ) AS reply_count 
            FROM posts p
            WHERE p.parent_id IS NULL
            AND p.deleted_at IS NULL
            ORDER BY post_id DESC;
        "#
    )
    .fetch_all(&db)
    .await?)
}

pub async fn fetch_post(db: DB, post_id: i32) -> Result<Option<PostWithReplies>> {
    let db_post = sqlx::query_as!(
        DbPostWithReplies,
        r#"
        SELECT
            post_id AS "id!",
            text AS "text!",
            likes AS "likes!",
            parent_id,
            (
                SELECT COUNT(*)
                FROM posts
                WHERE parent_id = $1
            ) AS reply_count
            FROM posts
            WHERE post_id = $1
            AND deleted_at IS NULL
            UNION
            SELECT
                p.post_id AS "id!",
                p.text AS "text!",
                p.likes AS "likes!",
                p.parent_id,
                (
                    SELECT COUNT(*)
                    FROM posts r
                    WHERE r.parent_id = p.post_id
                ) AS reply_count
            FROM posts p
            WHERE p.parent_id = $1
            AND p.deleted_at IS NULL
            ORDER BY "id!" ASC;
        "#,
        post_id
    )
    .fetch_all(&db)
    .await?;

    let mut posts = db_post.into_iter();

    let Some(first) = posts.next() else {
        return Ok(None);
    };

    if first.id != post_id {
        return Ok(None);
    }

    let replies = posts
        .map(|reply| Post {
            id: reply.id,
            text: reply.text,
            likes: reply.likes,
            reply_count: reply.reply_count,
        })
        .collect();

    Ok(Some(PostWithReplies {
        id: first.id,
        text: first.text,
        likes: first.likes,
        replies,
    }))
}

pub async fn update_post_text(db: DB, post_id: i32, new_text: &str) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE posts
            SET text = $1
            WHERE post_id = $2;            
        "#,
        new_text,
        post_id
    )
    .execute(&db)
    .await?;

    Ok(())
}

pub async fn soft_delete_post(db: DB, post_id: i32) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE posts
            SET deleted_at = NOW()
            WHERE post_id = $1
        "#,
        post_id
    )
    .execute(&db)
    .await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: i32,
    text: String,
    likes: i32,
    reply_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbPostWithReplies {
    id: i32,
    text: String,
    likes: i32,
    parent_id: Option<i32>,
    reply_count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PostWithReplies {
    id: i32,
    text: String,
    likes: i32,
    replies: Vec<Post>,
}
