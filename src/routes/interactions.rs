use axum::{routing::get, Router};

pub fn interactions_routes() -> Router {
    Router::new().route("/interactions", get(get_interactions))
}

async fn get_interactions() -> &'static str {
    "List of interactions (likes, comments, shares)"
}
