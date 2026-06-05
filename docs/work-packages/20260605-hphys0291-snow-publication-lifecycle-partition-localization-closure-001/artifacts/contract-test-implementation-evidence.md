# Contract Test Implementation Evidence

Status: complete
Evidence mode: static

Static: Contract-derived tests were authored after canonical `SC-*` amendments
and before production code changes.

## Added Test Target

- `Cargo.toml` registers `hphys0291_snow_publication_lifecycle_contract`.
- `tests/integration/hphys0291_snow_publication_lifecycle_contract.rs`
  validates:
  - `INV-SNOWFREEZE-024`, `INV-RUNOFFPART-021`, and `INV-WATBAL-066` exist in
    canonical contracts.
  - runoff reconciliation uses a named
    `publish_same_day_snow_publication_fluxes` helper.
  - direct runoff reconciliation execution publishes
    `snow.post_winter_rain_m` and `snow.routed_melt_m` for dry/no-snow and
    active-snow vectors.
  - WB13 requires `snow.post_winter_rain_m` from the flux surface.
  - WB13 has a regression test rejecting state-only routed melt.
  - trace schema exposes flux-only `snow_routed_melt_m` and
    `snow_post_winter_rain_m`.

## Adjacent Authority Alignment

- `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`
  now requires flux-only `snow.routed_melt_m` publication rather than
  flux-preferred state fallback.
