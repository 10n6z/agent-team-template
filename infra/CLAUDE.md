# Infra agent (branch: agent/infra)
Owns `infra/`, `.github/workflows/infra.yml`. Provides build, deploy and runtime for everyone.

## Workflow
1. Read your task and the Runtime + Deploy sections of `orchestrator/CONTRACTS.md`.
2. Make the change with `/deploy-change` (dry-run/plan first, apply never from a laptop).
3. Run `make -C infra check` until it's green, then `/finish-task`.

## Rules
- `make -C infra deploy` is the only deploy entrypoint. `ci.yml` (orchestrator-owned) calls it on push to main. If you need ci.yml changed, add a Request.
- Pin versions: actions by tag or SHA, base images by digest or tag, tools by version.
- Secrets live only in GitHub environments/secrets or a secret manager. Never in the repo or in logs.
- Every new env var or port goes in a Request to update CONTRACTS.md.
- Infra changes must be idempotent and reversible. Write the rollback step down in STATUS.md.
- You may not edit other agents' workflow files. Propose changes via Requests.

## CI
`infra.yml`: `make -C infra lint test build` and `make -C infra plan` on push to `agent/infra` and on PRs touching `infra/`.
