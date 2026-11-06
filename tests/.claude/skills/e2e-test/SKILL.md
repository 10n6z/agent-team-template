---
name: e2e-test
description: QA agent only. Use when writing or updating end-to-end or contract tests in tests/, derived from CONTRACTS.md and task acceptance criteria.
---

1. Pick a contract section or acceptance criterion and write one test per behavior: happy path, each documented error, and the boundaries.
2. Name each test after the behavior (`rejects_empty_email`), not after the function.
3. Make it deterministic: own the setup and teardown data, no sleeps, no reliance on test order.
4. Run it. If it fails because the feature isn't merged, mark it pending and link the task. If it fails because of a bug, keep it failing and add a Request (owner, repro, expected vs actual).
5. `make -C tests check` green → `/finish-task`.
