# Server Specs

Component spec for `sdkwork-forum-standalone-gateway`.

- **Crate type**: HTTP API server process
- **Domain**: communication
- **Capability**: forum
- **Surfaces**: app-api (22 routes), backend-api (36 routes), open-api (8 routes)
- **Total routes**: 66
- **Public exports**: compose_forum_api_routes(), ForumRouteInfo, route_count(), find_route()
- **Dependencies**: sdkwork-routes-forum-app-api, sdkwork-routes-forum-backend-api, sdkwork-routes-forum-open-api
