use axum::{routing::get, Router};
use hyper::server::conn::AddrIncoming;
use hyper::service::make_service_fn;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Welcome to ShareSphere!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Create a TCP listener instead of directly using `Server::bind`
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
