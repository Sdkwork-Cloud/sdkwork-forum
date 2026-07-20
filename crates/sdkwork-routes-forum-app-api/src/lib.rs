pub mod error;
pub mod handlers;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;
mod runtime;

pub use routes::{build_sdkwork_forum_app_api_router, RouteDescriptor};

pub fn gateway_mount() -> axum::Router<sdkwork_forum_http_support::AppState> {
    runtime::router()
}
