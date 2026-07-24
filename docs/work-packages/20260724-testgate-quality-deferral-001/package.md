# TESTGATE Quality Execution Deferral

Package ID: `20260724-testgate-quality-deferral-001`

Status: `COMPLETE`

## Objective

Implement ADR-0041 in TESTGATE by removing affected/global coverage and CRAP
execution nodes from the increment DAG and issuing a verified,
closure-eligible `DEFERRED_TO_QUALITY_CI` receipt disposition.

## Included Scope

- Gate definitions, plan/receipt schemas, planner, executor, verifier, fixtures,
  source contracts, and tests.
- Removal of `affected-adjudicated-crap-v1`, `adjudicated-crap-v1`, and
  `combined-workspace-quality-v1` from ordinary increment execution.
- Explicit typed quality deferral in the plan and receipt.
- Preservation of all mechanically selected correctness and science gates.

## Excluded Scope

- QA collection/workflow implementation.
- Changes to CRAP thresholds, classifications, or CQR behavior.
- Live workflow dispatch.

## Declared Write Set

- `gate-policy/v1/**`
- `crates/openwepp-gate-planner/**`
- `tests/integration/testgate_align_authority_contract.rs`
- `tests/integration/testgate_assure_campaign_currency_contract.rs`
- `tests/integration/testgate_ci_executor_contract.rs`
- `tools/local_ci/**`
- `tools/release/**`
- `.github/workflows/testgate-shadow.yml`
- `.github/workflows/testgate-conservative.yml`
- `.github/workflows/release-gates.yml`
- `docs/work-packages/20260724-testgate-quality-deferral-001/**`
- `docs/work-packages/README.md`

## Dependencies

- `20260724-testgate-quality-authority-separation-001` complete.

## Phase Plan

1. Characterize current quality-node selection and verifier rejection.
2. Add the closed typed deferral representation to policy and schemas.
3. Remove ordinary increment quality processes from planner/executor DAGs.
4. Update independent verification and receipts to prove intentional deferral.
5. Prove all non-quality selected nodes and inventories remain unchanged.
6. Reconcile, review, verify, and disposition.

## Exit Criteria

- Ordinary production TESTGATE plans execute no coverage/CRAP subprocess.
- Plan and receipt contain exactly `DEFERRED_TO_QUALITY_CI` with the adopted
  owner/trigger fields; absent, unknown, or conflicting states fail closed.
- Independent reconstruction derives the same deferral rather than trusting
  producer text.
- Golden tests prove all correctness/authority/conservation/consumer/science
  nodes are unchanged for representative affected plans.
- Combined-quality proof inputs cannot smuggle a quality PASS into increment
  closure and obsolete combined-proof dispatch input is removed or rejected.
- Conservative TESTGATE and routine hosted release validation no longer
  recollect global CRAP through the retired combined-quality path. Any adopted
  release-only QA prerequisite verifies a supplied current evidence identity.
- A retained pre-split receipt containing removed quality nodes is classified
  `REJECTED_INCOMPATIBLE_RECEIPT`; valid recovery provenance is preserved and
  the new execution proceeds normally.
- Focused planner/executor/verifier/schema/source-contract tests, dual review,
  dual verification, and security-impact review pass.

## Security Impact

The verifier independently reconstructs the quality deferral. Producer-only
claims, `NOT_APPLICABLE`, `SKIPPED`, and unknown strings are invalid.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only implementation/security reviewers and two read-only terminal
verifiers; expected outputs are compact package artifacts; write access is
read-only.

## Prospective Amendments

- 2026-07-24: Added
  `tests/integration/testgate_assure_campaign_currency_contract.rs` to the
  declared write set before editing it. Removing the obsolete
  `combined_quality_proof_id` planner request field requires updating this
  compile-time integration caller; retaining a deprecated request surface would
  violate the package exit criterion that obsolete combined-proof input be
  removed or rejected.
