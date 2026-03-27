# HTTP Conventions

## Handler Signature

All handlers are async functions with this signature:

```
pub async fn <name>(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode>
```

- State is extracted via `State(state): State<AppState>`.
- The pool and any other shared state is accessed via fields on `AppState`.
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

Each feature module exposes a `router(app_state: AppState)` function that returns `Router<AppState>`:

```
pub fn router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(super::<entity>_handler::<handler>))
        .layer(from_fn_with_state(app_state.clone(), <middleware>))
}
```

- Middleware is applied per-router using `from_fn_with_state`, giving fine-grained control over which routes are protected and by which middleware.
- `AppState` is passed in so the router can bind middleware that requires access to shared state (e.g. JWT secret for auth).

Feature routers are registered in `lib.rs` via `.nest(prefix, <module>::<entity>_router::router(app_state.clone()))`.

**Reference:** `src/roles/role_router.rs`, `src/articles/article_router.rs`

## App Setup (`lib.rs`)

State is bound once at the top level; middleware is applied per-router, not globally:

```
Router::new()
    .nest("/prefix", module::entity_router::router(app_state.clone()))
    // ...additional routes...
    .route("/", get(handler))
    .with_state(app_state)
    .layer(TraceLayer::new_for_http())
```

- `.with_state(app_state)` is called once on the root router.
- `TraceLayer::new_for_http()` from `tower-http` provides automatic request/response logging — do not add manual request logging in handlers.
- Do not apply auth middleware globally at the root level; use per-router layers instead.

**Reference:** `src/lib.rs`

---

## Conventions Enforcement

If a suggested implementation deviates from these conventions, Claude must explicitly flag the deviation and ask: **"Is this an intentional exception, or should the conventions be updated to reflect this approach?"**
