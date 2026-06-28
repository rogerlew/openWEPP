# Kickoff: SNOWDENSITY-10.3.16 Open-Surface Ablation Stage A

Execution mode: package-end-to-end

Autonomy: Execute all package phases through disposition without additional
user intervention unless a hard blocker is reached. If any current-scope gate
fails or cannot be run, close `HOLD` or non-promotion with the blocker named;
do not activate the candidate.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/work-packages/20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001/package.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.2 item 7 and §10.3
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

Conditional:

- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `tests/AGENTS.md`

On-demand:

- `references/copyrighted/marks1999.pdf`
- `references/copyrighted/source_pdfs/marks1998.pdf`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `tools/snowfreeze_observed/*`

Implement contract-first, selector-gated Stage A only:

- Add opt-in `coe_open_sublimation_stage_a_v1`.
- Keep default activated bundle and explicit rollback intact.
- Track sublimated mass as vapor loss, not routed melt.
- Run real coupled WAT/trace diagnostics and close truthfully.
