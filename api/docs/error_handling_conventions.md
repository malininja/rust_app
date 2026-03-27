# Error Handling Conventions

## Domain Error Types

All domain errors are defined as enums using the `thiserror` crate:

```
#[derive(thiserror::Error, Debug)]
pub enum <Entity>Error {
    #[error("Human-readable description of the error")]
    VariantName,
}
```

- Derive both `thiserror::Error` and `Debug`.
- Name the enum `<Entity>Error` (e.g., `RoleError`).
- Every variant must have a `#[error("...")]` attribute with a human-readable message.

**Reference:** `src/roles/role_error.rs`

## Error Propagation

Services return `Result<T, <Entity>Error>`. Repository errors are mapped into the domain error enum at the service boundary — raw `sqlx` or other library errors do not escape the repository layer.

## Mapping Errors to HTTP Responses

Handlers are responsible for mapping domain errors to HTTP status codes. The pattern is:

```
match service::operation(repo).await {
    Ok(data) => Ok(Json(data)),
    Err(e) => {
        tracing::error!("<module>_handler: <operation> error: {}", e);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
```

- Always log the error with `tracing::error!` before discarding it.
- Return `Err(StatusCode::...)` — no response body on error.
- The handler return type is `Result<impl IntoResponse, StatusCode>`.

**Reference:** `src/roles/role_handler.rs`

---

## Conventions Enforcement

If a suggested implementation deviates from these conventions, Claude must explicitly flag the deviation and ask: **"Is this an intentional exception, or should the conventions be updated to reflect this approach?"**
