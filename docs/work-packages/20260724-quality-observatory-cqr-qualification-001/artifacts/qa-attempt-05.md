# QA Attempt 5

Evidence class: Ran.

- Provider run:
  [`30186661261`](https://github.com/rogerlew/openWEPP/actions/runs/30186661261)
- Source/workflow head:
  `46ee538e95b87e919e0f4c59e2bc90b4e48d1761`
- Qualification TESTGATE run: `30185142429`
- Result: `EXECUTION_FAILED`
- Child exit: `2`
- Occupancy: `CLEAR`
- Full log SHA-256:
  `a12e1c0c87a64ed676e06cdcd2124804b02188ebc5e3e094b9860a83bd3441fa`
- Independently verified retained-tail SHA-256:
  `a43e9a5267377450bda9a7ae284f53f80fbe45f570047a13c92f1c8b6db4e558`

The disk-backed temporary-root correction was live and identity-bound. All
three verifier tests that previously failed with linker `SIGBUS` passed. No
linker bus error or OOM occurred.

Two new environment-contract failures remained:

1. The long disk-backed attempt path made a Unix-domain socket fixture exceed
   `SUN_LEN`.
2. The CQR self-test's nested inventory hit the container PID ceiling while
   competing with other full-profile tests (`os error 11`;
   `pids.events max=6`).

`full` completed with exactly those two failures. `science-manual`, JUnit,
LCOV, snowbench reconstruction, CRAP, debt disposition, the 11-file
publication, and quality evidence ID were not reached.

The next changed head shortens the disk-backed attempt root to
`/testgate-history/q/<run>-<attempt>` and adds only the nested CQR self-test to
the existing globally exclusive repository-snapshot cohort. Inventory
selection, profile order, timeouts, and retry policy remain unchanged.

Read-only evidence is retained at
`/home/workdir/openWEPP-quality-history/20260726-order7-qa-run-30186661261`.
