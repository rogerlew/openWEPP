# CQR Nightly Batch 02, Target 03 — Soil Parser

Package: `20260709-cqr-nightly-b02-03-soil-parser-001`
Status: `SCAFFOLDED-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-input-contract/src/parsers/soil.rs`
Target rank: `3` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce each eligible production function above CRAP `30` in the target to
`<= 30`, or close in a documented local hold when behavior-preserving CQR cannot
safely meet ADR-0021. Preserve every `.sol` grammar branch, datver family,
typed error, units, fail-closed validation, output profile, and numeric order.

## Scope and Write Set

In scope: target-local characterization tests, private parser helper extraction,
removal of target-local lint debt when structurally justified, and this package's
artifacts/prompt. Out of scope: grammar, science/contract changes, thresholds,
datver policy, serialization, public API, runtime projection, or behavior
changes.

Intended write set:

- `crates/openwepp-input-contract/src/parsers/soil.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-03-soil-parser-001/**`
- `docs/work-packages/README.md` after closure

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- the target module and its existing parser tests

`SC-INFILE-SOIL-001` governs exact datver-specific `.sol` grammar, typed errors,
and fail-closed parser behavior. `SC-SOIL-001` supplies the downstream
soil-domain invariants that the parser's typed fields must not weaken.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and `.sol` parse identity checks. Expected outputs
are package-local review, verification, gate, and metric artifacts. Write access
is read-only unless a subagent receives an explicit bounded fix assignment in
this package's target module or artifact directory.

Subagent requirement: spawn `comparator_suite_runner` for every heavy
full-workspace coverage, CRAP, comparator, clippy, full-nextest, or deny run.
Do not run those heavy gates on the parent model unless the runner is unavailable;
record command-level evidence before any permitted local fallback.

## Commit and Phase Gates

The scaffold commit must precede target production or test edits. Then execute:

1. Record selected row, baseline CRAP/LCOV, required-reading map, and parser
   behavior oracle across every affected datver branch.
2. Add characterization coverage before decomposition when insufficient; record
   ADR-0021 tier, line/region thresholds, per-function floor, and
   obligation-to-test binding.
3. Extract only cohesive private parser branches while preserving token order,
   numeric conversion order, datver arity, typed errors, and exact messages.
4. Record after metrics, parse/API/numeric identity, line-count governance,
   focused gates, `git diff --check`, documentation lint, format, delegated
   workspace clippy, full nextest, and deny.
5. Complete dual review, finding disposition, dual verification, and either a
   completion or hold commit before target 04 begins.

## Hold Rules and Exit Criteria

A local hold rolls back only this package's target production/test edits, records
the blocker, attempted in-envelope route, rollback proof, and a concrete first
follow-on in `artifacts/hold-legitimacy-audit.md`, then commits the evidence.
Global tooling, baseline, dirty-overlap, or shared-identity blockers stop batch
02. Completion requires target CRAP closure or explicit disposition, ADR-0021
closure for changed tests, parse behavior proof, all current-scope gates, dual
review, dual verification, line-count governance, and a completion commit.
