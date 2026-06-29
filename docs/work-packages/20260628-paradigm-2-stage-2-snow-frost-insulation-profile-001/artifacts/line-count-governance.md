# Line-Count Governance

Status: `RECORDED`

This artifact records touched-file size checks and any required mitigation.

Ran:

- Command: `wc -l Cargo.toml crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md docs/work-packages/README.md docs/work-packages/20260628-paradigm-2-stage-2-snow-frost-insulation-profile-001/package.md tests/integration/paradigm2_stage2_snow_frost_insulation_profile.rs tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py tests/integration/snowdensity03_physics_bulk_offline_contract.rs`

Selected line counts:

- `00_builders_and_authority.rs`: `2846`
- `00a_snow_frost_authority_impl.rs`: `816`
- `SC-SNOWFREEZE-001.md`: `3016`
- Stage 2 package: `136`
- Stage 2 diagnostic tool: `738`
- Stage 2 integration test: `111`

Mitigation: Stage 2 edits stay in the existing direct-publication snow/frost
handoff helpers because those files already own the relevant trace and thermal
input seams. No new large production module was introduced; generated evidence
JSON is retained as package artifact evidence.
