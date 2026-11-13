# Task: qa — walking skeleton e2e suite
Status: assigned        <!-- assigned | in-progress | done -->

## Goal
End-to-end tests that prove the deployed stack honors contract v1, written from the contract, not from the code.

## Acceptance criteria
- `tests/` is a Cargo package named `e2e` (Rust 1.100, edition 2021, `Cargo.lock` committed, rustls only).
- `#[ignore]`d e2e tests, reading `API_URL` / `FRONTEND_URL` per contract:
  - backend `/health`: 200, `application/json`, exact keys/values for `status` and `service`, non-empty `version`.
  - backend unknown path: 404 with `{"error":"not_found"}`.
  - frontend `/health`: 200 + exact `status`/`service`.
  - frontend `/`: 200, `text/html`, exactly one `id="backend-status"` element whose text is `ok` (stack up).
  - frontend unknown path: 404.
- Non-ignored unit tests for any helper logic (e.g. URL defaulting) so `make -C tests check` runs something real.
- `tests/Makefile` implements `lint test build check` per contract plus `e2e` = `cargo test -- --ignored`; `make -C tests check` passes with no services running.
- `.github/workflows/qa.yml` adds the contract's toolchain step before `make`.
- `tests/STATUS.md` updated, `Ready for integration: yes`.

## Contracts
Toolchain, Make targets, Backend service, Frontend service, Tests.

## Out of scope
Load tests, browser automation, fixing product code (report defects as Requests).
