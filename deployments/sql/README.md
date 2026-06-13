# SQL Deployments

SQL migrations will be generated from `specs/forum-database.schema.yaml`.

## Status

Database schema contract is defined in YAML. SQL migrations require the schema generator tool.

## Schema Contract

- 45 tables across 8 groups
- Full constraint and index definitions
- See `specs/forum-database.schema.yaml` for complete table contracts

## Planned DDL Targets

- PostgreSQL (primary)
- MySQL (secondary)
- SQLite (development/testing)
