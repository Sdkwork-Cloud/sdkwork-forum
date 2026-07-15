mod auth;
mod context;
mod dto;
mod iam;
mod infra_router;
mod middleware;
mod routes;

use axum::{middleware::from_fn, middleware::from_fn_with_state, Router};
use sdkwork_database_ops_http::{attach_ops_routes, BearerTokenOpsAuth, DatabaseOpsHttpState};
use sdkwork_forum_service_host::{default_seed_locale, default_seed_profile, ForumServiceHost};
use sdkwork_web_bootstrap::ServiceRouterConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub service_host: Arc<ForumServiceHost>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting SDKWork Forum API Server...");

    if iam::iam_enabled() {
        tracing::info!("IAM session resolution enabled");
    }

    let service_host = Arc::new(ForumServiceHost::new().await);
    let state = AppState {
        service_host: service_host.clone(),
    };

    let ops_auth = Arc::new(BearerTokenOpsAuth::from_env("SDKWORK_ACCESS_TOKEN"));
    let ops_state = DatabaseOpsHttpState::new(
        service_host.database_pool(),
        service_host.database_module(),
        default_seed_locale(),
        default_seed_profile(),
        ops_auth,
    );

    let app = infra_router::mount_service_routes(
        attach_ops_routes(
            Router::new()
                .merge(routes::build_forum_routes())
                .layer(from_fn(middleware::require_dual_token_auth))
                .layer(from_fn_with_state(state.clone(), iam::resolve_iam_context))
                .with_state(state),
            ops_state,
        )
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_FORUM_ENVIRONMENT"],
            &["SDKWORK_FORUM_CORS_ALLOWED_ORIGINS", "SDKWORK_CORS_ALLOWED_ORIGINS"],
        )),
        ServiceRouterConfig::default().with_always_ready(),
    );

    let addr = "0.0.0.0:8080";
    tracing::info!("Forum API server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
