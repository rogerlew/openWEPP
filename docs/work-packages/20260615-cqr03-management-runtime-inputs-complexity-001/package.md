# CQR03 - Management Runtime Inputs Complexity Refactor

Status: complete

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
so `build_hillslope_pl_runtime_surfaces_from_management` and the associated
primary live-canopy assimilation helper no longer concentrate management
runtime projection, schedule validation, initial-state seeding, yearly slot
projection, drain projection, annual/perennial branch projection, and primary
alias assimilation in long high-risk functions.

## Rationale

`01_management.rs` is the strict `PL-MAN-SEAM-001` parser-to-runtime
projection surface. It already has focused runtime-input and parser/runtime seam
coverage, but the implementation contains two `#[allow(clippy::too_many_lines)]`
functions and a high-CRAP projection dispatcher. The code-quality target is a
behavior-preserving intra-module decomposition into cohesive private helpers
without changing projected symbol names, map insertion order where observable,
typed error variants/codes, guard thresholds, arithmetic expression grouping,
public entrypoints, or PL runtime seam policy.

## Quality Dimension

- Dimension: cyclomatic complexity / CRAP reduction for
  `runtime_inputs/01_management.rs`.
- Closure metric: every eligible function in
  `runtime_inputs/01_management.rs` has CRAP `<= 30` after the refactor.
- Supporting lint-ratchet outcome: remove obsolete
  `#[allow(clippy::too_many_lines)]` suppressions from this file when the
  decomposed functions satisfy clippy.
- Supporting metrics: record before/after function length and target coverage.

## Included Scope

- Private helper extraction inside `01_management.rs`.
- Private context structs if needed to pass projection-stage state explicitly.
- Focused characterization only if measured pre-refactor coverage is below the
  glue-tier safety-net threshold for this runtime adapter.
- Package catalog update in `docs/work-packages/README.md`.
- Focused runtime-input management tests and required Rust closure gates.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No parser behavior, PL runtime seam policy, public API, symbol naming,
  schedule projection shape, typed error code, or typed error message changes.
- No arithmetic regrouping, threshold changes, fallback defaults, guard
  loosening, or domain canonicalization changes.
- No edits to management parsing, PL hydrology/growth/decomposition kernels, or
  external authority suites.
- No module/file split beyond this file; this package targets intra-module
  function decomposition only.
- No unrelated dead-code deletion, naming cleanup, or broad lint ratchet outside
  the target file.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr03-quality-plan-report.md`
   - `artifacts/cqr03-public-api-surface-parity-report.md`
   - `artifacts/cqr03-function-length-before.md`
   - `artifacts/cqr03-function-length-after.md`
   - `artifacts/cqr03-crap-before.md`
   - `artifacts/cqr03-crap-after.md`
   - `artifacts/cqr03-coverage-closure.md`
   - `artifacts/cqr03-parser-runtime-equivalence.md`
   - `artifacts/cqr03-implementation-and-test-evidence.md`
   - `artifacts/cqr03-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr03_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/management.rs`
- `tests/integration/parser_runtime_seam_integration/runtime_projection_and_management.rs`

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-boundary-contract.md`
- `docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-runtime-adapter-contract.md`
- `docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-runtime-surface-projection-map.md`
- `crates/AGENTS.md`

## Phase Plan

### Phase A - Intake, Metric Baseline, and Surface Freeze

- Capture line/function-length baseline for `01_management.rs`.
- Capture public runtime-input management API surface expectations.
- Capture before coverage/LCOV and CRAP evidence for the target module.
- Record package scope and write-set boundaries.

### Phase B - Precondition and Focused Characterization

- Run focused runtime-input management tests and parser/runtime seam tests.
- If focused characterization fails before edits, stop and record a blocker
  instead of refactoring under a red precondition.
- If coverage is below glue-tier closure thresholds, add focused
  characterization before production decomposition.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive projection stages from the target functions into private
  helpers in `01_management.rs`.
- Preserve guard order, arithmetic grouping, map keys, typed errors, and public
  runtime-input entrypoints.
- Remove target-file `too_many_lines` suppressions only when clippy accepts the
  decomposed functions.

### Phase D - Validation and Evidence

- Run focused checks after the refactor.
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
- Complete dual verification artifacts, including gate legitimacy,
  parser/runtime equivalence, metric target, and line-count-governance checks.
- Complete disposition and worker handoff.

## Exit Criteria

- Every eligible function in `01_management.rs` has CRAP `<= 30`.
- `build_hillslope_pl_runtime_surfaces_from_management` and primary
  live-canopy assimilation are decomposed into cohesive private helpers without
  changing parser/runtime seam behavior, typed errors, or public runtime-input
  APIs.
- Focused runtime-input management characterization passes before and after the
  refactor.
- Target coverage does not regress below glue-tier closure thresholds
  (`>= 85%` line and region) unless a reviewed, scoped hold is recorded.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched `.rs` file is at or above 2000 lines unless a WARN disposition is
  recorded with owner and follow-on intent.
- No review finding remains undispositioned.
- Gate Evidence Non-Deferral Rule is satisfied for every current-scope gate.

## Subagent Requirement

Subagent authorization: this package explicitly authorizes
spawning/delegating to review and verification subagents for bounded read-only
review of this package's artifacts and source diff. Expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is limited to the package
artifact files. If subagents are unavailable or tool policy does not allow
delegation from this turn, the executing agent must perform equivalent
independent local reviews and record that path.

## Instruction Precedence

Package-required gates override ambient test/validation-skip guidance,
including: `UNLESS you are explicitly requested to do so, NEVER run tests or
validate your work.`

## Security Impact Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: private Rust runtime-input helper extraction only; no new
  subprocess, network, serialization format, unsafe, dependency, or public API.
