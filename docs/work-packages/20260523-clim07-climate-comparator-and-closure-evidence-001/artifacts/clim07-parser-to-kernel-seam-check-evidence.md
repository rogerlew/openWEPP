# CLIM07 Parser-to-Kernel Seam-Check Evidence

Status: `completed`
Evidence mode: `Ran`

## Scope
Verify climate parser outputs project to kernel runtime seams without silent
field omission, rewrite, or fallback for accepted continuous-daily and
breakpoint branches.

## CLIM07 Seam Checks
- New CLIM07 contract test vectors:
  - continuous-daily hillslope + watershed `hs{id}_*` projection parity.
  - breakpoint hillslope + watershed `hs{id}_*` projection parity.
  - duplicate breakpoint-time typed hard-fail at both seams.

## ARCH17 Non-Regression Checks
- `cargo test --test parser_runtime_seam_integration`
  - result: pass (`45 passed`).
  - includes climate seam closure tests:
    - `climate_parser_to_hillslope_runtime_surface_closure`
    - `climate_parser_to_watershed_runtime_surface_closure`
    - `climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path`
    - `climate_wc1_fixture_applies_timep_floor_and_ip_policy_scaling`
    - `climate_wc1_fixture_caps_storm_duration_before_runtime_projection`.

## CLIM07 Targeted Run Evidence
- `cargo test --test clim07_climate_comparator_and_closure_contract`
  - result: pass (`4 passed`).
