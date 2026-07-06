# Required Reading Map

Status: **EXECUTED-HOLD-SOURCE-AUTHORITY**.

Core required-reading byte total: **279,270 bytes** (`OK`, below the
400,000-byte WARN threshold in
`docs/standards/kernel-work-package-preparation.md`).

## Core

Read before activation work:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260705-mofefid-d15-opt-in-production-activation-001/package.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1
- `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/artifacts/disposition.md`
- `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d12-melt-limb-hourly-shape-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d13-routed-hydrograph-erosion-shape-001/artifacts/final-disposition.md`
- `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/artifacts/worker-handoff.md`

## Conditional

Read when the hold is lifted and runtime activation work resumes:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `tests/integration/laned_shadow_h2637.rs`

## Notes

The D15 execution did not reach runtime edits because `SC-OFEROUTE-001`
preflight blocks production activation. The conditional runtime set is retained
as the hold-lift map for the eventual activation rerun.
