# CQR Nightly Batch 02, Target 06 — Irrigation Depletion Parser

Package: `20260709-cqr-nightly-b02-06-irrigation-depletion-001`
Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs`
Target rank: `6` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce all eligible target functions above CRAP `30` to `<= 30`, or record an
ADR-0021 disposition. Preserve irrigation-depletion grammar, datver policy,
strict/compatibility behavior, typed errors/warnings, topology constraints,
period order, units, and fail-closed validation.

## Scope and Write Set

In scope: target-local tests, private parser/error-display extraction, and
package evidence. Out of scope: file grammar, science/contract, units,
thresholds, parser policy, public API, or error behavior changes.

Intended write set:

- `crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs`
- `tests/integration/infile_irrigation_depletion_parser_contract.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-06-irrigation-depletion-001/**`
- `docs/work-packages/README.md` after closure

## Required Reading

- `AGENTS.md`, `crates/AGENTS.md`, `tests/AGENTS.md`,
  `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md`
- CQR ExecPlan, mechanical/CQR guides, ADR-0021, target module, and its
  integration contract test

This is structural parser maintenance only. It may not change datver acceptance,
token arity/field association, record order, exact error IDs/messages, warning
policy, or parser-to-consumer meaning.

## Subagent Authorization and Gates

This package authorizes bounded review, verification, test, and comparator
delegation. Use a comparator suite runner for heavy target coverage/CRAP and
workspace closure gates. Commit scaffold before source/test edits; then require
test-first characterisation, metrics/ADR closure, output identity, dual review,
dual verification, and completion or local hold commit before target 07.
