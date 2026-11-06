# Team rules (every agent, every branch)

This repo is worked by 5 Claude Code sessions: one orchestrator on `main`, four workers on `agent/<role>`, each in its own git worktree.
Your role, editable paths and current task are injected at session start from `.claude/owners` and `orchestrator/tasks/<role>.md`.
Re-print them any time: `.claude/hooks/agent.sh start`.

## Rules
1. Stay on your branch. Never switch branches and never merge into `main`. Only the orchestrator merges.
2. Edit only your owned paths (hook-enforced for Edit/Write; also applies to shell writes). If you need a change elsewhere, add it under **Requests** in your `STATUS.md` and move on or stop.
3. Never push, force, or `reset --hard`. A human pushes.
4. Interfaces between agents live in `orchestrator/CONTRACTS.md`. Code to the contract. If the contract is wrong, request a change. Don't work around it.
5. Commit small, Conventional Commits, scope = role: `feat(backend): add /health`. Commit only when `make -C <your dir> check` passes.
6. Keep `<your dir>/STATUS.md` current. The orchestrator reads it from your branch; it is how it sees you.
7. Get other agents' merged work only via `git merge main`.
8. No secrets in the repo. Config comes from env vars listed in CONTRACTS.md; CI secrets via `${{ secrets.* }}`.
9. Done = acceptance criteria in your task met + checks green + STATUS.md updated. Use `/finish-task`.
