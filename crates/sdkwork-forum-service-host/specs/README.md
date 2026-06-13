# Host Specs

Component spec for `sdkwork-forum-service-host`.

- **Crate type**: In-process service host
- **Domain**: communication
- **Capability**: forum
- **Public exports**: ForumServiceHost, build_forum_service()
- **Dependencies**: sdkwork-communication-forum-repository-sqlx, sdkwork-communication-forum-service
- **Implementation**: ForumServiceHost wraps ForumService with placeholder repository.
