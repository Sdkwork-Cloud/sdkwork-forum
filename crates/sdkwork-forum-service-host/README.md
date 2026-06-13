# SDKWork Forum Service Host

Dependency composition for forum service runtime.

## Implementation Status

- `ForumServiceHost`: Wraps `ForumService<SqlxForumRepository>` with `service()` accessor and `build_request_context()` factory.
- `build_forum_service()`: Constructs `ForumService<SqlxForumRepository>` with placeholder repository.

Awaiting SQLx pool injection, Drive/Search/Notification adapter wiring, and appbase context integration.
