# CQR31 - Runner Output/Climate Complexity Closure

Status: complete-with-warnings

Package type: code-quality refactor

## Objective

Close the current CQR31 target in
`crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`,
originally tracked as rank 25 with snapshot CRAP `252`, CC `76`, and coverage
`69%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The target file owns runner-side hillslope output, HBP payload, climate-summary,
and WB13 publication helper behavior. Changes must preserve public API,
published output schemas, output formulas, symbols, units, parser
compatibility, error IDs/messages, row ordering, and existing contract behavior.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Characterization coverage before production refactor when needed.
- Behavior-preserving private decomposition of the scoped metric target only.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No output schema, row-key, unit, symbol, filename, parquet, HBP, or JSON
  payload changes.
- No public API, typed error, parser compatibility, climate projection, runoff
  routing, WB13 publication formula, or science-contract behavior changes.
- No edits to unrelated runner scheduler, intake lane setup, or kernel crates
  unless required only for characterization.

## Intended Write Set

- `docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
  only if live metrics require behavior-preserving refactor.
- Focused tests only if characterization coverage is required before refactor.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`

## Exit Criteria

- Current CQR31 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- Public API, error behavior, output schemas, units, symbols, formulas, row
  ordering, and parser compatibility are preserved.
- Required closure gates are run and recorded.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.
