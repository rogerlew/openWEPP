# CQR: TESTGATE Resume Complexity

Package: `20260721-cqr-testgate-recovery-04-resume-001`
Status: `COMPLETE`
ExecPlan: `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
Target module: `crates/openwepp-gate-planner/src/resume.rs`
Target rank: `4` of `7`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce all three eligible resume/recovery functions above CRAP 30 to at most 30
while preserving exact candidate admission, attestation verification,
checkpoint validation, error precedence, and recovery behavior.

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
- `crates/openwepp-gate-planner/src/resume.rs`
- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/checkpoint_mirror.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to independent review, verification, and
comparator/closure-runner subagents for eligibility, semantic review, focused
metrics, and terminal verification. Outputs are package-local review,
verification, metric, and command evidence. Write access is read-only unless
explicitly assigned a bounded change inside the declared write set. Heavy
global gates remain owned by the master ExecPlan.

## Scope

In scope: characterization and behavior-preserving private helper extraction in
`resume.rs`, one test-only call from the existing pre-HEAVY coverage fixture to
the public constructed-audit resume entry point, plus package evidence. Out of
scope: schema/policy changes, attestation trust changes, checkpoint format
changes, consumer changes, other production modules, and campaign-global
TESTGATE.

## Declared Write Set

- `crates/openwepp-gate-planner/src/resume.rs`
- `crates/openwepp-gate-planner/src/resume_coverage_tests.rs` (test-only
  characterization split for line-count governance)
- `crates/openwepp-gate-planner/src/pre_heavy_coverage_tests.rs` (test-only
  characterization; no second production module)
- `docs/work-packages/20260721-cqr-testgate-recovery-04-resume-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

## Phase Plan

1. Validate exact rows, eligibility, and pre-production baseline.
2. Bind or add candidate/attestation/checkpoint characterization.
3. Extract private helpers without changing validation/error order.
4. Measure matching production coverage/regions and CRAP.
5. Complete dual review, dual verification, disposition, and prompt archival.

## Exit Criteria

- All three originals and every extracted helper are CRAP at most 30.
- Candidate selection, trust/identity checks, checkpoint decisions, errors, and
  outputs remain equivalent.
- Applicable ADR-0021 coverage/floor gates and dual review/verification pass.

## Security Impact Gate

- security_impact: high
- dedicated_security_review_required: no
- rationale: trust-bearing recovery admission receives dual independent review.
