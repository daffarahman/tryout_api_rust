use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;

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

async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "Hello, World!"
    });
    Json(json_response)
}
