# Owned File Manifest

Status: complete / reconciled

Evidence mode: Static

The terminal base-to-worktree scope is fully owned by 21K.

## Production and authority

- `Cargo.toml`
- `assurance/v2/identity.lock.json`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json`
- `assurance/v2/transactions/1005fe71e46692c63a9b8bd4afc849621cd15415df659542e892a76f715e5c35.json`
- `assurance/v2/transactions/3a88e5036e93d51239eb69e7abbecf90dfb4da49242293d7be00958087238bd3.json`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`

## Tests and fixture custody

- `tests/integration/snow_wet_compaction_operand_authority.rs`
- `tests/integration/paradigm2_{multilayer_promotion,stage1_layered_snow_density,stage2_snow_frost_insulation_profile,stage3_decouple_water_temperature,stage3_liquid_routing_meltwater_temperature}.rs`
- `tests/integration/snow_{mass_transition_ledger_persistence_contract,surface_eb03_contract,surface_eb04w_accumulation_melt_diagnostics_contract}.rs`
- All touched `tests/integration/snowdensity*.rs` files listed by
  `git diff --name-only d41a67c7`; these are exact v124-to-v125 contract-pin
  adoption, except the owning 06B and 10.3.1A behavioral targets.
- `tests/fixtures/snotel_observed/README.md`
- `tests/fixtures/snotel_observed/snotel_snowbird_ut/manifest.md`
- `tests/fixtures/snotel_observed/snotel_snowbird_ut/development/precip_x1p2155576/{README.md,manifest.json,p8.cli}`

Canonical `tests/fixtures/snotel_observed/snotel_snowbird_ut/p8.cli` is
intentionally untouched at SHA-256
`10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7`.
The derived CLI is
`c673145ee7fd41e71e3f2e21c529fba2d12691abd5f0f055444e621fb0b80afb`.

## Package and roadmaps

- The complete
  `docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/`
  tree, including both tools and the archived execution prompt.
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`

Protected and intentionally untouched surfaces include
`09_snow_density.rs`, `Cargo.lock`, observations, canonical Snowbird climate,
PRCPSA, public v1 CoE/report/replay schemas, runtime JSONL schema, WAT/HBP/PASS,
and every phase/energy/radiation/canopy/frost/default/calibration surface.
