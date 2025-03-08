use axum::{routing::{get, post}, Json, Router};
use crate::db::queries::create_post;
use crate::db::models::Post;


pub fn posts_routes() -> Router {
    Router::new()
        .route("/posts", post(create_new_post)) // Create a new post
}

async fn create_new_post(Json(post): Json<Post>) -> Json<Post> {
    let new_post = create_post(post.user_id, post.content).await;
    Json(new_post)
}
