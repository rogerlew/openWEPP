# Characterization

Status: `COMPLETE`

Ran: added explicit symbol-string characterization in
`tests/integration/arch22_typed_state_surface_contract.rs`.

Added tests:

- `arch22_wb11_wb12_state_symbols_match_authority_snapshot`
- `arch22_irrigation_scalar_state_symbols_match_authority_snapshot`
- `arch22_plant_hyetograph_soil_state_symbols_match_authority_snapshot`
- `arch22_snow_frost_state_symbols_match_authority_snapshot`
- `arch22_peak_method_state_symbols_match_authority_snapshot`
- `arch22_all_hillslope_flux_symbol_projections_match_authority_snapshot`
- `arch22_all_dynamic_irrigation_field_suffixes_match_authority_snapshot`
- `arch22_climate_forcing_surface_error_display_matches_authority_snapshot`
- `arch22_climate_forcing_symbol_surface_accessors_match_authority_snapshot`
- `arch22_all_watershed_channel_symbols_match_authority_snapshot`
- `arch22_all_watershed_impoundment_symbols_match_authority_snapshot`

Behavior oracle:

- Every static `HillslopeProductionStateSymbol` variant maps to its existing
  authority string.
- Every `HillslopeProductionFluxSymbol` variant maps to its existing authority
  string.
- Every dynamic irrigation depletion and fixed-date field suffix maps through
  `BoundarySymbol::from` with the existing zero-padded period/event formatting.
- The climate forcing point-count guard preserves its typed error and display
  string, and hillslope/watershed climate forcing accessors preserve their
  zero-padded `timem_*` and `intsty_*` symbol projections.
- All watershed channel and impoundment state and flux field suffixes preserve
  their typed symbol strings.
- Watershed hillslope contributor symbol projection covers peak, duration,
  totals, particle class count, sediment concentration, particle diameter, and
  particle flow fraction formats.

Focused command evidence:

- `cargo nextest run --test arch22_typed_state_surface_contract` exited `0` with
  `17 tests run: 17 passed, 0 skipped`.
- `cargo check -p openwepp-kernel-contract` exited `0`.
- `cargo fmt --check` initially reported rustfmt-only diffs in the new test;
  `cargo fmt` was run and the repeated `cargo fmt --check` exited `0`.
- `cargo clippy --workspace --all-targets -- -D warnings` initially failed on a
  test-only `too_many_lines` warning in the exhaustive state-symbol test. The
  oracle was split into category-scoped tests without adding a lint allow, and
  clippy then exited `0`.
- Review B identified uncovered target rows for
  `ClimateForcingSymbolSurfaceError::fmt`,
  `WatershedImpoundmentStateField::as_str`, and
  `WatershedImpoundmentFluxField::as_str`; the two additional characterization
  tests above were added before final coverage refresh.
- A later unique-region audit identified remaining source spans in climate
  forcing accessors, watershed channel fields, `Nchnum`, and particle diameter
  projection; the final two characterization tests and the particle-diameter
  assertion above were added before the refreshed final closure run.
