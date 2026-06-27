pub fn compose_forum_api_routes() -> Vec<ForumRouteInfo> {
    let _assembly = sdkwork_forum_gateway_assembly::assemble_application_router();
    let app_routes = sdkwork_forum_gateway_assembly::build_sdkwork_forum_app_api_router();
    let backend_routes = sdkwork_forum_gateway_assembly::build_sdkwork_forum_backend_api_router();
    let open_routes = sdkwork_forum_gateway_assembly::build_sdkwork_forum_open_api_router();

    let mut all_routes: Vec<ForumRouteInfo> = Vec::new();

    for route in &app_routes {
        all_routes.push(ForumRouteInfo {
            surface: route.surface.to_string(),
            method: route.method.to_string(),
            path: route.path.to_string(),
            operation_id: route.operation_id.to_string(),
            auth_mode: route.auth_mode.to_string(),
        });
    }

    for route in &backend_routes {
        all_routes.push(ForumRouteInfo {
            surface: route.surface.to_string(),
            method: route.method.to_string(),
            path: route.path.to_string(),
            operation_id: route.operation_id.to_string(),
            auth_mode: route.auth_mode.to_string(),
        });
    }

    for route in &open_routes {
        all_routes.push(ForumRouteInfo {
            surface: route.surface.to_string(),
            method: route.method.to_string(),
            path: route.path.to_string(),
            operation_id: route.operation_id.to_string(),
            auth_mode: route.auth_mode.to_string(),
        });
    }

    all_routes
}

#[derive(Debug, Clone)]
pub struct ForumRouteInfo {
    pub surface: String,
    pub method: String,
    pub path: String,
    pub operation_id: String,
    pub auth_mode: String,
}

pub fn route_count() -> usize {
    compose_forum_api_routes().len()
}

pub fn route_count_by_surface(surface: &str) -> usize {
    compose_forum_api_routes().iter().filter(|r| r.surface == surface).count()
}

fn path_matches(template: &str, actual: &str) -> bool {
    let template_segments: Vec<&str> = template.split('/').collect();
    let actual_segments: Vec<&str> = actual.split('/').collect();
    if template_segments.len() != actual_segments.len() {
        return false;
    }
    template_segments.iter().zip(actual_segments.iter()).all(|(t, a)| {
        t.starts_with('{') || t == a
    })
}

pub fn find_route(method: &str, path: &str) -> Option<ForumRouteInfo> {
    compose_forum_api_routes().into_iter().find(|r| r.method == method && path_matches(&r.path, path))
}
