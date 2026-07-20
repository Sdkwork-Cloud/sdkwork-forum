pub mod error;
pub mod handlers;
pub mod http_route_manifest;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;
mod runtime;
pub mod web_bootstrap;

use sdkwork_web_core::HttpRouteManifest;

pub use http_route_manifest::open_route_manifest;
pub use routes::{build_sdkwork_forum_open_api_router, RouteDescriptor};
pub use web_bootstrap::{
    forum_open_api_prefixes, forum_open_api_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};

pub fn gateway_route_manifest() -> HttpRouteManifest {
    open_route_manifest()
}

pub fn gateway_mount() -> axum::Router<sdkwork_forum_http_support::AppState> {
    runtime::router()
}
