mod dto;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use handlers::{bookmark_topic, create_reply, create_topic, health, list_boards, list_replies, list_topics, retrieve_topic, vote_topic};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/app/v3/api/forum/boards", get(list_boards))
        .route("/app/v3/api/forum/topics", get(list_topics).post(create_topic))
        .route("/app/v3/api/forum/topics/:topic_id", get(retrieve_topic))
        .route("/app/v3/api/forum/topics/:topic_id/vote", post(vote_topic))
        .route("/app/v3/api/forum/topics/:topic_id/bookmark", post(bookmark_topic))
        .route("/app/v3/api/forum/topics/:topic_id/replies", get(list_replies).post(create_reply))
        .layer(CorsLayer::permissive());

    let addr = "0.0.0.0:8080";
    println!("🚀 Forum API server starting on {}", addr);
    println!("📡 Health check: http://{}/health", addr);
    println!("📋 Topics: http://{}/app/v3/api/forum/topics", addr);
    println!("📋 Boards: http://{}/app/v3/api/forum/boards", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
