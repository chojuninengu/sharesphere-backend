use axum::{routing::{get, post}, Json, Router};
use crate::db::queries::create_user;
use crate::db::models::User;

pub fn users_routes() -> Router {
    Router::new()
        .route("/users", post(create_new_user)) // Create a new user
}

async fn create_new_user(Json(user): Json<User>) -> Json<User> {
    let new_user = create_user(user.username, user.email, user.avatar_url).await;
    Json(new_user)
}
