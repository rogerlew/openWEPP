# Guard-Path Taxonomy Evidence (CLIM15)

Evidence mode: `Static + Ran`
Status: `collected`

Static:
- Shared breakpoint adaptation no longer exposes a reachable `E-010` emission path.
- Watershed contextual taxonomy no longer exposes retired `E-010` variant.
- Shared/hillslope/watershed tests exercise real guard paths for `E-006`, `E-009`, and `E-011`.

Ran:
- Executed required gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- All required commands passed.

## Guard-Path Test Evidence
1. Shared runtime adapter
- `runtime_request_rejects_negative_breakpoint_drain_with_e006`
- `runtime_request_rejects_non_monotone_breakpoint_times_with_e009`
- `runtime_request_rejects_breakpoint_cardinality_over_1500_even_with_parser_override`

2. Hillslope runtime seam
- `climate_runtime_surface_rejects_negative_breakpoint_drain`
- `climate_runtime_surface_rejects_duplicate_breakpoint_times`
- `climate_runtime_surface_rejects_breakpoint_cardinality_over_1500_even_with_parser_override`

3. Watershed runtime seam
- `climate_runtime_surface_rejects_negative_breakpoint_drain`
- `climate_runtime_surface_rejects_duplicate_breakpoint_times`
- `climate_runtime_surface_rejects_breakpoint_cardinality_over_1500_even_with_parser_override`
