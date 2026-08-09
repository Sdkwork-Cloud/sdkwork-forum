//! Forum app-api gateway route manifest derived from the authored route descriptors.

use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

use crate::routes::{RouteDescriptor, APP_ROUTES};

fn http_method(method: &str) -> HttpMethod {
    match method {
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "PATCH" => HttpMethod::Patch,
        "DELETE" => HttpMethod::Delete,
        _ => HttpMethod::Get,
    }
}

fn http_route(route: &RouteDescriptor) -> HttpRoute {
    let method = http_method(route.method);
    match route.auth_mode {
        "public" | "anonymous" => {
            HttpRoute::public(method, route.path, "forum", route.operation_id)
        }
        "api-key" => HttpRoute::api_key(method, route.path, "forum", route.operation_id),
        _ => HttpRoute::dual_token(method, route.path, "forum", route.operation_id),
    }
}

pub fn gateway_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::from_owned_routes(APP_ROUTES.iter().map(http_route).collect())
}
