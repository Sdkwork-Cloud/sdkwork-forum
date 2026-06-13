# Forum Database Design

## Design Baseline

The authoritative machine-readable contract is `specs/forum-database.schema.yaml`.

Database naming follows `DATABASE_SPEC.md`:
- Table prefix is `forum_`.
- Multi-tenant tables use `tenant_id`, `organization_id`, `data_scope`, `status`, `version`, `created_at`, and `updated_at`.
- Stable external ids use `uuid`.
- Soft delete uses `deleted_at` and `deleted_by`.
- Event consistency uses `forum_outbox_event`, `forum_inbox_event`, and `forum_idempotency_record`.

The domain vocabulary is `topic` and `reply`. The rejected ambiguous term is documented in ADR-0001 only.

## Table Groups

Taxonomy:
- `forum_space`: top-level forum area.
- `forum_node`: category/board tree.
- `forum_board_profile`: board behavior and rules.
- `forum_tag`, `forum_topic_tag`: reusable tags and topic relations.
- `forum_topic_prefix`: board-local labels.
- `forum_node_acl`: forum-specific ACL overrides over IAM principals.

Discussion:
- `forum_topic`: source-of-truth topic.
- `forum_topic_revision`: immutable topic edit history.
- `forum_topic_reply`: source-of-truth reply.
- `forum_reply_revision`: immutable reply edit history.
- `forum_attachment`: Drive/media references bound to topics or replies.

Q&A and poll:
- `forum_question_profile`: accepted answer and bounty state.
- `forum_poll`, `forum_poll_option`, `forum_poll_vote`: poll state and votes.

Engagement:
- `forum_reaction`, `forum_vote`, `forum_bookmark`, `forum_subscription`, `forum_read_state`, `forum_notification_preference`.

Member and reputation:
- `forum_member_profile`, `forum_trust_level`, `forum_privilege_grant`.
- `forum_badge`, `forum_user_badge`.
- `forum_reputation_rule`, `forum_reputation_ledger`.

Moderation:
- `forum_report`, `forum_moderation_queue_item`, `forum_moderation_case`, `forum_moderation_decision`, `forum_moderation_policy`, `forum_sanction`, `forum_appeal`.

Projection:
- `forum_feed_item`, `forum_public_topic_projection`, `forum_topic_stats`, `forum_board_stats`, `forum_member_stats`, `forum_search_document`.

Integration:
- `forum_outbox_event`, `forum_inbox_event`, `forum_idempotency_record`.

## Core Query Patterns

- Board topic list: `forum_topic(tenant_id, board_id, moderation_status, last_activity_at, id)`.
- Topic replies: `forum_topic_reply(tenant_id, topic_id, moderation_status, created_at, id)`.
- Public Open API list: `forum_public_topic_projection(tenant_id, site_slug, status, updated_at, id)`.
- Moderation queue: `forum_moderation_queue_item(tenant_id, queue_status, severity, due_at, id)`.
- Search staging: `forum_search_document(tenant_id, index_status, updated_at, id)`.
- Outbox polling: `forum_outbox_event(status, next_attempt_at, id)`.

## Ownership Boundaries

IAM/appbase owns users, tenants, organizations, sessions, roles, and API keys. Forum stores stable ids and never duplicates IAM source tables.

Drive owns file bytes, upload sessions, download grants, and media lifecycle. Forum stores Drive/media references in `forum_attachment`, `forum_space`, `forum_node`, `forum_member_profile`, and `forum_badge`.

Search owns the external index. Forum owns the staging projection `forum_search_document`.

Notification providers own delivery. Forum owns subscription and preference state plus outbox events.

## Implementation Priorities

1. Generate migrations for taxonomy, discussion, integration, and projections needed by topic/reply list/create.
2. Implement idempotency and outbox transaction helpers before write endpoints.
3. Implement revisions for topic/reply update before exposing update APIs.
4. Implement moderation queue and decisions before automated policy rules.
5. Implement stats/search rebuild jobs after core write paths are stable.

## DDL Generation

SQL DDL snapshots will be generated from `specs/forum-database.schema.yaml` when the schema generator tool is connected.

### Planned DDL Targets

- **PostgreSQL** (primary): `deployments/sql/postgresql/V0.1.0__forum_foundation.sql`
- **MySQL** (secondary): `deployments/sql/mysql/V0.1.0__forum_foundation.sql`
- **SQLite** (development): `deployments/sql/sqlite/V0.1.0__forum_foundation.sql`

### Schema Contract Summary

- 45 tables across 8 groups
- Standard field sets: tenant_entity (11 fields), integration_log (8 fields)
- 4 required indexes minimum per tenant table
- Unique constraints on uuid, business keys, and tenant-scoped identifiers
- Soft delete via deleted_at/deleted_by on all tenant_entity tables
