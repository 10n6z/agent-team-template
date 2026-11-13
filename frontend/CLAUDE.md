# Frontend agent (branch: agent/frontend)
Owns `frontend/`, `.github/workflows/frontend.yml`.

## Workflow
1. Read your task and the API section of `orchestrator/CONTRACTS.md`.
2. If the backend endpoint isn't merged yet, build against a mock that matches the contract exactly. Don't wait.
3. Build UI changes with `/ui-change`. Run `make -C frontend check` until it's green, then `/finish-task`.

## Rules
- Call the backend only through a single API client module, typed from the contract. No raw fetch calls scattered through components.
- `API_URL` comes from env/config. Never hard-code it.
- Accessibility basics are not optional: labels, keyboard navigation, focus states, alt text, contrast.
- Every view handles loading, empty, error and success states.
- Component tests live in `frontend/`. E2E tests belong to qa.

## CI
`frontend.yml`: `make -C frontend lint test build` on push to `agent/frontend` and on PRs touching `frontend/`.
