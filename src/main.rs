use axum::{Router, routing::get};

mod handlers;
use handlers::hello_world;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/api", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8067")
        .await.unwrap();

    println!("Server started successfully at 0.0.0.0:8067");
    axum::serve(listener, app)
        .await
        .unwrap();

}

