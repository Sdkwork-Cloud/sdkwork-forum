# FORUM Database Module

Canonical lifecycle assets for `sdkwork-forum` per `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `forum`
- serviceCode: `FORUM`
- tablePrefix: `forum_`

## Commands

```bash
pnpm run db:validate
pnpm run db:materialize:contract
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```

## Contract sources

- Semantic schema registry: `specs/forum-database.schema.yaml` (authoritative for table design)
- Framework contract: `database/contract/*` (materialized from baseline via `db:materialize:contract`)
- Baseline DDL: `database/ddl/baseline/postgres/0001_forum_baseline.sql`

## Runtime

PostgreSQL services MUST bootstrap through `sdkwork-forum-database-host`:

- `bootstrap_forum_database()` / `bootstrap_forum_database_from_env()`
- Repository re-exports: `sdkwork-communication-forum-repository-sqlx::bootstrap`
