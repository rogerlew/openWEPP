# Required Reading Map

Status: **SCAFFOLDED**.

Core required-reading byte total: **485,790 bytes** (`WARN`, above the
400,000-byte WARN threshold and below the 800,000-byte
REQUIRES-JUSTIFICATION threshold in
`docs/standards/kernel-work-package-preparation.md`). The heavy core set is
kept because D13 crosses `SC-OFEROUTE-001`, `SC-SED-001`, ADR-0036, and the
real erosion/routing consumer path.

## Core

Read before edits:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/package.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/worker-handoff.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`

## Conditional

Read when touched:

- Erosion runtime internals:
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs`
  - `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- HBP/interchange contracts when publication/schema claims change:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- H2637 fixture/test surfaces when proving active-candidate behavior:
  - `tests/fixtures/laned_shadow_h2637/`
  - `tests/integration/laned_shadow_h2637.rs`

## On Demand

- `docs/standards/local-ci-gate-selection.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- D10/D14 package artifacts only if D13 finds a numerical-method or
  performance boundary.

## Notes

D13 must not treat DC01 source-shape weights as the active-routed-water erosion
hydrograph. DC01 may remain the default/off and pre-activation path.
