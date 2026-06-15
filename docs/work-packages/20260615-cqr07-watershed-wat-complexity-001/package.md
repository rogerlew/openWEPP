# CQR07 - Watershed WAT Reader Function-Length Refactor

Status: complete-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose
`crates/openwepp-runner/src/watershed_wat.rs` so `read_batch_into` no longer
needs its `#[allow(clippy::too_many_lines)]` suppression while preserving WAT
reader, aggregation, optional-column, fail-closed, and public publication
behavior.

## Rationale

`watershed_wat.rs` builds daily watershed publication rows from sibling WAT
parquet outputs. The module is below line-count governance thresholds, but
`read_batch_into` concentrates column discovery, row validation, day-key
construction, WAT value extraction, optional-column defaults, and row assembly in
one suppressed function. This package reduces localized maintenance risk through
private helper extraction only.

## Quality Dimension

- Dimension: function-length / lint-debt burndown for `read_batch_into`.
- Closure metric: remove the module's `#[allow(clippy::too_many_lines)]` from
  `read_batch_into` and keep workspace clippy green under `-D warnings`.
- Supporting metrics: record before/after function length, target line count,
  target coverage, and target CRAP rows.

## Included Scope

- Test-first characterization for the existing WAT batch reader behavior inside
  `watershed_wat.rs`.
- Private helper extraction inside `watershed_wat.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No WAT aggregation formula, operand, unit, denominator, area-weighting,
  outlet-lateral-flow, optional-column default, required-column, fail-closed
  guard, or publication-field change.
- No public crate API change for
  `build_watershed_daily_rows_from_wat` or `WatershedWatPublicationError`.
- No parquet schema compatibility change beyond exercising the existing
  aliases/defaults in tests.
- No science-contract amendment.
- No module/file split.
- No unrelated naming cleanup, dead-code deletion, dependency change, or broad
  lint ratchet outside the target function.

## Conservation / Publication Acceptance Gate

This module aggregates conservation-sensitive water-balance publication fields.
Because this package is behavior-preserving, the current-scope conservation gate
is frozen-output equivalence:

- preserve existing operands and units exactly;
- preserve area weighting and volume conversion expressions exactly;
- preserve optional absent/all-null column defaults exactly;
- preserve row ordering by `DayKey` exactly;
- stop and hold if any production edit requires changing an output formula,
  operand source, denominator, guard threshold, or alias mapping.

Independent closure reconstruction is not in scope because no output formula or
operand lineage is intentionally changed. The package must record a numeric
equivalence statement and focused characterization tests for the reader path.

## Deliverables

1. Source refactor:
   - `crates/openwepp-runner/src/watershed_wat.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr07-quality-plan-report.md`
   - `artifacts/cqr07-public-api-surface-parity-report.md`
   - `artifacts/cqr07-function-length-before.md`
   - `artifacts/cqr07-function-length-after.md`
   - `artifacts/cqr07-crap-before.md`
   - `artifacts/cqr07-crap-after.md`
   - `artifacts/cqr07-coverage-closure.md`
   - `artifacts/cqr07-wat-publication-boundary.md`
   - `artifacts/cqr07-numeric-equivalence.md`
   - `artifacts/cqr07-implementation-and-test-evidence.md`
   - `artifacts/cqr07-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr07_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/watershed_wat.rs`

## Dependencies

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`

## Phase Plan

### Phase A - Intake, Metric Baseline, and Surface Freeze

- Capture line/function-length baseline for `watershed_wat.rs`.
- Capture target `#[allow(clippy::too_many_lines)]` census.
- Capture before coverage/LCOV and CRAP evidence for the target module.
- Record public API surface parity expectations and conservation/publication
  protected boundaries.

### Phase B - Precondition and Focused Characterization

- Run existing focused module tests before edits:
  `cargo test -p openwepp-runner watershed_wat::tests -- --nocapture`.
- Add targeted reader characterization in `watershed_wat.rs` before production
  decomposition if baseline coverage does not exercise `read_batch_into`.
- Run focused module tests again after characterization and before production
  refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract WAT batch column loading into a private batch-column helper.
- Extract row assembly into private helpers for area validation, day-key
  construction, WAT value construction, and optional ID loading.
- Preserve statement order, expression grouping, guard behavior, alias lookup,
  and optional all-null handling.
- Remove the obsolete `#[allow(clippy::too_many_lines)]` only after the function
  is below the lint threshold.

### Phase D - Validation and Evidence

- Run focused `watershed_wat` tests after the refactor.
- Run the required Rust closure loop:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Re-run coverage/LCOV and `cargo-crap`; target-module CRAP rows must not
  regress above the ADR-0021 bound.
- Run markdown-doc lint for the package and catalog.
- Record command outcome and explicit exit code for every gate.

### Phase E - Review, Verification, and Disposition

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts, including gate legitimacy,
  numeric equivalence, lint-ratchet, conservation/publication boundary, and
  line-count-governance checks.
- Complete disposition and worker handoff.

## Exit Criteria

- `read_batch_into` no longer has `#[allow(clippy::too_many_lines)]`.
- Workspace clippy passes under `-D warnings` without adding a replacement
  function-length suppression.
- Focused WAT reader/aggregation characterization passes before and after the
  production refactor.
- Numeric equivalence is preserved by helper extraction; no WAT output formula,
  operand source, threshold, alias, optional default, or float expression
  grouping changes are introduced.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched `.rs` file is at or above 2000 lines unless a WARN disposition is
  recorded with owner and follow-on intent.
- No review finding remains undispositioned.
- Gate Evidence Non-Deferral Rule is satisfied for every current-scope gate.

## Review and Verification

This package requires dual independent local review and dual independent local
verification artifacts. Subagent spawning is not required for closure; if
separate subagents are unavailable or not authorized in the current tool policy,
the executing agent must perform equivalent independent local reviews and record
that path.

## Instruction Precedence

Package-required gates override ambient test/validation-skip guidance,
including: `UNLESS you are explicitly requested to do so, NEVER run tests or
validate your work.`

## Security Impact Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: private Rust helper extraction and focused unit characterization
  only; no new subprocess, network, serialization format, unsafe, dependency, or
  public API.

## Execution Summary

Static: `read_batch_into` was decomposed into private column-loading and
row-construction helpers in
`crates/openwepp-runner/src/watershed_wat.rs`. The obsolete
`#[allow(clippy::too_many_lines)]` suppression was removed. Public function and
error surfaces are unchanged.

Ran: focused characterization, metrics, workspace Rust gates, dependency policy,
markdown lint, and whitespace checks passed. `read_batch_into` line span changed
from lines `380-524` in `HEAD` to lines `482-495` in the working tree, and CRAP
changed from `4830.0` to `4.0`.

WARN: after LCOV for the target file is `665/877` lines and `46/59` functions,
below the science-tier closure threshold. CRAP rows above `30` remain for
pre-existing out-of-scope entry/display functions:
`WatershedWatPublicationError::fmt`, `build_watershed_daily_rows_from_wat`, and
`read_wat_file_into`.
