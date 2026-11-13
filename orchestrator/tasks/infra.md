# Task: infra — container runtime, local stack, deploy stub
Status: assigned        <!-- assigned | in-progress | done -->

## Goal
Make backend + frontend runnable as containers with one command, and give CI a deploy entrypoint.

## Acceptance criteria
- `infra/docker/backend.Dockerfile`, `infra/docker/frontend.Dockerfile`, `infra/compose.yaml` exactly per contract Runtime section (multi-stage, `rust:1.100` → `debian:bookworm-slim` + ca-certificates, non-root, binary names `backend`/`frontend`).
- `make -C infra up` / `down` / `deploy` behave per contract (`up` waits for both `/health` from the host, 300s timeout, non-zero on timeout).
- `make -C infra lint`: `docker compose -f compose.yaml config -q` plus actionlint over `../.github/workflows/*.yml` (run actionlint via its Docker image `rhysd/actionlint:1.7.7` if not installed). `test`: `true` for now. `check` = `lint test`. `build`: `docker compose build`. `plan`: print the image/service set.
- `make -C infra check` passes.
- The backend/frontend code is not on your branch yet, so verify `up` against them by pointing compose at the other worktrees or the merged state, or leave a precise note in STATUS.md that the orchestrator will verify `up` at integration.
- `.github/workflows/infra.yml`: keeps `make -C infra lint test build` + `plan`. `build` needs `../backend` and `../frontend` to exist, so skip it with a clear message if they don't (e.g. `[ -d backend ] || exit 0` inside the Make target).
- `infra/STATUS.md` updated, `Ready for integration: yes`.

## Contracts
Toolchain, Runtime, Deploy.

## Out of scope
Real cloud deploy, registries, secrets, TLS.
