//! Shared HTTP state, context extraction, and security middleware for Forum routes.

pub mod auth;
pub mod context;
pub mod dto;
pub mod iam;
pub mod middleware;

use std::sync::Arc;

use sdkwork_forum_service_host::ForumServiceHost;

#[derive(Clone)]
pub struct AppState {
    pub service_host: Arc<ForumServiceHost>,
}

impl AppState {
    pub fn new(service_host: Arc<ForumServiceHost>) -> Self {
        Self { service_host }
    }
}
