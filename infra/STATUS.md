# infra status
Ready for integration: yes

## Done
- `infra/docker/{backend,frontend}.Dockerfile`: multi-stage `rust:1.100` (`cargo build --release --locked`, BuildKit cache mounts) -> `debian:bookworm-slim` + ca-certificates, non-root uid 10001, `PORT` default 8080/3000, `EXPOSE`, `CMD [<binary>]`. `*.Dockerfile.dockerignore` keeps `target/` and `.git` out of the context.
- `infra/compose.yaml`: project `agent-team`, `backend` 8080:8080, `frontend` 3000:3000 with `API_URL=http://backend:8080`, depends_on backend. Contexts `../backend`, `../frontend`.
- `infra/Makefile`: `lint` (compose config -q + actionlint 1.7.7, binary if on PATH else Docker image), `test` (true), `check`, `build` (skips with message if `../backend` or `../frontend` missing), `plan`, `up` (waits for both `/health`, `TIMEOUT` default 300s, dumps `compose logs` + non-zero on timeout), `down` (`--remove-orphans`), `deploy` (stub, prints images, exit 0). All use `-p agent-team`.
- Verified locally: `make -C infra check` green; actionlint fails on a deliberately broken workflow. `up`/`down` smoke-tested on a scratch copy of `infra/` with std-only stub crates named `backend`/`frontend`: up exits 0 when both healthy, exits non-zero with "not ready" on timeout; containers run as uid 10001; frontend sees `API_URL=http://backend:8080`; down leaves nothing behind.
- `.github/workflows/infra.yml` unchanged: `make -C infra lint test build` + `plan` already work on ubuntu-latest (Docker present, no Rust needed).

## In progress
## Blocked
## Requests
- Orchestrator: verify `make -C infra up` against the real backend/frontend crates at integration (they were not on this branch; real crates not yet written when this was done).
- Note for backend/frontend: the image build uses `cargo build --release --locked`, so each crate's `Cargo.lock` must be committed (already required by the contract).

## Rollback
Infra is additive and local-only (no deploy target yet). Roll back with `make -C infra down` then `git revert <infra commit>`; no external state to clean except local images `agent-team-backend`/`agent-team-frontend` (`docker rmi`).
