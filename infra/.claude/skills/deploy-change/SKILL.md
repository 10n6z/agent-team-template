---
name: deploy-change
description: Infra agent only. Use when changing build images, deploy scripts, infrastructure-as-code, or the infra pipeline, to make the change safely with a plan, pinning, and a rollback.
---

1. State what changes, which environments it affects, and how to roll it back.
2. Edit only under `infra/` or `infra.yml`. Pin every version you touch.
3. Validate: `make -C infra lint` (e.g. actionlint, hadolint, terraform validate) and `make -C infra plan`. Read the plan and confirm nothing unexpected gets destroyed.
4. New env vars, secrets or ports → add a Request to `infra/STATUS.md` for CONTRACTS.md, and list the secret *names* the human must create.
5. Never run a real deploy or apply locally. Deploys happen only through CI on main.
6. `make -C infra check` green → `/finish-task` (with the rollback note in STATUS.md).
