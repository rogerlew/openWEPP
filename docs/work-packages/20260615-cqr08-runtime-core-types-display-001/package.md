# CQR08 - Runtime Core Error Display Function-Length Refactor

Status: complete

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs` so
`HillslopeRuntimeInputError`'s `fmt::Display` implementation no longer needs
its `#[allow(clippy::too_many_lines)]` suppression while preserving stable error
codes, display text, typed variant semantics, runtime projection guard behavior,
and public API behavior.

## Rationale

`00_core_types.rs` owns the runtime input error taxonomy for the hillslope
runtime seam. The target file is below line-count governance thresholds, but the
`Display` implementation concentrates every error message in one suppressed
function. This package reduces localized lint debt by grouping display
formatting into private helper functions only.

## Quality Dimension

- Dimension: function-length / lint-debt burndown for
  `HillslopeRuntimeInputError::fmt`.
- Closure metric: remove the target `#[allow(clippy::too_many_lines)]` and keep
  workspace clippy green under `-D warnings`.
- Supporting metrics: record before/after function length, target line count,
  target coverage, and target CRAP rows.

## Included Scope

- Test-first characterization for representative `HillslopeRuntimeInputError`
  display strings and stable codes inside the runtime-input test module.
- Private helper extraction inside `00_core_types.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No public API change for `HillslopeRuntimeInputError`, `code()`, or any
  runtime-input builder.
- No error-code, display-string, variant, field, guard, threshold, alias,
  symbol, runtime projection, parser, or contract-authority change.
- No changes to process-physics math, runtime projection semantics, or
  fail-closed behavior.
- No module/file split.
- No unrelated naming cleanup, dead-code deletion, dependency change, or broad
  lint ratchet outside the target function.

## Conservation / Runtime-Seam Acceptance Gate

This file does not publish conservation outputs directly, but it is a
contract-sensitive runtime-seam error surface. Because this package is
behavior-preserving, the current-scope acceptance gate is frozen error-surface
equivalence:

- preserve each stable `HS-RUNTIME-E-*` code exactly;
- preserve each display string exactly;
- preserve typed variants and field names exactly;
- preserve runtime projection guard behavior by leaving producers untouched;
- stop and hold if any production edit requires changing an error code, display
  text, guard, threshold, symbol, alias, or public API.

Independent numeric reconstruction is not in scope because no numeric formula
or runtime projection path is intentionally changed.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr08-quality-plan-report.md`
   - `artifacts/cqr08-public-api-surface-parity-report.md`
   - `artifacts/cqr08-function-length-before.md`
   - `artifacts/cqr08-function-length-after.md`
   - `artifacts/cqr08-crap-before.md`
   - `artifacts/cqr08-crap-after.md`
   - `artifacts/cqr08-coverage-closure.md`
   - `artifacts/cqr08-error-surface-equivalence.md`
   - `artifacts/cqr08-numeric-equivalence.md`
   - `artifacts/cqr08-implementation-and-test-evidence.md`
   - `artifacts/cqr08-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr08_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260615-cqr08-runtime-core-types-display-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/core_types.rs`

## Dependencies

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260615-cqr08-runtime-core-types-display-001/package.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`

## Phase Plan

### Phase A - Intake, Metric Baseline, and Surface Freeze

- Capture line/function-length baseline for `00_core_types.rs`.
- Capture target `#[allow(clippy::too_many_lines)]` census.
- Capture before coverage/LCOV and CRAP evidence for the target module.
- Record public API surface parity expectations and runtime-seam protected
  boundaries.

### Phase B - Precondition and Focused Characterization

- Run existing focused runtime-input tests before edits:
  `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests -- --nocapture`.
- Add targeted display/code characterization before production decomposition.
- Run focused runtime-input tests again after characterization and before
  production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive display formatting groups into private helpers in the same
  file.
- Preserve every `write!` format string, argument order, `self.code()` use, and
  match-arm behavior.
- Remove the obsolete `#[allow(clippy::too_many_lines)]` only after the target
  function is below the lint threshold.

### Phase D - Validation and Evidence

- Run focused runtime-input tests after the refactor.
- Run the required Rust closure loop:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Re-run coverage/LCOV and `cargo-crap`; current-scope helpers must remain at or
  below the ADR-0021 CRAP bound.
- Run markdown-doc lint for the package and catalog.
- Record command outcome and explicit exit code for every gate.

### Phase E - Review, Verification, and Disposition

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts, including gate legitimacy,
  error-surface equivalence, lint-ratchet, runtime-seam boundary, and
  line-count-governance checks.
- Complete disposition and worker handoff.

## Exit Criteria

- `HillslopeRuntimeInputError::fmt` no longer has
  `#[allow(clippy::too_many_lines)]`.
- Workspace clippy passes under `-D warnings` without adding a replacement
  function-length suppression.
- Focused display/code characterization passes before and after the production
  refactor.
- Error-surface equivalence is preserved; no error code, display string, typed
  variant, field, guard threshold, symbol, alias, or public API changes are
  introduced.
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
- rationale: private Rust display-helper extraction and focused unit
  characterization only; no new subprocess, network, serialization format,
  unsafe, dependency, or public API.
