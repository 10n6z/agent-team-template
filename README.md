# Agent Team Template

**Repo:** https://github.com/10n6z/agent-team-template (private)

One orchestrator plus four worker agents. Each one is a separate Claude Code session in its own git worktree and branch.

| Branch | Agent | Owns | CI |
|---|---|---|---|
| `main` | orchestrator | `orchestrator/`, root config, `ci.yml` | `ci.yml`: all checks + e2e, deploy on push to main |
| `agent/backend` | backend | `backend/` | `backend.yml` |
| `agent/frontend` | frontend | `frontend/` | `frontend.yml` |
| `agent/infra` | infra (CI/CD, deploy) | `infra/` | `infra.yml` |
| `agent/qa` | qa | `tests/` | `qa.yml` |

`.claude/owners` is the single source of truth. To add or rename a role, edit it there.

## Start
```bash
make worktrees                                   # ../agent-team-template-{backend,frontend,infra,qa}
claude                                           # here, on main = orchestrator
cd ../agent-team-template-backend && claude      # one terminal per worker
```

## Loop
1. **Orchestrator** writes contracts (`orchestrator/CONTRACTS.md`) and tasks (`/dispatch <role> <goal>`) and commits them on main.
2. **Worker** session starts. A hook injects its role, its role `CLAUDE.md`, and its task read live from `main`.
3. The worker edits only its own dir, commits on its own branch, and runs `/finish-task`, which updates `<dir>/STATUS.md`.
4. **Orchestrator** sees every STATUS.md at session start and runs `/integrate <role>`: scope check, review, `merge --no-ff`, `make check`.
5. Workers run `git merge main` to pick up the others' work.

Owned paths don't overlap, so merges never conflict. Cross-agent needs go through **Requests** in STATUS.md, never through direct edits.

## Guardrails
| What | How |
|---|---|
| Who am I / what's my task | `SessionStart` hook → `.claude/hooks/agent.sh start` |
| Stay in your lane | `PreToolUse` hook blocks Edit/Write outside owned paths; `/integrate` re-checks the scope of the whole diff (catches shell writes) |
| Don't finish red | `Stop` hook runs `make -C <dir> check` and sends failures back to the agent |
| Nothing leaves the machine | `permissions.deny` on `git push`, `reset --hard`, `branch -D`, `worktree remove`; no remote configured |
| Shared rules | root `CLAUDE.md`; role rules in `<dir>/CLAUDE.md`; role skills in `<dir>/.claude/skills/` |

## CI/CD
- **Per agent** (`.github/workflows/<role>.yml`): runs on push to `agent/<role>` and on PRs touching its dir, using `make -C <dir> lint test build`.
- **Integration** (`ci.yml`, main): `make check` + `make -C tests e2e` on PRs and pushes. A `deploy` job runs `make -C infra deploy` on push to main, gated by the `production` environment.
- Every role `Makefile` has `TODO` echo targets. Replace them with your stack's real commands.

## Before the first push
1. Create the remote and push `main` and the `agent/*` branches by hand (the agents can't push).
2. Create the `production` environment with required reviewers.
3. Optional: protect `main` so it only accepts PRs with green `integration`. The orchestrator then opens PRs instead of merging locally.
