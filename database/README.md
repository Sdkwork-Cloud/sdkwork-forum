# SDKWork Forum Database Module

Owner: forum-platform  
Service code: `FORUM` (`SDKWORK_FORUM_DATABASE_*`)  
Engines: PostgreSQL  
Baseline strategy: `baseline-plus-migrations`

## Commands

Run from repository root:

```bash
pnpm db:validate
pnpm db:init
pnpm db:migrate
pnpm db:seed
pnpm db:status
pnpm db:drift
pnpm db:drift:check
pnpm db:bootstrap
```

## Standards

- `../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`
- `../sdkwork-specs/DATABASE_SPEC.md`
- Authoritative domain contract: `../specs/forum-database.schema.yaml` (mirrored to `contract/schema.yaml`)

## Baseline

Production schema baseline lives in `ddl/baseline/postgres/0001_forum_baseline.sql`, sourced from `deployments/sql/forum-ddl-postgresql.sql`.

Incremental changes MUST be added under `migrations/postgres/` using sortable `{version}_{name}.up.sql` names.
