# SDKWork Forum

`sdkwork-forum` is the SDKWork forum foundation under the canonical `communication` domain and `forum` capability.

This repository root currently owns backend/domain contracts only. Frontend application work under `apps/` is intentionally out of scope for this phase.

## Active Layout

- `apis/`: authored OpenAPI contracts and validation fixtures.
- `specs/`: component and database schema registry contracts.
- `sdks/`: SDK family workspaces, materialized OpenAPI authority files, route manifests, and generator inputs.
- `crates/`: Rust service, repository, route, host, server, and worker crate skeletons.
- `jobs/`: queue, schedule, batch, and runbook definitions.
- `tools/`: materializers and validators.
- `docs/`: architecture, database, API, ADR, runbook, and implementation planning documents.
- `tests/`: static, schema, SDK, and contract tests.

## Naming Contract

Forum public contracts use `topic` and `reply`.

The term `thread` is not used for forum resources because it collides with runtime and concurrency terminology. Route crates may still use `forum` as the capability name.

## API Surfaces

- App API: `/app/v3/api/forum`, generated into `sdkwork-forum-app-sdk`.
- Backend API: `/backend/v3/api/forum`, generated into `sdkwork-forum-backend-sdk`.
- Open API: `/forum/v3/api`, generated into `sdkwork-forum-sdk`.

Open API public reads are anonymous and do not require SDKWork dual-token headers or business context headers.

## Verification

Run:

```bash
node tests/static/forum-contract-boundary.test.mjs
node tools/validators/validate_forum_contracts.mjs
```
