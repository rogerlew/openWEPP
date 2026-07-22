# CQR: TESTGATE Gate-Planner CLI Complexity

Package: `20260721-cqr-testgate-recovery-02-main-001`
Status: `QUEUED`
ExecPlan: `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
Target module: `crates/openwepp-gate-planner/src/main.rs`
Target rank: `2` of `7`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce the four eligible `main.rs` CLI functions above CRAP 30 to at most 30
while preserving exact option admission, transition ordering, ledger lifecycle,
typed errors, canonical output, and process exit behavior.

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
- `crates/openwepp-gate-planner/src/main.rs`
- `tests/integration/testgate_ci_executor_contract.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes independent review,
verification, and comparator/closure-runner agents for eligibility, semantic
diff review, focused metrics, and terminal verification. Heavy global gates are
delegated and remain owned by the master ExecPlan after all seven packages.

## Scope

In scope: characterization and behavior-preserving private helper extraction in
`main.rs`, plus package evidence. Out of scope: public CLI behavior, policy,
schema, thresholds, other production modules, and campaign-global TESTGATE.

## Declared Write Set

- `crates/openwepp-gate-planner/src/main.rs`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-02-main-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

## Phase Plan

1. Validate target selection and exact pre-production baseline.
2. Characterize option, transition, HEAVY lifecycle, and fallback behavior.
3. Extract private helpers without changing branch/error/side-effect order.
4. Measure production-only coverage/regions and matching CRAP.
5. Complete dual review, dual verification, disposition, and prompt archival.

## Exit Criteria

- All four eligible rows are CRAP at most 30 without exception.
- Public output, error codes, option sets, transition order, and ledger effects
  remain equivalent.
- ADR-0021 aggregate, per-function region, and obligation gates pass if tests
  are materially changed.
- Dual review and dual verification pass; no package-local gate remains open.

## Security Impact Gate

- security_impact: moderate
- dedicated_security_review_required: no
- rationale: fail-closed CLI/trust orchestration; dual review checks precedence.
