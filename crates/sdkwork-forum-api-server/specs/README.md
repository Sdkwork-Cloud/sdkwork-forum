# Server Specs

Component spec for `sdkwork-forum-api-server`.

- **Crate type**: HTTP API server process
- **Domain**: communication
- **Capability**: forum
- **Surfaces**: app-api (22 routes), backend-api (36 routes), open-api (8 routes)
- **Total routes**: 66
- **Public exports**: compose_forum_api_routes(), ForumRouteInfo, route_count(), find_route()
- **Dependencies**: sdkwork-router-forum-app-api, sdkwork-router-forum-backend-api, sdkwork-router-forum-open-api
