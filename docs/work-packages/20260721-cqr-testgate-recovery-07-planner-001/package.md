# CQR: TESTGATE Planner Complexity

Package: `20260721-cqr-testgate-recovery-07-planner-001`
Status: `HOLD-CORRECTION-DEPENDENCY`
ExecPlan: `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
Target module: `crates/openwepp-gate-planner/src/planner.rs`
Target rank: `7` of `7`
Quality dimension: `CRAP/cyclomatic-complexity`

Current phase: the one authorized baseline traversal failed because the
rank-6 verifier characterization derives its package authority from the
ambient `HEAD^..HEAD` diff. RTR-029 and the bounded package
`20260722-testgate-verifier-ready-audit-fixture-001` own the canonical fixture
correction. No unchanged planner metric traversal may run while this hold is
active.

## Objective

Reduce the sole eligible planner function above CRAP 30 to at most 30 while
preserving plan selection, terminal reconciliation, execution-context binding,
node identity, ordering, errors, schemas, and public behavior.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/openwepp-gate-planner/src/planner.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation for independent eligibility, implementation review,
matching-module metrics, and terminal verification. Outputs are package-local
review, verification, metric, and command evidence. Write access is read-only
unless explicitly assigned a bounded change inside the declared write set.
Heavy global gates remain owned by the master ExecPlan.

## Scope

In scope: mechanical test-only splitting, characterization, and
behavior-preserving private helper extraction in `planner.rs`, the split
`planner_coverage_tests.rs`, and package evidence. Out of scope: schema/policy
changes, selection or reconciliation semantics, other production modules, and
campaign-global TESTGATE.

## Declared Write Set

- `crates/openwepp-gate-planner/src/planner.rs`
- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-07-planner-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

## Phase Plan

1. Validate the retained row, eligibility, and one exact baseline.
2. Mechanically split inline tests and bind characterization before extraction.
3. Extract one cohesive private decision block with exact behavior/order.
4. Measure matching production coverage/regions and CRAP once per changed head.
5. Complete fresh dual review, dual verification, disposition, and closeout handoff.

## Exit Criteria

- The target and every extracted helper are CRAP at most 30.
- Plan selection, identities, errors, reconciliation, and outputs remain exact.
- Applicable ADR-0021 coverage/floor gates and dual review/verification pass.
- `planner.rs` remains below the 3,000-line closure block.

## Security Impact Gate

- security_impact: high
- dedicated_security_review_required: no
- rationale: trust-bearing planning and execution selection receive dual
  independent review.
