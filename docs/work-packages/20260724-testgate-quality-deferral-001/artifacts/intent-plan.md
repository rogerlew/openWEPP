# Order 2 Pre-Implementation Intent Plan

Evidence class: Static.

Base head: `9dbade9bd31dab5d94ab804e2264d3e031e965e3`

Risk: `CRITICAL`.

Reason: this package changes gate policy, planner selection, receipt production,
independent verification, recovery compatibility, workflow execution, and
anti-evasion behavior.

## Intended Change

1. Add one closed quality disposition:
   `DEFERRED_TO_QUALITY_CI`.
2. Bind its owner and trigger fields in policy, plan, receipt, and source
   contracts.
3. Prohibit coverage/CRAP nodes in ordinary TESTGATE plans and execution.
4. Make independent verification reconstruct the disposition and reject
   producer-only, absent, unknown, skipped, not-applicable, passed, or
   conflicting quality claims.
5. Preserve every non-quality gate selected from the same representative
   changes.
6. Preserve historical receipt bytes and classify a pre-split receipt with
   removed quality nodes as `REJECTED_INCOMPATIBLE_RECEIPT`.
7. Remove or reject obsolete combined-quality workflow inputs without
   implementing the later quality observatory.

## Selected Increment Gates

- Gate-policy schema and source-contract validation.
- Focused planner unit/integration tests for selection, serialization,
  reconstruction, negative disposition cases, and non-quality DAG identity.
- Focused TESTGATE executor/verifier/recovery contract tests.
- Workflow source-contract tests for TESTGATE and release validation.
- Rust formatting and warnings-denied Clippy for the affected planner/tooling
  packages selected by the terminal plan.
- Documentation lint for changed package/catalog evidence.
- Repository diff hygiene and package/write-set reconciliation.
- Pre-heavy closure audit followed by delegated full-workspace correctness
  regression because the policy/planner change is critical.

Coverage and CRAP are not selected. This package implements their typed
TESTGATE deferral; it is not an explicit metric-focused package.

## Closure Claims

Order 2 may claim executable typed deferral and focused/full correctness
validation. It may not claim a live forest1 workflow dispatch, quality
observation, merged coverage, CQR intake, or changed-head qualification; those
remain roadmap Orders 3 through 7.
