# CQR: TESTGATE Pre-Heavy Admission Complexity

Package: `20260721-cqr-testgate-recovery-01-pre-heavy-001`
Status: `ACTIVE`
ExecPlan: `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
Target module: `crates/openwepp-gate-planner/src/pre_heavy.rs`
Target rank: `1` of `7`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce each eligible `pre_heavy.rs` function above CRAP 30 to at most 30 while
preserving the pre-HEAVY audit's exact fail-closed admission behavior, error
codes, canonical JSON, artifact identity, and execution ordering.

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
- `crates/openwepp-gate-planner/src/pre_heavy.rs`
- `tests/integration/testgate_ci_executor_contract.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to independent read-only review, verification, and comparator/closure-runner
subagents for CQR eligibility review, source/diff review, focused/full gate
execution, and output-identity checks. Expected outputs are package-local
review, verification, and compact metric artifacts. Write access is read-only
unless a subagent is explicitly assigned a bounded fix in the target module or
this package's artifacts.

Heavy closure/comparator runs required by the terminal plan must be delegated
to `comparator_suite_runner`; the parent must not run them locally unless that
agent is unavailable and the package records command-level evidence.

## Scope

In scope:

- behavior-preserving private helper extraction and local control-flow
  simplification inside `crates/openwepp-gate-planner/src/pre_heavy.rs`;
- characterization tests required for safe extraction;
- package evidence, review, verification, and disposition records.

Out of scope:

- gate-policy, schema, contract, threshold, scheduling, or public CLI changes;
- changes to fail-closed outcome, error precedence, receipt/audit bytes, or
  public output semantics;
- production edits outside the one target module.

## Declared Write Set

- `crates/openwepp-gate-planner/src/pre_heavy.rs`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-01-pre-heavy-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

## Scaffold Commit Gate

This scaffold, prompt, exact retained metric rows, eligibility classification,
and required-reading map must be committed before any production or test edit.

## Phase Plan

1. Confirm the retained baseline and obtain two independent eligibility reviews.
2. Characterize current audit construction and validation behavior before each
   extraction.
3. Extract cohesive private helpers without changing branch order, error code,
   canonicalization, expression order, or consumer call sites.
4. Run focused planner/integration checks and affected-surface metrics.
5. Reconcile the exact terminal plan, delegate selected heavy gates, and record
   dual review, dual verification, line-count governance, and disposition.

## Exit Criteria

- Every eligible target row is CRAP `<= 30` in current affected-surface
  measurement, or has an ADR-0021-compliant, independently accepted exact
  disposition.
- Audit construction, validation, and receipt admission retain behavior,
  canonical artifacts, typed error codes, and fail-closed ordering.
- All terminal-plan gates have current evidence; no failed, blocked, or
  unjustified not-run current-scope gate remains.
- Two independent reviews and two independent verifications disposition every
  finding.

## Security Impact Gate

- security_impact: moderate
- dedicated_security_review_required: no
- rationale: private decomposition of security-sensitive admission logic;
  dual review must explicitly check fail-closed behavior and error precedence.
