# CQR31 CRAP After

Ran: `cargo llvm-cov clean --workspace`

Ran: `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001/artifacts/lcov_after.info`

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001/artifacts/crap_after.json`

After metrics for the target and extracted helpers:

- `build_simulation_owned_wb13_row_for_ofe`: CRAP `16.0`,
  cyclomatic `16.0`, coverage `100.0`.
- `validate_wb13_publication_context`: CRAP `12.584884659264825`,
  cyclomatic `7.0`, coverage `51.515151515151516`.
- `wb13_profile_storage_inputs`: CRAP `12.332361516034986`,
  cyclomatic `10.0`, coverage `71.42857142857143`.
- `wb13_frozen_storage_inputs`: CRAP `11.5625`, cyclomatic `10.0`,
  coverage `75.0`.
- `wb13_subsurface_flow_terms`: CRAP `9.648`, cyclomatic `9.0`,
  coverage `80.0`.
- `wb13_evaporation_terms`: CRAP `9.187054752696927`,
  cyclomatic `8.0`, coverage `73.52941176470588`.
- `wb13_calendar_publication_keys`: CRAP `8.929333333333334`,
  cyclomatic `6.0`, coverage `56.666666666666664`.
- `wb13_liquid_input_terms`: CRAP `8.421875`, cyclomatic `8.0`,
  coverage `81.25`.
- `wb13_runoff_publication_terms`: CRAP `6.702548930359582`,
  cyclomatic `6.0`, coverage `73.07692307692307`.
- `wb13_deep_percolation_mm`: CRAP `6.216378662659654`,
  cyclomatic `6.0`, coverage `81.81818181818183`.
- `wb13_precipitation_mm`: CRAP `5.2459912536443145`,
  cyclomatic `5.0`, coverage `78.57142857142857`.
- `build_wb13_row_surface`: CRAP `4.002`, cyclomatic `4.0`,
  coverage `95.0`.
- `wb13_physical_runoff_mm`: CRAP `3.182569496619083`,
  cyclomatic `3.0`, coverage `72.72727272727273`.
- `wb13_interception_mm`: CRAP `3.140625`, cyclomatic `3.0`,
  coverage `75.0`.
- `wb13_calendar_projection`: CRAP `3.072`, cyclomatic `3.0`,
  coverage `80.0`.

LCOV summary:

- `FNF: 133`
- `FNH: 90`
- `LF: 1750`
- `LH: 1224`

Line counts after:

- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`:
  `2095`
- `docs/work-packages/README.md`: `673`
- `docs/work-packages/cqr-burndown-execplan.md`: `754`

Warning: `cargo crap` reported `126` source files with no matching LCOV entry,
matching the before run and prior CQR package evidence.

Disposition: target and all newly extracted helpers are CRAP `<= 30`.

Note: `derive_profile_fc_store_from_authoritative_layers` remains CRAP
`31.780588037757312`; it was already present in the target file before CQR31
and was outside the scoped `build_simulation_owned_wb13_row_for_ofe` metric
target.
