# Task: infra — v1.1 runtime image fix
Status: done            <!-- assigned | in-progress | done -->

## Goal
Fix the integration failure: on main, `make -C infra up` times out because the frontend container crashes with
`frontend: /lib/aarch64-linux-gnu/libc.so.6: version 'GLIBC_2.38' not found`.
Root cause (contract 1.0 bug, not yours): build `rust:1.100` is Debian trixie (glibc 2.41), runtime `debian:bookworm-slim` is glibc 2.36.

## Acceptance criteria
- Both Dockerfiles: build `FROM rust:1.100-trixie`, runtime `FROM debian:trixie-slim` (CONTRACTS.md 1.1 Runtime). Nothing else changes.
- First `git merge main` (main now contains the real backend + frontend + tests).
- `make -C infra up` succeeds against the REAL crates on your branch, both `/health` 200, `curl localhost:3000/` shows `<span id="backend-status">ok</span>`; then `make -C infra down`, no containers left.
- `make -C infra check` passes. `infra/STATUS.md` updated (rollback note: revert both FROM lines together), `Ready for integration: yes`.

## Contracts
Runtime (1.1).

## Out of scope
Any other Dockerfile or compose change.
