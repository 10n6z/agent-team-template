---
name: ui-change
description: Frontend agent only. Use when adding or changing a page, component, or API call in frontend/, to build it contract-first with tests and a11y checks.
---

1. List the endpoints the view needs and confirm each one in CONTRACTS.md. Missing → add a Request to `frontend/STATUS.md`.
2. Add or extend the typed API client (and a contract-matching mock if the backend isn't merged yet).
3. Write a component test that covers loading, empty, error and success states.
4. Build the UI. Use labels, keyboard-only navigation and visible focus.
5. `make -C frontend check` green → `/finish-task`.
