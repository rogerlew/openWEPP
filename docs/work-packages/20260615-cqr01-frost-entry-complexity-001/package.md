# CQR01 - Frost Entry Complexity Refactor

Status: complete

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
so `compute_active_frost_coupling` no longer needs its
`#[allow(clippy::too_many_lines)]` suppression while preserving frost coupling
numeric behavior, guards, units, public call sites, and publication surfaces.

## Rationale

`frost_entry.rs` was split out by REFACTOR023 and is now below line-count
governance thresholds, but its main active-frost orchestrator remains a very
large function with a local function-length lint suppression. The code-quality
target is a behavior-preserving intra-function decomposition: extract cohesive
validation, layer-loading, thermal-context, hourly-stepping, and outcome
assembly blocks into private helpers without changing formula order or guard
thresholds.

## Quality Dimension

- Dimension: function-length / lint-debt burndown for
  `compute_active_frost_coupling`.
- Closure metric: remove the module's `#[allow(clippy::too_many_lines)]` from
  the function and keep workspace clippy green under `-D warnings`.
- Supporting metric: record before/after `cargo-crap` output for the target
  module and disposition any remaining complexity outside this package's single
  dimension.

## Included Scope

- Private helper extraction inside `frost_entry.rs`.
- Mechanical call-site rewiring inside the same file.
- Focused characterization/gate runs for existing frost coverage.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No process-physics formula, constant, threshold, tolerance, or unit changes.
- No contract amendment; this package is behavior-preserving refactor only.
- No public API shape change for
  `Wb11HydrologyKernel::compute_active_frost_coupling` or
  `Wb11HydrologyKernel::resolve_active_frost_coupling`.
- No new fallback wrappers, silent defaults, unbounded clamping, or guard
  loosening.
- No changes to tests except if a measured precondition proves additional
  characterization coverage is required before the refactor.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/cqr01-quality-plan-report.md`
   - `artifacts/cqr01-public-api-surface-parity-report.md`
   - `artifacts/cqr01-function-length-before.md`
   - `artifacts/cqr01-function-length-after.md`
   - `artifacts/cqr01-crap-before.md`
   - `artifacts/cqr01-crap-after.md`
   - `artifacts/cqr01-numeric-equivalence.md`
   - `artifacts/cqr01-contract-implementation-evidence.md`
   - `artifacts/cqr01-contract-test-implementation-evidence.md`
   - `artifacts/cqr01-preimplementation-contract-gate.md`
   - `artifacts/cqr01-implementation-and-test-evidence.md`
   - `artifacts/cqr01-kernel-profile-compliance-checklist.md`
   - `artifacts/cqr01-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/cqr01_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260615-cqr01-frost-entry-complexity-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`

## Dependencies

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/20260614-refactor023-hillslope-coupling-line-count-split-001/package.md`

## Phase Plan

### Phase A - Intake, Metric Baseline, and Surface Freeze

- Capture line/function-length baseline for `frost_entry.rs`.
- Capture `#[allow(clippy::too_many_lines)]` census for the target module.
- Capture before `cargo-crap` evidence if the tool is available.
- Record public API surface parity expectations.

### Phase B - Precondition and Focused Characterization

- Run the existing focused frost integration suite:
  `cargo test --test clim06_frost_frozen_soil_kernel_contract`.
- If coverage tooling is available in the environment, record target-module
  coverage/LCOV evidence before code edits.
- If focused frost characterization fails before edits, stop and record a
  blocker instead of refactoring under a red precondition.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive blocks from `compute_active_frost_coupling` into private
  helpers in `frost_entry.rs`.
- Preserve expression grouping, statement order, branch order, and validation
  thresholds.
- Keep public method signatures and call sites stable.
- Remove the obsolete `#[allow(clippy::too_many_lines)]` only after the function
  is below the lint threshold.

### Phase D - Validation and Evidence

- Run focused checks after the refactor.
- Run the required Rust closure loop:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Record command outcome and explicit exit code for every gate.

### Phase E - Review, Verification, and Disposition

- Complete dual review artifacts with finding disposition.
- Complete dual verification artifacts, including gate legitimacy,
  numeric-equivalence, lint-ratchet, and line-count-governance checks.
- Complete disposition and worker handoff.

## Exit Criteria

- `compute_active_frost_coupling` no longer has
  `#[allow(clippy::too_many_lines)]`.
- Workspace clippy passes under `-D warnings` without adding a replacement
  suppression.
- Numeric equivalence is preserved by helper extraction; no formula, threshold,
  or float expression grouping changes are introduced.
- Focused frost characterization and required closure gates are run and
  recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched `.rs` file is at or above 2000 lines unless a WARN disposition is
  recorded with owner and follow-on intent.
- No review finding remains undispositioned.
- Gate Evidence Non-Deferral Rule is satisfied for every current-scope gate.

## Execution Summary

Static: behavior-preserving helper extraction was completed inside
`frost_entry.rs`. Public `pub(crate)` method signatures remain unchanged for
`resolve_active_frost_coupling` and `compute_active_frost_coupling`.
The former `#[allow(clippy::too_many_lines)]` suppression was removed, and no
replacement function-length suppression was added.

Ran: focused frost characterization, workspace coverage/CRAP metric collection,
focused clippy, `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, `cargo deny check`, and `git diff --check` all exited
0. See `artifacts/gate-results.md` and the CQR01 metric artifacts.

Disposition: complete. Supporting CRAP metric improved from
`compute_active_frost_coupling` CRAP `238.28646229402713` to `8.003859752282304`;
the largest remaining target-module CRAP row is
`require_frost_layer_water_state` at `16.12455583153302`.

## Subagent Requirement

Subagent authorization: this package explicitly authorizes
spawning/delegating to review and verification subagents for bounded read-only
review of this package's artifacts and source diff. Expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is limited to the package
artifact files. If subagents are unavailable, the executing agent must perform
equivalent independent local reviews and record that path.

## Instruction Precedence

Package-required gates override ambient test/validation-skip guidance,
including: `UNLESS you are explicitly requested to do so, NEVER run tests or
validate your work.`

## Security Impact Gate

- security_impact: low
- dedicated_security_review_required: no
- rationale: private Rust helper extraction only; no new parser, subprocess,
  network, serialization, external authority, or unsafe surface.
