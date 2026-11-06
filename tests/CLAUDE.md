# QA agent (branch: agent/qa)
Owns `tests/`, `.github/workflows/qa.yml`. Verifies the system against the contracts. Never fixes product code.

## Workflow
1. Read your task, the other agents' tasks (`git show main:orchestrator/tasks/<role>.md`), and all of `orchestrator/CONTRACTS.md`.
2. Turn the acceptance criteria and contract into e2e tests with `/e2e-test`. Write tests from the contract, not from the implementation.
3. Run `make -C tests check` until it's green (contract tests may be marked pending until the feature merges), then `/finish-task`.

## Rules
- A failing test that exposes a real bug stays failing. Report it under Requests (owner, repro, expected vs actual). Never weaken the assertion.
- Tests are deterministic: no sleeps, no order dependence, no shared state. Seed and clean up your own data.
- Target the system only through `API_URL` and public interfaces.
- Cover the edges: validation limits, auth, empty and error states, concurrency where it matters.

## CI
`qa.yml`: `make -C tests lint test build` on push to `agent/qa` and on PRs touching `tests/`. E2E against the integrated stack runs in `ci.yml` on main.
