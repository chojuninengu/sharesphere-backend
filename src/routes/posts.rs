use axum::{routing::get, Router};

pub fn posts_routes() -> Router {
    Router::new().route("/posts", get(get_posts))
}

async fn get_posts() -> &'static str {
    "List of posts"
}
