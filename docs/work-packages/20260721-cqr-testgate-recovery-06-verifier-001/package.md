# CQR: TESTGATE Verifier Complexity

Package: `20260721-cqr-testgate-recovery-06-verifier-001`
Status: `EXECUTING`
ExecPlan: `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
Target module: `crates/openwepp-gate-planner/src/verifier.rs`
Target rank: `6` of `7`
Quality dimension: `CRAP/cyclomatic-complexity`

Current phase: dual implementation review findings and the discovered
write-set syntax defect are corrected; focused clean-head validation and the
single corrected-head metric passed. Renewed dual review passed and RTR-028 is
durably closed. Dual terminal verification remains pending.

## Objective

Reduce the one eligible verifier function above CRAP 30 to at most 30 while
preserving READY-audit admission, live execution-context validation, HEAVY-plan
requirements, receipt verification order, typed errors, and public behavior.

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
- `crates/openwepp-gate-planner/src/verifier.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation for independent eligibility, implementation review,
matching-module metrics, and terminal verification. Outputs are package-local
review, verification, metric, and command evidence. Write access is read-only
unless explicitly assigned a bounded change inside the declared write set.
Heavy global gates remain owned by the master ExecPlan.

## Scope

In scope: characterization and behavior-preserving private helper extraction in
`verifier.rs`, the split test-only `verifier_coverage_tests.rs`, and package
evidence. Out of scope: schema/policy changes, receipt or audit semantics,
execution-context policy, other production modules, and campaign-global
TESTGATE.

## Declared Write Set

- `crates/openwepp-gate-planner/src/verifier.rs`
- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-06-verifier-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

## Phase Plan

1. Validate retained row, eligibility, and one exact pre-production baseline.
2. Bind valid and rejecting READY-audit receipt-verification paths before edit.
3. Extract private helpers without changing validation or error order.
4. Measure matching production coverage/regions and CRAP once per changed head.
5. Complete fresh dual review, dual verification, disposition, and handoff.

## Exit Criteria

- The original and every extracted helper are CRAP at most 30.
- READY-audit receipt verification, errors, ordering, and output remain exact.
- Applicable ADR-0021 coverage/floor gates and dual review/verification pass.

## Security Impact Gate

- security_impact: high
- dedicated_security_review_required: no
- rationale: trust-bearing receipt verification receives dual independent
  review.
