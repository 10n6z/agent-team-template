# Task: backend — walking skeleton health service
Status: assigned        <!-- assigned | in-progress | done -->

## Goal
Smallest real Rust backend that satisfies the contract, plus a real pipeline.

## Acceptance criteria
- `backend/` is a Cargo package named `backend` (Rust 1.100, edition 2021, `Cargo.lock` committed).
- `GET /health` returns exactly the contract body/status/content-type; unknown path returns 404 `{"error":"not_found"}`.
- Binds `0.0.0.0:$PORT`, default 8080.
- In-process tests (no network listener needed) cover: `/health` status + exact JSON + content-type, and the 404 shape.
- `backend/Makefile` implements `lint test build check run` per contract; `make -C backend check` and `make -C backend build` pass.
- `.github/workflows/backend.yml` adds the contract's toolchain step before `make`.
- `backend/STATUS.md` updated, `Ready for integration: yes`.

## Contracts
Toolchain, Make targets, Backend service.

## Out of scope
Database, auth, Docker (infra owns Dockerfiles), e2e tests (qa).
