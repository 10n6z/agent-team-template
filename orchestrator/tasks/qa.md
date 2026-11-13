# Task: qa — v1.1 e2e runner
Status: done            <!-- assigned | in-progress | done -->

## Goal
Apply contract 1.1 answers to your Requests.

## Acceptance criteria
- First `git merge main`.
- `tests/Makefile` `e2e` = `cargo test --no-fail-fast -- --ignored` (CONTRACTS.md 1.1 Tests).
- Your Requests are answered in CONTRACTS.md 1.1: frontend `/health` is `application/json` (keep your assertion); frontend 404 body is unspecified (keep status-only); span match is literal `id="backend-status"`, exact `ok` (current strictness is correct). Move them to Done in `tests/STATUS.md`.
- `make -C tests check` passes. `Ready for integration: yes`.

## Contracts
Tests (1.1), Frontend service (1.1).

## Out of scope
New tests.
