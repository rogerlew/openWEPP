# TESTGATE Incompatible Recovery Receipt Hardening

Package ID: `20260723-testgate-incompatible-recovery-receipt-001`

Status: `ACTIVE`

## Objective

Retain an incompatible prior recovery receipt as a typed rejection decision and
continue the newly admitted attempt without importing it. Malformed provenance,
ledger, path, or checkpoint evidence remains fail-closed.

## Intended Write Set

- `crates/openwepp-gate-planner/src/resume.rs`
- `gate-policy/v1/schemas/gate-receipt.schema.json`
- `tools/ci/omarchy-runner/manage.sh`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/**`
- `docs/work-packages/20260720-testgate-workflow-qualify-001/**`
- `docs/work-packages/README.md`

## Acceptance

- `GATE-RESUME-RECEIPT-INVALID` from a prior archive produces a retained
  `REJECTED_INCOMPATIBLE_RECEIPT` resume decision and does not prevent the
  current admitted heavy attempt.
- The receipt schema admits that archive-level rejection as a distinct,
  fail-closed decision shape without weakening node-level `IMPORTED` or
  `RERUN` bindings.
- The trusted forest1 runner provides enough bounded tmpfs capacity for the
  selected full-workspace gate without weakening its read-only or ephemeral
  workspace controls.
- All other recovery-integrity errors remain typed and fail closed.
- Focused resume tests pass.
- One exact changed-head forest1 TESTGATE qualification proves that a rejected
  incompatible receipt is retained without blocking the current attempt. The
  permanent queued records from retired Omarchy are ignored; no duplicate
  dispatch is permitted.
