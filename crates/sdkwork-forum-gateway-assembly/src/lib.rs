//! Gateway assembly for sdkwork-forum.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_application_router, ApplicationAssembly};
pub use sdkwork_routes_forum_app_api::build_sdkwork_forum_app_api_router;
pub use sdkwork_routes_forum_backend_api::build_sdkwork_forum_backend_api_router;
pub use sdkwork_routes_forum_open_api::build_sdkwork_forum_open_api_router;

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
