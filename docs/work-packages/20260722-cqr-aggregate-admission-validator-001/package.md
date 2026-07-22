# CQR Aggregate Admission Validator

Package ID: `20260722-cqr-aggregate-admission-validator-001`

Queue ID: `TESTGATE-CQR-AGGREGATE-VALIDATOR-01`

Status: `ACTIVE`

## Objective

Close RTR-031 with an executable pre-implementation validator that proves a
multi-package CQR module binds an earlier immutable `ACTIVE`/`READY` aggregate
authority whose scaffold write set covers the module's intended paths.

## Authority And Scope

The predecessor documentation package stopped at a legitimate immutable-write-
set boundary after review found its control was prose-only. This package owns
the validator, focused Python contract tests, canonical usage docs, process
bindings, recovery evidence, and durable closure. It may not weaken package
validation, authorize retroactive widening, or alter production/science logic.

## Declared Write Set

- `tools/local_ci/check_cqr_aggregate_admission.py`
- `tools/local_ci/README.md`
- `tests/python/test_cqr_aggregate_admission.py`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/work-packages/templates/cqr-nightly-package.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
- `docs/work-packages/20260722-testgate-cqr-aggregate-admission-001/**`
- `docs/work-packages/20260722-cqr-aggregate-admission-validator-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Commit this scaffold before tool/test edits.
2. Implement the fail-closed validator and positive/negative Git-fixture tests.
3. Bind the exact validator command into the standard, process, template, and
   tool documentation.
4. Run focused validation, obtain dual independent review, and close RTR-031
   durably only at the exact correction commit.
5. Return to one delegated changed-head recovery qualification attempt.

## Exit Criteria

- The validator rejects missing, late, non-active, mismatched, mutated, or
  insufficient aggregate authority and accepts a valid earlier scaffold.
- Focused tests, scoped docs lint, formatting, and diff hygiene pass.
- Package validation from this immutable scaffold is READY with zero
  unauthorized paths.
- Dual independent review passes and RTR-031 is durably closed.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers, one
comparator runner for the final recovery qualification, and two independent
read-only terminal verifiers. Outputs are package-local reviews/verifications
and retained external qualification evidence. Write access is read-only except
for the comparator's ignored evidence root. Do not push, deploy, switch
branches, dispatch workflows, run HEAVY locally, or repeat unchanged gates.
