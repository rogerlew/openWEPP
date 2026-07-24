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

## Qualification Progress

Evidence class: Ran

- Exact production reconstruction passed on clean head `1366e5b4`: one test
  passed in 459.20 seconds.
- Routine gate-planner regression passed: 174 tests passed, 14
  development-only tests skipped, in 62.089 seconds.
- Fresh affected CRAP round 1 on `1366e5b4` correctly rejected
  `Planner::build_nodes` at CRAP 30.605 and 82.963% coverage.
- A focused reconstruction now proves exact selected-authority arguments and
  inventory through `Planner::build_nodes`.
- Fresh affected CRAP round 2 passed on clean head `e6b0feb1`: raw debt 0,
  adjudicated debt 0, actionable debt 0. Retained local artifacts:
  `target/cqr-testgate-coverage-reconstruction-affected-crap-r2/`.
- Exact terminal planning passed on `4aa731d4`: authority `READY`, zero
  unauthorized paths, 15 nodes, and 2,316 planned inventory items.
- Forest1 run `30100370376` passed all ten pre-heavy checks and every executed
  test, including 2,262 full-profile tests, but failed closed with
  `GATE-EXEC-JUNIT-INVENTORY`: the planner expected 2,298 tests because its
  Nextest inventory command omitted `--profile full`. The exact 36-item delta
  was the manual science cohort intentionally excluded from `full`.
- The inventory enumerator now propagates the gate definition's selected
  Nextest profile. A coverage-only regression proves that `full` excludes
  exactly 36 manual science fixtures; routine regression remains 175 passing
  tests with 15 development-only tests skipped.
- Forest1 run `30104030978` confirmed the profile fix by clearing exact JUnit
  comparison, then the full suite failed its own cohort-governance contract
  because the new profile regression was temporarily a thirteenth development
  fixture while the explicit profile still declared twelve.
- Forest1 run `30107431813` cleared both prior seams, but the added thirteenth
  fixture launched nested Cargo compilation inside instrumented coverage and
  failed on the runner's protected target context. The nested live fixture was
  removed, profile binding is covered by a pure unit test, and the explicit
  live reconstruction cohort is restored to 12.
- Changed-head forest1 TESTGATE success remains pending.
