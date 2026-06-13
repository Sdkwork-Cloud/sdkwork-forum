mod dto;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use sdkwork_forum_service_host::ForumServiceHost;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub service_host: Arc<ForumServiceHost>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting SDKWork Forum API Server...");

    let service_host = Arc::new(ForumServiceHost::new().await);
    let state = AppState { service_host };

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/app/v3/api/forum/boards", get(handlers::list_boards))
        .route("/app/v3/api/forum/topics", get(handlers::list_topics).post(handlers::create_topic))
        .route("/app/v3/api/forum/topics/{topic_id}", get(handlers::retrieve_topic))
        .route("/app/v3/api/forum/topics/{topic_id}/replies", get(handlers::list_replies).post(handlers::create_reply))
        .route("/app/v3/api/forum/topics/{topic_id}/vote", post(handlers::vote_topic))
        .route("/app/v3/api/forum/topics/{topic_id}/bookmark", post(handlers::bookmark_topic))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:8080";
    tracing::info!("Forum API server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
