use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: String,
}
