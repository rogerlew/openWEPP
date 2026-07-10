# CQR Nightly Batch 02, Target 05 — Totalwatsed3 CLI

Package: `20260709-cqr-nightly-b02-05-runner-totalwatsed3-001`
Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs`
Target rank: `5` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible target production function above CRAP `30` to `<= 30`, or
record an ADR-0021 disposition. Preserve CLI argument semantics, aggregate and
per-hillslope input discovery, required/optional fail-closed behavior, output
publication, row identity, and units.

## Scope and Write Set

In scope: target-local characterisation tests, private CLI helper extraction,
and package artifacts/prompt. Out of scope: totalwatsed3 formulas, input/output
schemas, thresholds, public CLI, units, serialization, or fail-closed behavior.

Intended write set:

- `crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-05-runner-totalwatsed3-001/**`
- `docs/work-packages/README.md` after closure

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- target module and `totalwatsed3_cli_contract.rs`

`SC-SYSTEM-001` protects watershed handoff and publication identity. This CQR
package is structural only and may not alter its consumer/error semantics.

## Subagent Authorization

This package authorizes bounded review, verification, and comparator/closure
delegation for behavior-preserving CQR. Write access is read-only unless an
explicit assignment limits it to this target, declared test, or package
artifacts. Spawn a comparator suite runner for heavy coverage/CRAP, workspace
clippy, full-nextest, deny, or comparator work.

## Commit and Exit Gates

The scaffold commit precedes all target production/test edits. Record baseline
and reading map, add public CLI characterisation before decomposition, preserve
discovery/error/output ordering, record after metrics and line governance, run
focused and delegated workspace gates, complete dual review/verification, and
commit a completion or local hold before target 06 begins.
