# Contracts

Interfaces between agents. Only the orchestrator edits this file. Any change needs a version bump and a task for every affected agent.

Version: 1.1 — walking skeleton

Changelog
- 1.1: build/runtime images pinned to the same Debian release (1.0 mixed trixie glibc 2.41 build with bookworm glibc 2.36 runtime → frontend crashed `GLIBC_2.38 not found`); reqwest 0.13 feature name; frontend `/health` content-type and 404 body clarified; e2e runs `--no-fail-fast`. Open for v2: 405 JSON shape for wrong methods.

## Toolchain (everyone)
- Rust **1.100** stable, edition 2021, with `clippy` + `rustfmt`.
- Each role dir is its own standalone Cargo package with its own `Cargo.lock` (committed). **No root workspace.** Root `.gitignore` already ignores `target/`.
- No OpenSSL: HTTP clients use rustls. reqwest 0.13: `default-features = false, features = ["json", "rustls"]` (0.12 named it `rustls-tls`). `Cargo.lock` must not contain `openssl-sys` or `native-tls` (`openssl-probe` is fine).
- CI toolchain step (copy as-is into your workflow, before any `make`):
  ```yaml
  - run: rustup toolchain install 1.100 --profile minimal -c clippy,rustfmt && rustup default 1.100
  ```
- Local without host Rust: `docker run --rm -v "$PWD":/w -w /w -v agent-team-cargo:/usr/local/cargo/registry agent-team-rust:1.100 make -C <dir> check`

## Make targets (every role dir)
| Target | Must do |
|---|---|
| `lint` | `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` |
| `test` | `cargo test` (no running services needed) |
| `build` | `cargo build --release` |
| `check` | `lint` + `test` |
| `run` (backend, frontend) | `cargo run --release` |

## Backend service (backend provides; frontend, qa, infra consume)
- Package and binary name: `backend`. Binds `0.0.0.0:$PORT`, `PORT` default `8080`.
- `GET /health` → `200`, `Content-Type: application/json`, body exactly these keys:
  `{"status":"ok","service":"backend","version":"<CARGO_PKG_VERSION>"}`
- Any unknown path → `404`, JSON body `{"error":"not_found"}`.
- Error shape for every future error: `{"error":"<snake_case_code>"}`.

## Frontend service (frontend provides; qa, infra consume)
- Package and binary name: `frontend`. Binds `0.0.0.0:$PORT`, `PORT` default `3000`.
- Reads `API_URL` (default `http://localhost:8080`) and calls backend `GET {API_URL}/health` server-side on each `GET /` (timeout 2s).
- `GET /` → `200`, `Content-Type: text/html; charset=utf-8`, page contains exactly one `<span id="backend-status">VALUE</span>` where VALUE is `ok` when backend health returns 200 with `"status":"ok"`, otherwise `unavailable`. The page must render (200) even when backend is down.
- `GET /health` → `200`, `Content-Type: application/json`, body `{"status":"ok","service":"frontend","version":"<CARGO_PKG_VERSION>"}`.
- Unknown path → `404`; body unspecified (status is the contract).
- The status span is matched literally: `id="backend-status"` (double quotes), text exactly `ok` / `unavailable`, no surrounding whitespace.

## Runtime (infra provides)
- `infra/docker/backend.Dockerfile`, `infra/docker/frontend.Dockerfile`: multi-stage, build stage `rust:1.100-trixie`, runtime `debian:trixie-slim` + `ca-certificates`, non-root user. **Build and runtime must be the same Debian release** (glibc compatibility); change both together or neither. Build context = the service dir (`backend/`, `frontend/`).
- `infra/compose.yaml`: services `backend` (host port 8080) and `frontend` (host port 3000, `API_URL=http://backend:8080`).
- `make -C infra up`: builds and starts both, returns only when both `/health` answer 200 from the host (timeout 300s, non-zero exit on timeout).
- `make -C infra down`: stops and removes everything `up` created.
- `make -C infra deploy`: stub for now; prints what it would deploy, exit 0. Called only from `ci.yml` on push to main.

## Tests (qa provides)
- Package `e2e` in `tests/`. Reads `API_URL` (default `http://localhost:8080`) and `FRONTEND_URL` (default `http://localhost:3000`).
- E2E tests are `#[ignore]`d so `make -C tests check` passes with no services running; `make -C tests e2e` runs `cargo test --no-fail-fast -- --ignored` against a running stack.
