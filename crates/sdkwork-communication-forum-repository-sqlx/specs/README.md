# Repository Specs

Component spec for `sdkwork-communication-forum-repository-sqlx`.

- **Crate type**: SQLx repository implementation
- **Domain**: communication
- **Capability**: forum
- **Tables**: 45 tables across 8 groups (taxonomy, discussion, qa_poll, engagement, member, moderation, projection, integration)
- **Schema registry**: `specs/forum-database.schema.yaml`
- **Implementation**: All repository methods return `ForumServiceError::not_implemented` pending SQLx pool injection.
