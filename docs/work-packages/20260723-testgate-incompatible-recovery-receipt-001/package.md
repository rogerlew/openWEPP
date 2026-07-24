# TESTGATE Incompatible Recovery Receipt Hardening

Package ID: `20260723-testgate-incompatible-recovery-receipt-001`

Status: `ACTIVE`

## Objective

Retain an incompatible prior recovery receipt as a typed rejection decision and
continue the newly admitted attempt without importing it. Malformed provenance,
ledger, path, or checkpoint evidence remains fail-closed.

## Intended Write Set

- `crates/openwepp-gate-planner/src/resume.rs`
- `crates/openwepp-gate-planner/src/pre_heavy.rs`
- `crates/openwepp-gate-planner/src/pre_heavy_coverage_tests.rs`
- `crates/openwepp-gate-planner/src/pre_heavy_tests.rs`
- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`
- `crates/openwepp-gate-planner/src/verifier.rs`
- `.config/nextest.toml`
- `.github/workflows/testgate-shadow.yml`
- `gate-policy/v1/schemas/gate-receipt.schema.json`
- `tools/ci/omarchy-runner/manage.sh`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/**`
- `docs/work-packages/20260720-testgate-workflow-qualify-001/**`
- `docs/work-packages/20260724-cqr37-testgate-relocated-audit-001/**`
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
- Independent hosted verification accepts a sealed audit and LIGHT receipt
  after evidence relocation by checking their mutually sealed artifact-root
  identity. Runtime audit construction still binds both documents to the
  actual forest1 execution root, and artifact contents remain independently
  verified.
- Repository-snapshot tests use an executable disposable root and run without
  concurrent workspace-linker pressure inside the full nextest profile.
- All other recovery-integrity errors remain typed and fail closed.
- Focused resume tests pass.
- The public relocated-audit verifier is exercised directly under the
  instrumented affected-quality profile and has adjudicated CRAP at most `30`.
- Final hosted authority publication uploads only the authenticated signing
  subject, verification proofs, terminal plan, and reusable global-quality
  report. The already-published full verified archive is not uploaded again.
- Routine affected/full profiles retain deterministic planner, verifier,
  receipt, and JUnit contract coverage without repeatedly enumerating or
  reconstructing the complete repository. Explicit development-only fixtures
  own live repository reconstruction, have a documented invocation, and remain
  required when their inventory/reconstruction surfaces change.
- One exact changed-head forest1 TESTGATE qualification proves that a rejected
  incompatible receipt is retained without blocking the current attempt. The
  permanent queued records from retired Omarchy are ignored; no duplicate
  dispatch is permitted.
