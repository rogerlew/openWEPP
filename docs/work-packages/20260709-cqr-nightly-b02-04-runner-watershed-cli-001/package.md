# CQR Nightly Batch 02, Target 04 — Watershed CLI

Package: `20260709-cqr-nightly-b02-04-runner-watershed-cli-001`
Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-COVERAGE-TESTABILITY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
Target rank: `4` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce each eligible target production function above CRAP `30` to `<= 30`, or
close it with a documented ADR-0021 disposition. Preserve CLI argument meaning,
runfile grammar, manifest/HBP consumption, topology construction, typed CLI
errors, output/publication identity, unit conversion, groundwater authority,
and all fail-closed guards.

## Scope and Write Set

In scope: target-local characterisation tests, private CLI/parser/validation
helper extraction, and this package's artifacts/prompt. Out of scope: science,
formula, threshold, contract, grammar, serialization, public CLI, output, or
fail-closed behavior changes.

Intended write set:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-04-runner-watershed-cli-001/**`
- `docs/work-packages/README.md` after closure

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- target module and focused watershed CLI contract tests

`SC-SYSTEM-001`, `SC-ROUTE-001`, and `SC-GWBASEFLOW-001` bind the watershed
assembly surfaces that this CLI validates and dispatches. This package is a
structural refactor only and may not weaken their typed hard-fail behavior.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and watershed CLI output-identity checks. Expected
outputs are package-local review, verification, gate, and metric artifacts.
Write access is read-only unless a subagent receives an explicit bounded fix
assignment in this package's target module, declared test, or artifact path.

Subagent requirement: spawn `comparator_suite_runner` for every heavy
full-workspace coverage, CRAP, comparator, clippy, full-nextest, or deny run.
Do not run those heavy gates on the parent model unless the runner is
unavailable; record command-level evidence before any permitted local fallback.

## Commit and Phase Gates

The scaffold commit must precede target production or test edits. Then execute:

1. Record selected rows, baseline CRAP/LCOV, required-reading map, and CLI
   behavior oracle for every affected argument, runfile, manifest, and output
   branch.
2. Add characterisation before decomposition when insufficient; record ADR-0021
   tier, line/region thresholds, per-function floor, and obligation-to-test
   binding.
3. Extract only cohesive private branches while preserving command ordering,
   path resolution, parse/validation ordering, units, typed errors, publication,
   and fail-closed authority semantics.
4. Record after metrics, output/numeric identity, line-count governance,
   focused gates, `git diff --check`, documentation lint, format, delegated
   workspace clippy, full nextest, and deny.
5. Complete dual review, finding disposition, dual verification, and a
   completion or hold commit before target 05 begins.

## Hold Rules and Exit Criteria

A local hold rolls back only this package's target implementation/test edits,
records the blocker, attempted in-envelope route, rollback proof, and concrete
first follow-on in `artifacts/hold-legitimacy-audit.md`, then commits evidence.
Global tooling, baseline, dirty-overlap, or shared-identity blockers stop batch
02. Completion requires target CRAP closure or explicit disposition, ADR-0021
closure, real-consumer CLI behavior proof, all current-scope gates, dual review,
dual verification, line-count governance, and a completion commit.
