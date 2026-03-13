# HTTP Conventions

## Handler Signature

All handlers are async functions with this signature:

```
pub async fn <name>(State(pool): State<PgPool>) -> Result<impl IntoResponse, StatusCode>
```

- State is extracted via `State(pool): State<PgPool>`.
- The pool is passed directly to the repository constructor — it is not stored on a custom app state struct.
- Return type is always `Result<impl IntoResponse, StatusCode>`.

**Reference:** `src/roles/role_handler.rs`

## Response Conventions

| Outcome | Return value |
|---------|-------------|
| Success with body (200) | `Ok(Json(data))` — Serde serializes the payload |
| Created (201) | `Ok((StatusCode::CREATED, Json(data)))` — used for POST/create operations |
| Success without body (204) | `Ok((StatusCode::NO_CONTENT, ()))` — used for delete/undelete operations |
| Not found | `Err(StatusCode::NOT_FOUND)` — when a specific entity error variant indicates the resource does not exist |
| Error | `Err(StatusCode::INTERNAL_SERVER_ERROR)` — no response body |

Do not construct custom error response bodies. Error detail goes to the structured log, not the HTTP response.

When matching on service errors, check for specific variants (e.g. `UserNotFound`) before logging — expected business conditions like not found should not be logged as errors.

## Router Construction

Each feature module exposes a `router()` function that returns `Router<PgPool>`:

```
pub fn router() -> Router<PgPool> {
    Router::new().route("/", get(super::<entity>_handler::<handler>))
}
```

Feature routers are registered in `lib.rs` via `.nest(prefix, <module>::<entity>_router::router())`.

**Reference:** `src/roles/role_router.rs`

## App Setup (`lib.rs`)

State and middleware are bound once at the top level:

```
Router::new()
    .nest("/prefix", module::entity_router::router())
    // ...additional routes...
    .with_state(pool)
    .layer(TraceLayer::new_for_http())
```

- `.with_state(pool)` is called once on the root router.
- `TraceLayer::new_for_http()` from `tower-http` provides automatic request/response logging — do not add manual request logging in handlers.

**Reference:** `src/lib.rs`

---

## Conventions Enforcement

If a suggested implementation deviates from these conventions, Claude must explicitly flag the deviation and ask: **"Is this an intentional exception, or should the conventions be updated to reflect this approach?"**
