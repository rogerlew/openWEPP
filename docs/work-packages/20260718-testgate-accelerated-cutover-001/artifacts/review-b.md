# Review B: Schema, Selection, And Test Economy

Evidence class: static review plus focused and exact-candidate execution.

The independent schema/selection review checked closed schemas, risk mapping,
plan/receipt identity, Nextest inventory, affected/global quality selection,
workflow triggers and permissions, exact runner-image binding, rollback, and
test-economy regressions.

## Accepted Findings And Resolution

- Cold offline metadata lacked a locked dependency. Bootstrap now fetches both
  exact trees before any offline planner operation.
- Full-suite inventory counted five explicit filter mismatches as runnable.
  The adapter now excludes ignored inventory consistently; the already-passed
  full command was not rerun merely to refresh presentation evidence.
- Coverage bypassed Nextest, inherited coverage controls into nested
  reconstructions, used a non-executable fallback, and allowed uncontrolled
  competing builds. Global/affected coverage now uses Nextest, neutral nested
  Cargo state, an executable temp target, four Cargo build jobs, and stripped
  test debug symbols without weakening assertions or inventory.
- Ubuntu exposed portable fixture assumptions and one exact Iwagaki fingerprint
  variant. Accepted patches preserved the same assertions and physics while
  deriving repository-local paths and binding both immutable platform results.
- The conservative reuse predicate required zero raw/adjudicated rows despite
  canonical 2/2/0 closure. The accepted patch requires PASS, closure eligibility,
  canonical-valid adjudications, equality of raw and adjudicated counts, and
  zero actionable rows.

The exact candidate passed 2,165/2,165 full-profile tests, dependency policy,
and global CRAP with no actionable row. Only the two cheap provider consumer
proofs and terminal closure remain.
