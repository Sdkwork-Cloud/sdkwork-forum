mod app;
mod backend;
mod open;

use axum::Router;
use super::AppState;

pub fn build_forum_routes() -> Router<AppState> {
    Router::new()
        .merge(app::router())
        .merge(backend::router())
        .merge(open::router())
}
