# Task: frontend — walking skeleton status page
Status: assigned        <!-- assigned | in-progress | done -->

## Goal
Smallest server-rendered Rust frontend that shows the backend's health, plus a real pipeline.

## Acceptance criteria
- `frontend/` is a Cargo package named `frontend` (Rust 1.100, edition 2021, `Cargo.lock` committed, rustls only).
- `GET /` renders HTML with `<span id="backend-status">ok</span>` when backend health is OK, `unavailable` otherwise (backend down, non-200, bad JSON, timeout 2s). Page is always 200.
- `GET /health` and 404 per contract. Binds `0.0.0.0:$PORT`, default 3000; reads `API_URL`.
- Page basics: `<!doctype html>`, `lang="en"`, `<title>`, readable status text (not color-only).
- Tests cover `ok` (against an in-test fake backend started on an ephemeral port) and `unavailable` (backend unreachable), plus `/health`.
- `frontend/Makefile` implements `lint test build check run` per contract; `make -C frontend check` and `make -C frontend build` pass.
- `.github/workflows/frontend.yml` adds the contract's toolchain step before `make`.
- `frontend/STATUS.md` updated, `Ready for integration: yes`.

## Contracts
Toolchain, Make targets, Backend service (consumer), Frontend service.

## Out of scope
JS/WASM client, styling beyond basics, Docker (infra), e2e tests (qa).
