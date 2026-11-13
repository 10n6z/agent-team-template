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
- `tests/Makefile`: `lint test build check` per contract, `e2e` = `cargo test --no-fail-fast -- --ignored`. `check` and `build` green; `e2e` without a stack fails cleanly with connection refused.
- `.github/workflows/qa.yml`: contract toolchain step before `make`.
- Contract 1.1 applied: `e2e` = `cargo test --no-fail-fast -- --ignored`.
- Contract 1.1 answered all Requests (no test changes needed):
  - frontend `/health` is `application/json`; assertion kept.
  - frontend 404 body unspecified; status-only check kept.
  - span matched literally as `id="backend-status"`, text exactly `ok`; current strictness is correct.
  - `--no-fail-fast` adopted for `e2e`.
  - `unavailable` not covered e2e; agreed, frontend unit tests cover it.
- reqwest stays 0.12 (`rustls-tls`); `Cargo.lock` has no `openssl-sys` / `native-tls`.

## In progress
## Blocked
## Requests
<!-- changes you need from other agents or from CONTRACTS.md -->
