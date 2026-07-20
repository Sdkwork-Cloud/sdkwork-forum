//! Application API assembly bootstrap for sdkwork-forum.

use std::sync::Arc;

use axum::middleware::{from_fn, from_fn_with_state};
use axum::Router;
use sdkwork_database_spi::{DefaultDatabaseModule, LocaleTag, SeedProfile};
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_forum_http_support::{iam, middleware, AppState};
use sdkwork_forum_service_host::{default_seed_locale, default_seed_profile, ForumServiceHost};

pub struct ApiAssembly {
    pub router: Router,
    pub database_pool: DatabasePool,
    pub database_module: Arc<DefaultDatabaseModule>,
    pub seed_locale: LocaleTag,
    pub seed_profile: SeedProfile,
}

pub async fn assemble_api_router() -> ApiAssembly {
    let service_host = Arc::new(ForumServiceHost::new().await);
    let state = AppState::new(Arc::clone(&service_host));
    let router = Router::new()
        .merge(sdkwork_routes_forum_app_api::gateway_mount())
        .merge(sdkwork_routes_forum_backend_api::gateway_mount())
        .merge(sdkwork_routes_forum_open_api::gateway_mount())
        .layer(from_fn(middleware::require_dual_token_auth))
        .layer(from_fn_with_state(state.clone(), iam::resolve_iam_context))
        .with_state(state);

    ApiAssembly {
        router,
        database_pool: service_host.database_pool(),
        database_module: service_host.database_module(),
        seed_locale: default_seed_locale(),
        seed_profile: default_seed_profile(),
    }
}
