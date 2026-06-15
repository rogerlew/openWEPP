# CQR06 - WB19 Lateral/Drainage Complexity Refactor

Status: complete-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Execution Summary

Completed on 2026-06-15. The target module was refactored in place into
private WB19 lateral, drainage, writeback, and WB14 conductivity-adjustment
helpers. The public crate-visible functions and signatures are unchanged.

Primary closure metric passed: target-file max CRAP decreased from
`300.2455501433063` to `26.541362973760947`, so every eligible target-module
function is now `<= 30`.

Warnings:

- Target-file line count is now `2527`, above the `2000` WARN threshold and
  below the `3000` block threshold. Follow-on intent: defer a future file split
  until behavior-preserving module extraction is explicitly authorized.
- Target-file LCOV line coverage improved from `65.41%` to `80.02%`, but remains
  below the science-tier `>= 90%` line target. This CQR package records a scoped
  coverage hold rather than adding broader behavioral tests.

## Objective

Decompose
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`
so WB19 lateral-transfer, drainage, and top-layer conductivity adjustment logic
no longer concentrate domain loading, guard validation, lane iteration,
subsurface-drainage geometry, lateral diagnostics, and writeback construction in
high-complexity functions.

## Rationale

ADR-0021 names `run_lateral_transfer` as a CRAP/cyclomatic-complexity backlog
item. The target file also contains `run_drainage`,
`wb14_load_top_two_layer_ksatadj_metrics`, and
`resolve_wb14_effective_soil_conductivity`, all carrying
`#[allow(clippy::too_many_lines)]`. The file implements contract-bearing WB19
subsurface hydrology under `SC-SUBHYD-001`, so this package reduces maintenance
risk through private helper extraction while preserving physics, guard posture,
symbol names, thresholds, and public crate APIs.

## Quality Dimension

- Dimension: cyclomatic complexity / CRAP reduction for
  `hydrology_phase_lateral_drainage.rs`.
- Closure metric: every eligible function in
  `hydrology_phase_lateral_drainage.rs` has CRAP `<= 30` after the refactor.
- Supporting lint-ratchet outcome: remove target-file
  `#[allow(clippy::too_many_lines)]` suppressions when decomposed functions
  satisfy clippy.
- Supporting metrics: record before/after function length and target coverage.

## Included Scope

- Private helper extraction inside `hydrology_phase_lateral_drainage.rs`.
- Private context/state structs if needed to pass WB19 state explicitly.
- Focused WB19 lateral/drainage contract tests before and after refactor.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No science-contract amendments.
- No WB19 lateral/drainage formula, constant, threshold, tolerance, lane
  predicate, guard strictness, symbol name, unit-conversion path, or writeback
  field-order change.
- No public crate API changes.
- No edits to runtime projection, scheduler ordering, WB17/WB18/WB12/WB13
  phases, watershed routing code, or integration tests unless a focused
  characterization failure proves a strictly mechanical reconciliation is
  required.
- No module/file split beyond this file; this package targets intra-module
  function decomposition only.
- No unrelated naming cleanup, dead-code deletion, or broad lint ratchet outside
  the target file.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr06-quality-plan-report.md`
   - `artifacts/cqr06-public-api-surface-parity-report.md`
   - `artifacts/cqr06-function-length-before.md`
   - `artifacts/cqr06-function-length-after.md`
   - `artifacts/cqr06-crap-before.md`
   - `artifacts/cqr06-crap-after.md`
   - `artifacts/cqr06-coverage-closure.md`
   - `artifacts/cqr06-numeric-equivalence.md`
   - `artifacts/cqr06-implementation-and-test-evidence.md`
   - `artifacts/cqr06-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr06_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260615-cqr06-lateral-drainage-complexity-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`

## Dependencies

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`

## Phase Plan

### Phase A - Intake, Metric Baseline, and Surface Freeze

- Capture line/function-length baseline for
  `hydrology_phase_lateral_drainage.rs`.
- Capture public/internal caller surface expectations for crate-visible target
  functions.
- Capture before coverage/LCOV and CRAP evidence for the target module.
- Record package scope and write-set boundaries.

### Phase B - Precondition and Focused Characterization

- Run focused WB19 lateral/drainage contract tests before edits.
- If focused characterization fails before edits, stop and record a blocker.
- If target coverage is below science-tier closure thresholds, record a scoped
  coverage hold unless focused characterization must be added to support a
  behavior-preserving extraction.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive WB19 lateral, drainage, conductivity-adjustment, and
  writeback stages into private helpers.
- Preserve guard order where externally observable through returned typed guard
  errors.
- Preserve exact arithmetic expression grouping and accumulation order.
- Remove target-file `too_many_lines` suppressions only when clippy accepts the
  decomposed functions.

### Phase D - Validation and Evidence

- Run focused WB19 checks after the refactor.
- Run the required Rust closure loop:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Re-run coverage/LCOV and `cargo-crap`; target-module CRAP rows must be
  `<= 30`.
- Record command outcome and explicit exit code for every gate.

### Phase E - Review, Verification, and Disposition

- Complete dual review artifacts with finding disposition.
- Complete dual verification artifacts, including gate legitimacy, numeric
  equivalence, metric target, and line-count-governance checks.
- Complete disposition and worker handoff.

## Exit Criteria

- Every eligible function in `hydrology_phase_lateral_drainage.rs` has CRAP
  `<= 30`.
- WB19 functions are decomposed into cohesive private helpers without changing
  behavior, typed guard IDs, symbol names, branch predicates, arithmetic
  expression grouping, unit conversions, writeback order, or public APIs.
- Focused WB19 contract characterization passes before and after refactor.
- Target coverage does not regress below science-tier closure thresholds
  (`>= 90%` line and region) unless a reviewed, scoped hold is recorded.
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
- rationale: private Rust kernel helper extraction only; no new subprocess,
  network, serialization format, unsafe, dependency, or public API.
