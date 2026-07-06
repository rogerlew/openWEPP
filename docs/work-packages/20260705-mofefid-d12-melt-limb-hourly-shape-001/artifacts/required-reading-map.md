# Required Reading Map

Status: **SCAFFOLDED**.

Core required-reading byte total: **323,774 bytes** (`OK`, below the
400,000-byte WARN threshold in
`docs/standards/kernel-work-package-preparation.md`).

## Core

Read before edits:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/package.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1
- `docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md`
- `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/artifacts/worker-handoff.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`

## Conditional

Read when touched:

- Snow/liquid state and direct publication:
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
- Companion contracts when source ownership or unit governance changes:
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- H2637 fixture/test surfaces when reproducing `days_uniform_shape`:
  - `tests/fixtures/laned_shadow_h2637/`
  - `tests/integration/laned_shadow_h2637.rs`

## On Demand

- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/wepp-forest_260430_baseline` snow/runoff source files if a
  baseline snowmelt timing question cannot be resolved from current contracts.

## Notes

D12 must not treat uniform fallback as authority. Uniform shape is diagnostic
plumbing only unless contract text explicitly ratifies a residual class.
