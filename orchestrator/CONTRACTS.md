# Contracts

Interfaces between agents. Only the orchestrator edits this file. Any change needs a version bump and a task for every affected agent.

Version: 0

## API (backend provides; frontend and qa consume)
- Base URL from env `API_URL`.
- Endpoints: _define here: method, path, request, response, error shape._

## Runtime (infra provides; backend and frontend conform)
- Every role dir exposes `make lint`, `make test`, `make build`, `make check`.
- Build outputs: _define here (image names / artifact paths)._
- Env vars: _list name, owner, and example value. No real secrets._

## Deploy (infra)
- `make -C infra deploy` is the only deploy entrypoint; `ci.yml` calls it on push to main.

## Tests (qa)
- `make -C tests e2e` runs end-to-end tests against `API_URL`.
