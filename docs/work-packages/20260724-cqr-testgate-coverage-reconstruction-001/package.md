# CQR TESTGATE Coverage-Only Reconstruction

Package ID: `20260724-cqr-testgate-coverage-reconstruction-001`

Status: `ACTIVE`

## Objective

Make the coverage-only TESTGATE reconstruction cohort exact, deterministic, and
CRAP-clean without returning its multi-minute repository reconstruction cost to
routine work-package regression.

## Trigger

Fresh affected CRAP on `396f4895` failed in
`receipt_verification_reconstructs_identity_dag_inventory_and_artifacts` with
`GATE-RECEIPT-PLAN-RECONSTRUCTION`: the supplied synthetic plan did not match
independent exact reconstruction after proportional gate selection added
package doctest and conditional authority/dependency nodes.

## Intended Write Set

- `.config/nextest.toml`
- `Cargo.toml`
- `crates/openwepp-gate-planner/**`
- `gate-policy/v1/**`
- `tests/integration/testgate_align_authority_contract.rs`
- `tests/integration/testgate_ci_executor_contract.rs`
- `tools/release/**`
- `.github/workflows/testgate-shadow.yml`
- `docs/work-packages/20260724-cqr-testgate-coverage-reconstruction-001/**`
- `docs/work-packages/20260724-testgate-science-gate-proportionality-001/**`
- `docs/work-packages/20260723-testgate-incompatible-recovery-receipt-001/**`
- `docs/work-packages/20260720-testgate-workflow-qualify-001/**`
- `docs/work-packages/README.md`

## Constraints

- Routine affected/full regression continues to skip live reconstruction
  fixtures; `cfg(coverage)` and the explicit development profile retain them.
- Independent reconstruction must use the production inventory provider and
  compare exact source selection, node contracts, inventories, and artifacts.
- Synthetic fixtures may test local verifier guards but may not stand in for
  exact reconstruction claims.
- Applicable A0/A1/A3, anti-evasion, and CRAP obligations remain fail-closed.

## Acceptance

- The failing exact reconstruction fixture passes under `cfg(coverage)`.
- Every coverage-only reconstruction fixture passes as one explicit cohort.
- Routine gate-planner regression remains approximately one minute.
- Fresh affected CRAP reports zero actionable rows.
- Exact terminal planning and pre-heavy audit pass.
- A changed-head forest1 TESTGATE run reaches successful independent
  verification and attestation publication.
