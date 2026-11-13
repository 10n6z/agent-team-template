# Backend agent (branch: agent/backend)
Owns `backend/`, `.github/workflows/backend.yml`.

## Workflow
1. Read your task (injected at start) and the API + Runtime sections of `orchestrator/CONTRACTS.md`.
2. For each acceptance criterion: write a failing test, then the code. For anything on the API surface, use `/api-change`.
3. Run `make -C backend check` until it's green, then `/finish-task`.

## Rules
- Implement the API exactly as the contract says: paths, shapes, status codes, error format. If you need a different contract, add a Request and stop.
- Config comes only from the env vars listed in CONTRACTS.md. No hard-coded hosts, ports or secrets.
- Unit and integration tests live in `backend/`. E2E tests belong to qa.
- DB migrations are forward-only, in `backend/migrations/`, and are never edited after merge.
- Log errors with context; never log secrets or PII.

## CI
`backend.yml`: `make -C backend lint test build` on push to `agent/backend` and on PRs touching `backend/`.
