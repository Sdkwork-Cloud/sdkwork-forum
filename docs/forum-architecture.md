# Forum Architecture

## Scope

`sdkwork-forum` provides the forum foundation for the SDKWork `communication` domain. This phase defines contracts and module boundaries only; frontend app work is handled outside this repository phase.

## Bounded Context

The forum bounded context owns:
- Spaces, boards, nodes, tags, and topic prefixes.
- Topics, replies, revisions, attachments, polls, and Q&A acceptance.
- Reactions, votes, bookmarks, subscriptions, read state, and notification preferences.
- Member profiles, trust levels, badges, reputation rules, and reputation ledger entries.
- Reports, moderation queue items, cases, decisions, sanctions, appeals, and policies.
- Feed, public topic, statistics, and search projections.
- Outbox, inbox, and idempotency records for integration reliability.

The forum bounded context does not own:
- Login, sessions, tenants, users, roles, or API keys; use appbase/IAM.
- Binary file storage; use Drive and store stable media references.
- Frontend apps; consume generated SDKs from `sdks/`.

## Surfaces

- App API: authenticated app clients, `/app/v3/api/forum`.
- Backend API: backend-admin and operator automation, `/backend/v3/api/forum`.
- Open API: external/public read integration, `/forum/v3/api`.

Open API is intentionally simple. Anonymous public reads declare `security: []` and no custom business headers. If a future write integration is required, it must use one external API key scheme and remain mutually exclusive with dual-token mode.

## Naming

Use `topic` and `reply` in all public contracts. The rejected word is documented in `docs/adr/ADR-0001-forum-topic-reply-naming.md`.

## Implementation Layers

- Route crates define path descriptors, route manifests, and contract-test handler placeholders.
- API server crate mounts live handlers in `src/routes/` for app, backend, and open surfaces.
- Service crate owns domain orchestration and service ports.
- Repository crate owns SQL persistence adapters via `SqlxForumRepository`.
- Service host crate wires PostgreSQL pool, database module, and ops service.
- Worker crate processes outbox, search projection, stats, and moderation jobs.

## Runtime Dependency Wiring

```
┌─────────────────────────────────────────────────────────────┐
│                    sdkwork-api-forum-standalone-gateway                  │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐│
│  │ app-api      │ │ backend-api  │ │ open-api             ││
│  │ router crate │ │ router crate │ │ router crate         ││
│  └──────┬───────┘ └──────┬───────┘ └──────┬───────────────┘│
│         └────────────────┼────────────────┘                 │
│                          │                                  │
│  ┌───────────────────────┴────────────────────────────────┐ │
│  │              sdkwork-forum-service-host                 │ │
│  │  ┌───────────────────────────────────────────────────┐ │ │
│  │  │         sdkwork-communication-forum-service        │ │ │
│  │  │  ForumService<R: ForumRepository>                  │ │ │
│  │  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ │ │ │
│  │  │  │ ForumRepo   │ │ DrivePort   │ │ SearchPort  │ │ │ │
│  │  │  │ trait       │ │ trait       │ │ trait       │ │ │ │
│  │  │  └──────┬──────┘ └──────┬──────┘ └──────┬──────┘ │ │ │
│  │  └─────────┼───────────────┼───────────────┼────────┘ │ │
│  │            │               │               │          │ │
│  │  ┌─────────┴───────────┐   │               │          │ │
│  │  │ SqlxForumRepository │   │               │          │ │
│  │  │ (SQLx pool)         │   │               │          │ │
│  │  └─────────────────────┘   │               │          │ │
│  └────────────────────────────┼───────────────┼──────────┘ │
│                               │               │            │
│  ┌────────────────────────────┴───────────────┴──────────┐ │
│  │              sdkwork-communication-forum-worker        │ │
│  │  ForumWorker<R> ──► ForumService ──► Repository       │ │
│  └───────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘

External Dependencies (awaiting SDK generation):
  Drive SDK  ◄── ForumDrivePort
  Search SDK ◄── ForumSearchPort
  Messaging  ◄── ForumNotificationPort
  Appbase    ◄── ForumRequestContext (tenant/user/permissions)
```

## Adapter Status

| Adapter | Port Trait | Implementation | Status |
|---------|-----------|----------------|--------|
| Repository | `ForumRepository` | `SqlxForumRepository` | Implemented (PostgreSQL + snowflake ids) |
| Drive | `ForumDrivePort` | `NoopForumDrivePort` / logging | Implemented (validate on attachment) |
| Search | `ForumSearchPort` | `HttpForumSearchPort` / logging / noop | Implemented |
| Notification | `ForumNotificationPort` | `HttpForumNotificationPort` / logging / noop | Implemented |
| Request Context | `ForumRequestContext` | Typed struct | Implemented |
