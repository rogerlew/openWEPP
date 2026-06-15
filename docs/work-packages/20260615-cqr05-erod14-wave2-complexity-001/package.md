# CQR05 - EROD14 Wave-2 Complexity Refactor

Status: complete-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod14.rs`
so the EROD14 Wave-2 runtime kernel no longer concentrates activation,
payload validation, case semantics, class loading, enrichment/reproportion
math, enrichment-ratio calculation, and Wave-3 payload export in one
high-complexity function.

## Rationale

ADR-0021 names `run_erod14_wave2` as a CRAP/cyclomatic-complexity backlog item.
The target file is kernel sediment/enrichment code under `SC-SED-001` EROD14
Wave-2 authority and currently carries a `#[allow(clippy::too_many_lines)]`
suppression. This package reduces change risk by extracting cohesive private
helpers while preserving runtime behavior, numeric expression grouping, guard
families, symbol names, thresholds, and public crate APIs.

## Quality Dimension

- Dimension: cyclomatic complexity / CRAP reduction for
  `hydrology_phase_erod14.rs`.
- Closure metric: every eligible function in `hydrology_phase_erod14.rs` has
  CRAP `<= 30` after the refactor.
- Supporting lint-ratchet outcome: remove the target-file
  `#[allow(clippy::too_many_lines)]` suppression when the decomposed functions
  satisfy clippy.
- Supporting metrics: record before/after function length and target coverage.

## Included Scope

- Private helper extraction inside `hydrology_phase_erod14.rs`.
- Private context/state structs if needed to pass EROD14 state explicitly.
- Focused EROD14 contract tests before and after refactor.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No science-contract amendments.
- No EROD14 formula, constant, threshold, tolerance, case predicate, guard
  strictness, symbol name, or writeback field change.
- No public crate API changes.
- No edits to runner activation, runtime projection, EROD13, EROD15, or
  watershed routing code.
- No module/file split beyond this file; this package targets intra-module
  function decomposition only.
- No unrelated naming cleanup, dead-code deletion, or broad lint ratchet outside
  the target file.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod14.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr05-quality-plan-report.md`
   - `artifacts/cqr05-public-api-surface-parity-report.md`
   - `artifacts/cqr05-function-length-before.md`
   - `artifacts/cqr05-function-length-after.md`
   - `artifacts/cqr05-crap-before.md`
   - `artifacts/cqr05-crap-after.md`
   - `artifacts/cqr05-coverage-closure.md`
   - `artifacts/cqr05-numeric-equivalence.md`
   - `artifacts/cqr05-implementation-and-test-evidence.md`
   - `artifacts/cqr05-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr05_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod14.rs`

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
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`

## Phase Plan

### Phase A - Intake, Metric Baseline, and Surface Freeze

- Capture line/function-length baseline for `hydrology_phase_erod14.rs`.
- Capture public/internal caller surface expectations for `run_erod14_wave2`.
- Capture before coverage/LCOV and CRAP evidence for the target module.
- Record package scope and write-set boundaries.

### Phase B - Precondition and Focused Characterization

- Run focused EROD14 contract tests before edits.
- If focused characterization fails before edits, stop and record a blocker.
- If target coverage is below science-tier closure thresholds, record a scoped
  coverage hold unless focused characterization must be added to support a
  behavior-preserving extraction.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive EROD14 stages and branch bodies into private helpers.
- Preserve guard order where externally observable through returned typed guard
  errors.
- Preserve exact arithmetic expression grouping and accumulation order.
- Remove target-file `too_many_lines` suppression only when clippy accepts the
  decomposed functions.

### Phase D - Validation and Evidence

- Run focused EROD14 checks after the refactor.
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

- Every eligible function in `hydrology_phase_erod14.rs` has CRAP `<= 30`.
- `run_erod14_wave2` is decomposed into cohesive private helpers without
  changing EROD14 behavior, typed guard IDs, symbol names, branch predicates,
  arithmetic expression grouping, writeback order, or public APIs.
- Focused EROD14 contract characterization passes before and after refactor.
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

## Final Disposition

- Completed: EROD14 Wave-2 helper extraction is implemented in the target file.
- Completed: every measured target-module function has CRAP `<= 23.0`, meeting
  the package closure target of `<= 30`.
- Completed: `#[allow(clippy::too_many_lines)]` was removed from the target
  file and the workspace clippy gate passed with `-D warnings`.
- Completed: public crate-visible surface is unchanged.
- WARN: target-file coverage improved but remains below the science-tier
  threshold: line coverage `79.59183673469387%`, region coverage
  `82.99737072743207%`. This is recorded as a scoped coverage hold because the
  package objective was behavior-preserving code-quality decomposition, not
  module-test expansion.
