use crate::db::models::{User, Post};
use crate::db::connection::establish_connection;
use sqlx::prelude::*;
use chrono::Utc;

pub async fn create_user(username: String, email: String, avatar_url: Option<String>) -> User {
    let pool = establish_connection().await;
    let query = r#"
        INSERT INTO users (username, email, avatar_url) 
        VALUES (?, ?, ?)
        RETURNING id, username, email, avatar_url
    "#;
    let user = sqlx::query_as::<_, User>(query)
        .bind(username)
        .bind(email)
        .bind(avatar_url)
        .fetch_one(&pool)
        .await
        .unwrap();
    user
}

pub async fn create_post(user_id: i64, content: String) -> Post {
    let pool = establish_connection().await;
    let query = r#"
        INSERT INTO posts (user_id, content, created_at) 
        VALUES (?, ?, ?)
        RETURNING id, user_id, content, created_at
    "#;
    let post = sqlx::query_as::<_, Post>(query)
        .bind(user_id)
        .bind(content)
        .bind(Utc::now().to_rfc3339())
        .fetch_one(&pool)
        .await
        .unwrap();
    post
}
