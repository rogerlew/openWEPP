# Owned File Manifest

Status: complete
Evidence mode: Static

## Production / Test Files

Static:

- `Cargo.toml` — registers `hphys0290_post_winter_rain_publication_contract`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` — publishes and validates `snow.post_winter_rain_m`.
- `crates/openwepp-runner/src/hillslope/mod.rs` — consumes flux-required `snow.post_winter_rain_m` in WB13 `RM`, extends trace diagnostics, and adds unit regressions.
- `crates/openwepp-sim-contract/src/units.rs` — registers `snow.post_winter_rain_m` unit/domain/typed-boundary metadata.
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs` — source-level contract gate for WB13/kernel/unit registry.
- `tests/integration/sim_contract_boundary_unit_registry.rs` — runtime alias and metadata coverage for the new symbol.

## Contract Files

Static:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

## Work-Package Files

Static:

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/package.md`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/prompts/README.md`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/prompts/active/README.md`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/prompts/active/hphys0290_kickoff_agent_prompt.md`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/prompts/archived/README.md`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/**`

Disposition: all listed files are within the intended package write set.
