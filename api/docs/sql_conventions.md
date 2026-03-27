# SQL Conventions

## Timestamp Columns

All new tables must include standard audit timestamp columns:

```
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
```

`updated_at` must be kept current automatically via a `BEFORE UPDATE` trigger bound to the shared `on_update_timestamp()` trigger function:

```
CREATE TRIGGER <table>_updated_at
BEFORE UPDATE ON <table>
FOR EACH ROW
EXECUTE FUNCTION on_update_timestamp();
```

The `on_update_timestamp()` function is a project-wide shared function and should be created once (via `CREATE OR REPLACE FUNCTION`) in an early migration. It must not be recreated per-table.

**Reference:** `migrations/20260226141851_roles_timestamps.up.sql`

## Soft Deletes

For tables that explicitly require soft delete behaviour, add a nullable `deleted_at` column:

```
deleted_at TIMESTAMPTZ NULL
```

- No default value — the column is `NULL` until the row is soft-deleted, at which point it is set to the deletion timestamp.
- No trigger is needed (unlike `updated_at`).
- All queries against soft-deletable tables must exclude deleted rows with `WHERE deleted_at IS NULL`.
- Do **not** add this column to every table — only tables that have a specific requirement for soft deletes.
- Tables with `deleted_at` must **never** use hard deletes (`DELETE FROM`). Removal is always done by setting `deleted_at` to the current timestamp.
