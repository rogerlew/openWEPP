# REFACTOR023 - Hillslope Coupling Line-Count Split

Status: complete (executed 2026-06-14)

Package type: mechanical refactor (behavior-preserving)

## Objective

Mechanically split
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
so the 3000+ line file no longer violates Rust line-count governance, while
preserving public API shape, runtime behavior, formulas, constants, guards,
and snow/frost coupling semantics.

## Rationale

`coupling.rs` is the only `.rs` file in `openwepp-hillslope-orchestrator` above
the 3000-line refactor-required threshold. The file mixes snow activation,
runtime snow domain validation, frost activation, and the frost fine-layer
heat-flow state machine. This package extracts the frost internals into child
modules under the existing `coupling` module, leaving existing inherent
`Wb11HydrologyKernel` methods callable as before.

## Refactor Seam Declaration

Source file:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`

Target layout:

- `coupling.rs`: module wiring, frost state structs shared by child modules,
  interval helpers, active snow coupling, and runtime snow validation.
- `coupling/frost.rs`: frost fine-layer helpers, heat-flow helpers, seasonal
  temperature helpers, freeze/thaw helper steps, and `tmpadj` surface
  temperature helper.
- `coupling/frost_entry.rs`: active frost coupling gate and
  `compute_active_frost_coupling` orchestration.

Public surface expected to remain stable:

- `Wb11HydrologyKernel::interval_overlap_duration`
- `Wb11HydrologyKernel::bounded_interval_overlap_duration`
- `Wb11HydrologyKernel::resolve_active_snow_coupling`
- `Wb11HydrologyKernel::validate_runtime_snow_state_domains`
- `Wb11HydrologyKernel::resolve_active_frost_coupling`
- `Wb11HydrologyKernel::compute_active_frost_coupling`

## Included Scope

- Mechanical movement of frost helper code into child module files.
- Minimal module wiring and visibility adjustment needed for sibling helper
  modules to call moved helpers.
- Mechanical formatting only.
- Package artifacts, reviews, verification, gate evidence, and disposition.

## Excluded Scope / Protected Boundaries

- No process-physics formula, constant, threshold, unit, or guard changes.
- No contract amendment; this package is mechanical-only.
- No comparator-match tuning or new validation cohort.
- No changes outside the declared write set unless required for build
  correctness and recorded before editing.

## Deliverables

1. Source refactor:
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`
   - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
2. Package catalog update:
   - `docs/work-packages/README.md`
3. Package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/refactor023-symbol-inventory.md`
   - `artifacts/refactor023-modularization-plan-report.md`
   - `artifacts/refactor023-public-api-surface-parity-report.md`
   - `artifacts/refactor023-contract-implementation-evidence.md`
   - `artifacts/refactor023-contract-test-implementation-evidence.md`
   - `artifacts/refactor023-preimplementation-contract-gate.md`
   - `artifacts/refactor023-implementation-and-test-evidence.md`
   - `artifacts/refactor023-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor023-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
   - `artifacts/refactor023_disposition.md`
   - `artifacts/worker-handoff.md`

## Intended Write Set

- `docs/work-packages/20260614-refactor023-hillslope-coupling-line-count-split-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`

## Dependencies

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`

## Phase Plan

### Phase A - Intake, Sizing, and Surface Freeze

- Capture pre-refactor line count and symbol inventory for `coupling.rs`.
- Capture pre-refactor public surface snapshot for the exported methods listed
  above.
- Record that contract-first sequencing does not require amendments because
  the work is mechanical movement only.

### Phase B - Mechanical Extraction

- Create `coupling/frost.rs` and `coupling/frost_entry.rs`.
- Move frost helper code in coherent blocks without editing formulas or guard
  logic.
- Keep `coupling.rs` as the public wiring/snow surface for this seam.

### Phase C - Validation and Evidence

- Run focused checks after the move.
- Run the required Rust closure loop:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Record command outcome and explicit exit code for every gate.

### Phase D - Review, Verification, and Disposition

- Complete dual review artifacts with finding disposition.
- Complete dual verification artifacts, including gate legitimacy and
  line-count governance checks.
- Complete disposition and worker handoff.

## Exit Criteria

- `coupling.rs` is below 2000 lines.
- No touched `.rs` file is at or above 2000 lines unless a WARN disposition is
  recorded with owner and follow-on intent.
- Required cargo closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No review finding remains undispositioned.
- Gate Evidence Non-Deferral Rule is satisfied for every current-scope gate.

## Subagent Requirement

Subagent authorization: this package explicitly authorizes
spawning/delegating to review and verification subagents for bounded
read-only review of this package's artifacts and source diff. Expected outputs
are `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
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
- rationale: internal Rust module organization only; no new parser,
  subprocess, network, serialization, or unsafe surface.
