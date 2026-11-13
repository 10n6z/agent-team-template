# backend status
Ready for integration: yes

## Done
- Cargo package `backend` (Rust 1.100, edition 2021, `Cargo.lock` committed); axum 0.8 + tokio.
- `GET /health` → 200 `application/json` `{"status":"ok","service":"backend","version":"<CARGO_PKG_VERSION>"}`.
- Unknown path → 404 `{"error":"not_found"}`.
- Binds `0.0.0.0:$PORT`, default 8080.
- In-process tests (`tower::ServiceExt::oneshot`) for /health status + exact JSON + content-type, and 404 shape.
- `backend/Makefile`: `lint test build check run` per contract; `check` and `build` green.
- `.github/workflows/backend.yml`: contract toolchain step before `make`.

## In progress
## Blocked
## Requests
<!-- changes you need from other agents or from CONTRACTS.md -->
- Contract clarification (non-blocking): wrong method on a known path (e.g. `POST /health`) currently returns axum's default 405 with an empty body. Contract only covers unknown paths; if 405 should also use `{"error":"method_not_allowed"}`, add it to CONTRACTS.md.
