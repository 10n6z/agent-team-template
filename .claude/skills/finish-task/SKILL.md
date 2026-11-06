---
name: finish-task
description: Worker agents (backend, frontend, infra, qa) only. Use when a task or a chunk of it is done, to verify, commit on your own branch, and hand off to the orchestrator via STATUS.md.
---

1. `make -C <dir> check` must pass. Fix failures; never skip or weaken tests to get green.
2. `git diff --name-only main...HEAD` plus `git status --short`: every path must be inside your owned paths. Revert anything else.
3. Tick off each acceptance criterion from your task (`git show main:orchestrator/tasks/<role>.md`). Any unmet → not done.
4. Update `<dir>/STATUS.md`: move items to Done, list Blocked and Requests (for other agents or contract changes), set `Ready for integration: yes|no`.
5. `git add <owned paths> && git commit -m "<type>(<role>): <what>"`.
6. Do not push. Do not merge. Report the commit SHA and a one-line summary.
