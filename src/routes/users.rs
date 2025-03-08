use axum::{routing::get, Router};

pub fn users_routes() -> Router {
    Router::new().route("/users", get(get_users))
}

async fn get_users() -> &'static str {
    "List of users"
}
