# SDKWork Router Forum App API

Route crate for `/app/v3/api/forum` with 22 routes.

## Implementation Status

- **Route descriptors**: 22 `RouteDescriptor` entries with method, path, operationId, surface, auth_mode, and tags.
- **Handlers**: Per-operation handler functions that return `ForumRouteError::not_implemented`.
- **Mappers**: Request query param parsing, JSON response helpers, problem+json error mapping.
- **Path matching**: `find_route()` with template-based path matching for `{param}` segments.
- **Manifest**: `ManifestMetadata` with schemaVersion, kind, packageName, surface, owner, domain, capability, apiAuthority, sdkFamily, prefix.

Awaiting SDKWork Rust HTTP runtime selection for real Axum/router bindings.
