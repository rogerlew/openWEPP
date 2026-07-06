# Required Reading Map

Status: **EXECUTED-COMPLETE** (scaffold map retained as the execution intake
record).

Core required-reading byte total: **356,093 bytes** (`OK`, below the
400,000-byte WARN threshold in
`docs/standards/kernel-work-package-preparation.md`).

## Core

Read before edits:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/package.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/final-disposition.md`
- `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/artifacts/disposition.md`
- `docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/seam.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `tests/integration/laned_shadow_h2637.rs`

## Conditional

Read when touched:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- Runner publication, manifest, or audit-counter modules if timing evidence
  changes emitted diagnostics.
- `docs/standards/local-ci-gate-selection.md`
- `docs/specifications/unit-governance.md` when adding timing units or
  elapsed-time metadata.
- Any D14-created profiling helper under `tools/`.

## On Demand

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- D10/D11/D12/D13 artifacts beyond the final disposition/handoff only when
  a profiling result intersects their authority surface.

## Notes

D14 must not optimize by changing solver method, closure tolerance, source
authority, activation semantics, or sediment/water publication. Treat timing
diagnostics as evidence infrastructure; real acceptance still requires
endpoint timing and output/closure parity.
