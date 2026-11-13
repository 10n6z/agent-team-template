---
name: api-change
description: Backend agent only. Use when adding or changing an endpoint, request/response shape, or error code, to implement it test-first against orchestrator/CONTRACTS.md.
---

1. Find the endpoint in CONTRACTS.md. If it's missing or differs from what the task needs, add a Request to `backend/STATUS.md` and stop.
2. Write the test first: happy path, validation error, auth/permission error (if any), not-found. Run it and confirm it fails.
3. Implement the smallest handler that passes. Validate input at the boundary.
4. Check that the response matches the contract exactly (field names, types, status codes, error shape).
5. `make -C backend check` green → `/finish-task`.
