# Architecture Conventions

## Layered Module Structure

Each feature module follows a strict layered architecture. Layers only depend on layers below them — handlers call services, services call repositories, repositories call the database.

```
model → repository → service → handler → router
```

## File Naming

Each layer lives in its own file, named `<entity>_<layer>.rs`:

| File | Responsibility |
|------|----------------|
| `<entity>_model.rs` | Data types (structs, derives for Serde/SQLx) |
| `<entity>_repository.rs` | Database access; trait definition + `Pg` implementation |
| `<entity>_service.rs` | Business logic; accepts a repository, returns domain types or errors |
| `<entity>_handler.rs` | HTTP handler functions; calls service, maps errors to status codes |
| `<entity>_router.rs` | Wires routes to handlers; returns `Router<PgPool>` |
| `<entity>_error.rs` | Domain error enum using `thiserror` |

**Reference:** `src/roles/` — canonical example of this structure.

## `mod.rs` Structure

Each module's `mod.rs` declares and re-exports all submodules:

```
pub mod <entity>_error;
pub mod <entity>_handler;
pub mod <entity>_model;
pub mod <entity>_repository;
pub mod <entity>_router;
pub mod <entity>_service;
#[cfg(test)]
mod tests;
```

**Reference:** `src/roles/mod.rs`

## Test Layout

Unit tests live in `src/<module>/tests/` with:
- `mod.rs` — declares test submodules
- `<entity>_test.rs` — test file per layer being tested

Integration/E2E tests live in the top-level `tests/` directory as a separate crate.

**Reference:** `src/roles/tests/`

---

## Conventions Enforcement

If a suggested implementation deviates from these conventions, Claude must explicitly flag the deviation and ask: **"Is this an intentional exception, or should the conventions be updated to reflect this approach?"**
