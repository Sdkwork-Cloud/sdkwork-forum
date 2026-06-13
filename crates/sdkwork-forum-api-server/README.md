# SDKWork Forum API Server

API server composition for all 3 forum surfaces (app-api, backend-api, open-api).

## Implementation Status

- `compose_forum_api_routes()`: Returns `Vec<ForumRouteInfo>` aggregating 66 routes across 3 surfaces.
- `ForumRouteInfo`: surface, method, path, operation_id, auth_mode.
- `route_count()`, `route_count_by_surface()`, `find_route()` convenience functions.

Awaiting SDKWork appbase middleware, observability, and graceful shutdown integration.
