---
name: dispatch
description: Orchestrator only. Use to assign or update work for a worker agent (backend, frontend, infra, qa). Writes orchestrator/tasks/<role>.md and commits it on main.
---

Args: `<role> <goal>`

1. Check that `orchestrator/CONTRACTS.md` covers every interface the task touches. If not, update it first (bump Version) and dispatch the matching change to each affected agent.
2. Write `orchestrator/tasks/<role>.md`:
   ```markdown
   # Task: <role> — <short title>
   Status: assigned        <!-- assigned | in-progress | done -->
   ## Goal
   ## Acceptance criteria  <!-- checkable, prefer "test X passes" -->
   ## Contracts            <!-- CONTRACTS.md sections this relies on -->
   ## Out of scope
   ```
3. `git add orchestrator/ && git commit -m "chore(orchestrator): dispatch <role>: <title>"`
4. Tell the user to start or restart the `<role>` session in `../<repo>-<role>`. A session that is already running can read the task with `git show main:orchestrator/tasks/<role>.md`.
