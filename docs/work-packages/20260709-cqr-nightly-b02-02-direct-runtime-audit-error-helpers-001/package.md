# CQR Nightly Batch 02, Target 02 — Direct Runtime Audit/Error Helpers

Package: `20260709-cqr-nightly-b02-02-direct-runtime-audit-error-helpers-001`
Status: `SCAFFOLDED-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs`
Target rank: `2` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce each eligible production function above CRAP `30` in the target to
`<= 30`, or close in a documented local hold when behavior-preserving CQR cannot
safely meet ADR-0021. Preserve every typed direct-runtime error, exact display
string, fail-closed behavior, API, numeric expression order, and output meaning.

## Scope and Write Set

In scope: target-local characterization tests, private `Display` helper
extraction, target module test code, and this package's artifacts/prompt. Out of
scope: science or contract changes, new physics, thresholds, serialization,
public API changes, or behavior changes.

Intended write set:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-02-direct-runtime-audit-error-helpers-001/**`
- `docs/work-packages/README.md` after closure

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- the target module and its existing direct-runtime tests

`SC-WATBAL-001` requires typed hard errors for invalid runtime domain,
precondition, and closure states. `SC-OFEROUTE-001` requires missing or invalid
routing operands to fail closed. Characterization must preserve all variants and
their exact diagnostic strings.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and direct-runtime error identity checks. Expected
outputs are the package-local review, verification, gate, and metric artifacts.
Write access is read-only unless a subagent receives an explicit bounded fix
assignment in this package's target module or artifact directory.

Subagent requirement: spawn `comparator_suite_runner` for every heavy
full-workspace coverage, CRAP, comparator, clippy, full-nextest, or deny run.
Do not run those heavy gates on the parent model unless the runner is unavailable;
record command-level evidence before any permitted local fallback.

## Commit and Phase Gates

The scaffold commit must precede target production or test edits. Then execute:

1. Record the selected row, baseline CRAP/LCOV, required-reading map, and
   existing behavior oracle.
2. Add characterization coverage before decomposition when it is insufficient;
   record ADR-0021 tier, line/region thresholds, per-function floor, and any
   obligation-to-test binding.
3. Extract only cohesive private `Display` branches while preserving exact typed
   errors, string bytes, and arithmetic order. Re-run focused tests and target
   CRAP after each meaningful edit.
4. Record after metrics, error/API identity, line-count governance, focused
   gates, `git diff --check`, documentation lint, format, delegated workspace
   clippy, full nextest, and deny.
5. Complete dual review, finding disposition, dual verification, and either a
   completion or hold commit before target 03 begins.

## Hold Rules and Exit Criteria

A local hold rolls back only this package's target production/test edits, records
the blocker, attempted in-envelope route, rollback proof, and a concrete first
follow-on in `artifacts/hold-legitimacy-audit.md`, then commits the evidence.
Global tooling, baseline, dirty-overlap, or shared-identity blockers stop batch
02. Completion requires target CRAP closure or an explicit disposition, ADR-0021
closure for changed tests, behavior proof, all current-scope gates, dual review,
dual verification, line-count governance, and a completion commit.
