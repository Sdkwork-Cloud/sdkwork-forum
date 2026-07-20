use std::sync::Arc;

use sdkwork_api_forum_assembly::assemble_api_router;
use sdkwork_api_forum_standalone_gateway::mount_service_routes;
use sdkwork_database_ops_http::{attach_ops_routes, BearerTokenOpsAuth, DatabaseOpsHttpState};
use sdkwork_web_bootstrap::ServiceRouterConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let assembly = assemble_api_router().await;
    let ops_state = DatabaseOpsHttpState::new(
        assembly.database_pool,
        assembly.database_module,
        assembly.seed_locale,
        assembly.seed_profile,
        Arc::new(
            BearerTokenOpsAuth::from_env("SDKWORK_ACCESS_TOKEN")
                .expect("SDKWORK_ACCESS_TOKEN is required for Forum database operations"),
        ),
    );
    let app = mount_service_routes(
        attach_ops_routes(assembly.router, ops_state).layer(
            sdkwork_web_bootstrap::application_cors_layer_from_env(
                &["SDKWORK_FORUM_ENVIRONMENT"],
                &[
                    "SDKWORK_FORUM_CORS_ALLOWED_ORIGINS",
                    "SDKWORK_CORS_ALLOWED_ORIGINS",
                ],
            ),
        ),
        ServiceRouterConfig::default().with_always_ready(),
    );

    let bind_address = std::env::var("SDKWORK_FORUM_APPLICATION_PUBLIC_INGRESS_BIND")
        .expect("SDKWORK_FORUM_APPLICATION_PUBLIC_INGRESS_BIND must come from a topology profile");
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("bind Forum standalone gateway");
    tracing::info!(%bind_address, "sdkwork-api-forum-standalone-gateway listening");
    axum::serve(listener, app)
        .await
        .expect("serve Forum standalone gateway");
}
