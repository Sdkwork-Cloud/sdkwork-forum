# Forum Integration Roadmap

## Foundation Dependencies

Current metadata avoids declaring unresolved SDK dependencies as generation inputs. The following integrations must be added once their local SDK family paths and authority contracts are verified:

- Appbase app SDK: login/session/current user/workspace context for app clients.
- Appbase backend SDK: backend-admin IAM and permission management.
- Drive app/backend SDKs: attachment upload sessions, media resource selection, and download grants.
- Search backend SDK: full reindex and incremental indexing adapters.
- Messaging or notification SDK: subscription and moderation notification delivery.

## Integration Status

### SDK Dependencies

Planned `sdkDependencies` entries for each SDK family:

```yaml
# sdkwork-forum-app-sdk
sdkDependencies:
  - sdkFamily: sdkwork-appbase-app-sdk
    authority: sdkwork-appbase-app-api
    purpose: IAM login, session, current user context
  - sdkFamily: sdkwork-drive-app-sdk
    authority: sdkwork-drive-app-api
    purpose: Attachment upload sessions and media resource selection

# sdkwork-forum-backend-sdk
sdkDependencies:
  - sdkFamily: sdkwork-appbase-backend-sdk
    authority: sdkwork-appbase-backend-api
    purpose: IAM permission management and operator context
  - sdkFamily: sdkwork-drive-backend-sdk
    authority: sdkwork-drive-backend-api
    purpose: Download grants and media lifecycle management
  - sdkFamily: sdkwork-search-backend-sdk
    authority: sdkwork-search-backend-api
    purpose: Full reindex and incremental indexing

# sdkwork-forum-sdk (open)
sdkDependencies: []
# Open API is anonymous public reads only; no external SDK dependencies required.
```

### Service Ports (Implemented)

| Port | Trait | Status | Implementations |
|------|-------|--------|-----------------|
| Drive | `ForumDrivePort` | Implemented | `NoopForumDrivePort` (silent), `LoggingForumDrivePort` (stderr) |
| Search | `ForumSearchPort` | Implemented | `NoopForumSearchPort` (silent), `LoggingForumSearchPort` (stderr) |
| Notification | `ForumNotificationPort` | Implemented | `NoopForumNotificationPort` (silent), `LoggingForumNotificationPort` (stderr) |

### Drive Media Grants

`ForumDrivePort` provides:
- `validate_media_reference(media_resource_id)` - Verify tenant scope, ownership, scan status, and lifecycle
- `create_download_grant(media_resource_id)` - Create scoped download grant for attachments

Awaiting `sdkwork-drive-app-sdk` dependency resolution for real implementation.

### Notification Event Publisher

`ForumNotificationPort` provides:
- `publish_forum_event(event_type, aggregate_id)` - Generic forum event publication
- `publish_moderation_alert(case_id, severity)` - Moderation alert to operators
- `publish_subscription_notification(user_id, event_type, target_id)` - Subscription delivery

Awaiting `sdkwork-messaging-sdk` dependency resolution for real implementation.

### Search Indexing Adapter

`ForumSearchPort` provides:
- `index_document(source_type, source_id)` - Index topic/reply after projection refresh
- `delete_document(source_type, source_id)` - Remove hidden/deleted content from index
- `rebuild_index(board_id)` - Full or scoped reindex

Awaiting `sdkwork-search-backend-sdk` dependency resolution for real implementation.

### Appbase Permission Mapping

Forum permission codes planned:

| Code | Description | Surface |
|------|-------------|---------|
| `forum.topics.create` | Create topics | app-api |
| `forum.topics.read` | Read topics | app-api, open-api |
| `forum.topics.update` | Update own topics | app-api |
| `forum.topics.delete` | Delete own topics | app-api |
| `forum.replies.create` | Create replies | app-api |
| `forum.replies.read` | Read replies | app-api, open-api |
| `forum.replies.update` | Update own replies | app-api |
| `forum.replies.delete` | Delete own replies | app-api |
| `forum.moderation.read` | Read moderation queue | backend-api |
| `forum.moderation.write` | Create decisions | backend-api |
| `forum.admin.nodes` | Manage taxonomy | backend-api |
| `forum.admin.reputation` | Manage reputation rules | backend-api |
| `forum.admin.badges` | Manage badges | backend-api |

Awaiting `sdkwork-appbase-backend-sdk` dependency resolution for IAM integration.
