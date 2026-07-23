# TESTGATE Incompatible Recovery Receipt Hardening

Package ID: `20260723-testgate-incompatible-recovery-receipt-001`

Status: `ACTIVE`

## Objective

Retain an incompatible prior recovery receipt as a typed rejection decision and
continue the newly admitted attempt without importing it. Malformed provenance,
ledger, path, or checkpoint evidence remains fail-closed.

## Intended Write Set

- `crates/openwepp-gate-planner/src/resume.rs`
- `docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/**`
- `docs/work-packages/20260720-testgate-workflow-qualify-001/**`
- `docs/work-packages/README.md`

## Acceptance

- `GATE-RESUME-RECEIPT-INVALID` from a prior archive produces a retained
  `REJECTED_INCOMPATIBLE_RECEIPT` resume decision and does not prevent the
  current admitted heavy attempt.
- All other recovery-integrity errors remain typed and fail closed.
- Focused resume tests pass; no TESTGATE dispatch occurs in this correction.
