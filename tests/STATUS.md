# qa status
Ready for integration: yes

## Done
- `tests/` is Cargo package `e2e` (edition 2021, reqwest 0.12 blocking + rustls, serde_json; no OpenSSL in `Cargo.lock`).
- Helpers in `src/lib.rs`: `API_URL` / `FRONTEND_URL` defaulting (blank = unset, trailing `/` trimmed), `backend-status` count + text extraction; 7 non-ignored unit tests.
- `#[ignore]`d contract v1 e2e tests:
  - `backend_health_returns_ok_json_with_exact_keys`
  - `backend_unknown_path_returns_404_not_found_error`
  - `frontend_health_returns_ok_json_with_exact_keys`
  - `frontend_index_shows_backend_status_ok_when_stack_is_up`
  - `frontend_unknown_path_returns_404`
- `tests/Makefile`: `lint test build check` per contract, `e2e` = `cargo test -- --ignored`. `check` and `build` green; `e2e` without a stack fails cleanly with connection refused.
- `.github/workflows/qa.yml`: contract toolchain step before `make`.

## In progress
## Blocked
## Requests
<!-- changes you need from other agents or from CONTRACTS.md -->
- CONTRACTS (orchestrator): frontend `GET /health` says "200 JSON" without a Content-Type. Tests assert it starts with `application/json`, same as backend. Please state it explicitly.
- CONTRACTS (orchestrator): frontend unknown path is "404" with no body/content-type. Tests check status only. Say whether frontend 404s also use `{"error":"not_found"}`.
- CONTRACTS (orchestrator): `backend-status` VALUE is matched exactly (no whitespace trimming) and the attribute is matched as `id="backend-status"` (double quotes). Confirm this is the intended strictness.
- CONTRACTS (orchestrator): `cargo test -- --ignored` stops at the first failing test binary, so frontend e2e results are hidden when a backend e2e test fails. Consider `cargo test --no-fail-fast -- --ignored` for `e2e`.
- Not covered by e2e: frontend showing `unavailable` when backend is down (would need stopping a service mid-suite). Frontend's own tests cover it per its task.
