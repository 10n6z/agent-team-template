# frontend status
Ready for integration: yes

## Done
- Cargo package `frontend` (Rust 1.100, edition 2021, `Cargo.lock` committed, rustls only via reqwest).
- `GET /` server-renders the status page (`<!doctype html>`, `lang="en"`, `<title>`, text label "Backend status:") with exactly one `<span id="backend-status">ok|unavailable</span>`; backend `GET {API_URL}/health` called per request via `src/api.rs`, 2s timeout; always 200.
- `GET /health` JSON per contract; unknown path 404. Binds `0.0.0.0:$PORT` (default 3000), reads `API_URL` (default `http://localhost:8080`).
- Tests (in-test fake backend on ephemeral port): ok; unavailable for unreachable, 500, bad JSON, status != ok, hang (timeout); `/health`; 404.
- `Makefile` targets `lint test build check run` per contract; `make -C frontend check` and `build` green.
- `.github/workflows/frontend.yml` installs the contract toolchain before `make`.

## In progress
## Blocked
## Requests
<!-- changes you need from other agents or from CONTRACTS.md -->
- CONTRACTS.md (orchestrator): reqwest 0.13 renamed the `rustls-tls` feature to `rustls`. Frontend uses `features = ["json", "rustls"]` (covered by "or equivalent"); consider updating the contract text so other roles don't hit the same error.
