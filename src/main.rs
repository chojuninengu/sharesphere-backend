use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::posts::posts_routes()) // Add posts routes
        .merge(routes::users::users_routes()) // Add users routes
        .merge(routes::interactions::interactions_routes()) // Add interactions routes
        .route("/", get(|| async { "Welcome to ShareSphere!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
