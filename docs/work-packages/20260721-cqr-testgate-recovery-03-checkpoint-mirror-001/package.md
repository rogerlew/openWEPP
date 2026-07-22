# CQR: TESTGATE Checkpoint Mirror Complexity

Package: `20260721-cqr-testgate-recovery-03-checkpoint-mirror-001`
Status: `QUEUED`
ExecPlan: `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
Target module: `crates/openwepp-gate-planner/src/checkpoint_mirror.rs`
Target rank: `3` of `7`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce both eligible checkpoint-mirror functions above CRAP 30 to at most 30
while preserving exact fail-closed root validation, directory creation,
symlink/special-entry rejection, output-copy ordering, and canonical checkpoint
publication.

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
- `crates/openwepp-gate-planner/src/checkpoint_mirror.rs`
- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/resume.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to independent review, verification, and
comparator/closure-runner subagents for eligibility, semantic-diff review,
focused metrics, and terminal verification. Expected outputs are package-local
review, verification, compact metric, and command-evidence artifacts. Write
access is read-only unless explicitly assigned a bounded change inside the
declared write set. Heavy global gates remain owned by the master ExecPlan.

## Scope

In scope: characterization and behavior-preserving private helper extraction in
`checkpoint_mirror.rs`, plus package evidence. Out of scope: checkpoint schema,
environment contract, policy thresholds, consumer changes, other production
modules, and campaign-global TESTGATE.

## Declared Write Set

- `crates/openwepp-gate-planner/src/checkpoint_mirror.rs`
- `docs/work-packages/20260721-cqr-testgate-recovery-03-checkpoint-mirror-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`

## Phase Plan

1. Validate exact rows, eligibility, and pre-production baseline.
2. Bind existing mirror/root-rejection characterization.
3. Extract private helpers without changing validation or publication order.
4. Measure matching production coverage/regions and CRAP.
5. Complete dual review, dual verification, disposition, and prompt archival.

## Exit Criteria

- Both original rows and every extracted helper are CRAP at most 30.
- Root/path/error precedence, file bytes, canonical JSON, and consumer behavior
  remain equivalent.
- Dual review and verification pass; no package-local gate remains open.

## Security Impact Gate

- security_impact: moderate
- dedicated_security_review_required: no
- rationale: fail-closed filesystem confinement; dual review checks precedence.
