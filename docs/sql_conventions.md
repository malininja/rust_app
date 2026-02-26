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
