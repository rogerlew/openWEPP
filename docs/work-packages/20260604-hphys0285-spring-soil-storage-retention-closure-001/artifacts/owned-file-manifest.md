# Owned-File Manifest

Status: complete
Evidence mode: Static

## Owned Write Set

Static:
- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260604-hphys0285-spring-soil-storage-retention-closure-001/**`
- `tests/integration/hphys0284_negative_melt_snowpack_state_contract.rs`
- `tests/integration/hphys0285_spring_soil_storage_retention_contract.rs`

## Excluded

Static:
- No WB17 `Ep` production edits.
- No heuristic infiltration-capacity, percolation, or storage-retention compensation.
- No broad snowpack timing/melt rewrite beyond bounded pack-exhaustion canonicalization authorized in `SC-SNOWFREEZE-001`.
