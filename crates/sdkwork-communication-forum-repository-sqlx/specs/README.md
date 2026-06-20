# Repository Specs

Component spec for `sdkwork-communication-forum-repository-sqlx`.

- **Crate type**: SQLx repository implementation
- **Domain**: communication
- **Capability**: forum
- **Tables**: 46 tables across 8 groups (taxonomy, discussion, qa_poll, engagement, member, moderation, projection, integration)
- **Schema registry**: `specs/forum-database.schema.yaml`
- **Implementation**: `SqlxForumRepository` implements all `ForumRepository` methods against PostgreSQL with snowflake ids via `sdkwork-id-core`.
