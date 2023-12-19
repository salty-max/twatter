use eyre::Result;
use serde::{Deserialize, Serialize};

use super::connect::DB;

pub async fn save_post(db: DB, text: &str, parent_id: Option<i32>) -> Result<i32> {
    let result = sqlx::query!(
        r#"
            INSERT INTO posts (text, parent_id) 
            VALUES ($1, $2) 
            RETURNING post_id
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
                    SELECT 
                        COUNT(*) 
                    FROM posts p2 
                    WHERE p2.parent_id = p.post_id
                ) AS reply_count 
            FROM posts p 
            WHERE p.parent_id IS NULL 
            GROUP BY post_id;
        "#
    )
    .fetch_all(&db)
    .await?)
}

pub async fn fetch_post(db: DB, post_id: i32) -> Result<Option<PostWithReplies>> {
    let db_post = sqlx::query_as!(
        DbPostWithReplies,
        r#"
            WITH PostDetails AS (
                SELECT
                    p.post_id AS "id!",
                    p.text AS "text!",
                    p.likes AS "likes!",
                    p.parent_id,
                    COALESCE(r.reply_count, 0) AS reply_count
                FROM posts p
                LEFT JOIN (
                    SELECT
                        parent_id,
                        COUNT(*) AS reply_count
                    FROM posts
                    GROUP BY parent_id
                ) r ON p.post_id = r.parent_id
                WHERE p.post_id = $1
            ),
            Replies AS (
                SELECT
                    p.post_id AS "id!",
                    p.text AS "text!",
                    p.likes AS "likes!",
                    p.parent_id,
                    COALESCE(r.reply_count, 0) AS reply_count
                FROM posts p
                LEFT JOIN (
                    SELECT
                        parent_id,
                        COUNT(*) AS reply_count
                    FROM posts
                    GROUP BY parent_id
                ) r ON p.post_id = r.parent_id
                WHERE p.parent_id = $1
            )
            SELECT * FROM PostDetails
            UNION
            SELECT * FROM Replies
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

impl TryFrom<DbPostWithReplies> for PostWithReplies {
    type Error = eyre::Error;

    fn try_from(value: DbPostWithReplies) -> std::result::Result<Self, Self::Error> {
        let replies = vec![];

        Ok(Self {
            id: value.id,
            text: value.text.clone(),
            likes: value.likes,
            replies,
        })
    }
}
