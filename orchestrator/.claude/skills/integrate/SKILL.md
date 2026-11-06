---
name: integrate
description: Orchestrator only. Use to review and merge a worker agent's branch (agent/<role>) into main.
---

Args: `<role>`

1. `git log --oneline main..agent/<role>`. If there is nothing to merge, stop.
2. Scope: `git diff --name-only main...agent/<role>`. Every path must match that role's prefixes in `.claude/owners`. If any doesn't, reject and task the owner.
3. Readiness: `git show agent/<role>:<dir>/STATUS.md` must say `Ready for integration: yes`.
4. Review `git diff main...agent/<role>` against the task's acceptance criteria and CONTRACTS.md.
5. `git merge --no-ff agent/<role> -m "merge(<role>): <task title>"`
6. `make check`. If it's red: `git revert -m 1 HEAD`, then `/dispatch <role>` with the failure output.
7. Set `Status: done` in `orchestrator/tasks/<role>.md` and commit.
8. Act on the branch's Requests (contract changes / new tasks). Never push.
