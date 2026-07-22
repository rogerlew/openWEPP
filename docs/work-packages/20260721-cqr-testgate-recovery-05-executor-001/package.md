# CQR: TESTGATE Executor Complexity

Package: `20260721-cqr-testgate-recovery-05-executor-001`
Status: `QUEUED`
ExecPlan: `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
Target module: `crates/openwepp-gate-planner/src/executor.rs`
Target rank: `5` of `7`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce the two eligible executor functions above CRAP 30 to at most 30 while
preserving stage admission, recovery, execution order, receipt reconstruction,
typed errors, and public behavior.

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
- `crates/openwepp-gate-planner/src/executor.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation for independent eligibility, implementation review,
matching-module metrics, and terminal verification. Outputs are package-local
review, verification, metric, and command evidence. Write access is read-only
unless explicitly assigned a bounded change inside the declared write set.
Heavy global gates remain owned by the master ExecPlan.

## Scope

In scope: characterization and behavior-preserving private helper extraction in
`executor.rs`, test-only coverage fixtures when declared, and package evidence.
Out of scope: schema/policy changes, execution semantics, resource schedules,
other production modules, and campaign-global TESTGATE.

## Declared Write Set

- `crates/openwepp-gate-planner/src/executor.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-05-executor-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

## Phase Plan

1. Validate retained rows, eligibility, and one exact pre-production baseline.
2. Bind or add stage/receipt characterization before decomposition.
3. Extract private helpers without changing validation or error order.
4. Measure matching production coverage/regions and CRAP once per changed head.
5. Complete fresh dual review, dual verification, disposition, and handoff.

## Exit Criteria

- Both originals and every extracted helper are CRAP at most 30.
- Stage execution, receipt reconstruction, errors, and outputs remain equivalent.
- Applicable ADR-0021 coverage/floor gates and dual review/verification pass.

## Security Impact Gate

- security_impact: high
- dedicated_security_review_required: no
- rationale: trust-bearing execution and receipt reconstruction receive dual
  independent review.
