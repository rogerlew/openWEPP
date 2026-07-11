# Pre-Integration CQR Campaign Baseline

Evidence class: **Ran**

Campaign: `CQR-PREINT-20260711`
Source commit: `14dcb022a86aa2e8921ab1154a6b8335e9ef0c26`
Planning-only comparison commit: `e320ab69044f45c2f8e8898519ae532da268f58e`

## Reproducibility

The source scan ran from `/home/workdir/openWEPP`. Production Rust under
`crates/` is byte-identical between the source and planning commits. The
commands were:

    cargo llvm-cov clean --workspace
    cargo llvm-cov --workspace --ignore-run-fail --lcov \
      --output-path /tmp/openwepp-cqr-followup-final.lcov
    cargo llvm-cov --workspace --ignore-run-fail --json \
      --output-path /tmp/openwepp-cqr-followup-final.json
    cargo crap --workspace \
      --lcov /tmp/openwepp-cqr-followup-final.lcov \
      --min 0 --format json \
      --output /tmp/openwepp-cqr-followup-final-crap.json

| Step | Exit | Elapsed |
| --- | ---: | ---: |
| LCOV | 0 | 2118.66 s |
| JSON | 0 | 2079.91 s |
| CRAP | 0 | 2.08 s |

Both coverage commands used `--ignore-run-fail`. The underlying workspace run
returned 101 only for the known parallel environment interference in
`-p openwepp --test laned_shadow_h2637`; the final follow-up summary recorded
the varying fail-closed/environment-mutual-exclusion cases. No target module
test failed. A campaign executor must reattribute every ignored failure on each
fresh run; this baseline attribution is not reusable evidence.

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| LCOV | 4,376,909 | `a8ef30b6c6b748cdee3e5239bf74cabcab281fa9fa166e51dbc96bec979943b1` |
| JSON | 19,110,180 | `53e7740029043b290f4e3d26bbf60e204d3df8ccd2cca78fc2b9ea2a4aa759e0` |
| CRAP JSON | 2,869,984 | `bb67da1bf31bdfabcbba156c0f176a8365a2c3be4ec2f1a801644d71a6862c56` |

## Production Filter And Deduplication

The raw `cargo-crap` JSON includes tests, examples, documentation prototypes,
artifacts, and duplicate compilation rows. The fixed campaign census applies
all of these rules:

1. retain repository-relative files beginning with `crates/`;
2. require a `/src/` component;
3. exclude paths containing `/src/tests/`;
4. retain rows with CRAP strictly greater than 30;
5. deduplicate by the exact tuple `(file, function, line, cyclomatic, coverage,
   crap)`.

This produces 67 rows across 45 module paths. The campaign must preserve this
original ledger separately from each fresh raw/actionable rerank. Eligibility is
not encoded here; each current-source row still requires ADR-0021 classification
and two-review disposition where applicable.

## Fixed 67-Row Ledger

| Module | Function | Line | CC | Coverage % | CRAP |
| --- | --- | ---: | ---: | ---: | ---: |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs` | `DirectPublicationDayRow::from_day_frame` | 307 | 30.0 | 84.81012658227847 | 33.15431436458374 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | `DirectFrameExecutor::run_laned_active_publication_stream` | 427 | 43.0 | 79.83539094650206 | 58.16024886076688 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs` | `validate_erod13_inputs` | 1020 | 31.0 | 77.96610169491525 | 41.28010166570097 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs` | `wave1_xcrit` | 523 | 36.0 | 92.5925925925926 | 36.52674897119341 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs` | `wave1_erod` | 1030 | 53.0 | 76.26459143968872 | 90.56140892450266 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs` | `wave1_route` | 1412 | 31.0 | 76.75675675675676 | 43.06739612658677 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs` | `validate_wave1_inputs` | 1665 | 35.0 | 82.6086956521739 | 41.443659077833495 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs` | `wave1_apply_inter_ofe_continuity` | 2201 | 30.0 | 91.25 | 30.6029296875 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_operands.rs` | `erosion_particle_composition` | 207 | 46.0 | 78.88198757763976 | 65.92850804918237 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs` | `assemble_wave1_continuity_inputs_quantum` | 309 | 34.0 | 91.12426035502959 | 34.80829798734526 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | `laned_active_route_lane` | 810 | 30.0 | 83.84279475982532 | 33.796131381251165 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs` | `DirectDayFrame::validate_r4pqz_hydrology_projection_domain` | 214 | 20.0 | 61.33333333333333 | 43.12438518518519 |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` | `interpolate_unit_discharge` | 90 | 7.0 | 0.0 | 56.0 |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/iwagaki_oracle.rs` | `run_oracle` | 177 | 30.0 | 95.3125 | 30.092697143554688 |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | `CellParameters::alpha_q_celerity` | 405 | 33.0 | 86.04651162790698 | 35.958531953161355 |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | `KinematicWaveSolver::step` | 1091 | 53.0 | 91.03139013452915 | 55.026409146542875 |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | `KinematicWaveSolver::run_with_options_and_step_trace` | 1512 | 37.0 | 92.7536231884058 | 37.52091419108761 |
| `crates/openwepp-input-contract/src/parsers/climate.rs` | `parse_climate_from_str` | 354 | 41.0 | 74.07407407407408 | 70.29345120154446 |
| `crates/openwepp-input-contract/src/parsers/climate.rs` | `parse_no_breakpoint_day` | 574 | 21.0 | 69.23076923076923 | 33.84660901228949 |
| `crates/openwepp-input-contract/src/parsers/climate.rs` | `parse_breakpoint_day` | 644 | 31.0 | 81.37254901960785 | 37.21131672584451 |
| `crates/openwepp-input-contract/src/parsers/frost.rs` | `FrostParseError::fmt` | 160 | 7.0 | 0.0 | 56.0 |
| `crates/openwepp-input-contract/src/parsers/gwcoeff.rs` | `GwcoeffParseError::fmt` | 191 | 9.0 | 0.0 | 90.0 |
| `crates/openwepp-input-contract/src/parsers/gwcoeff.rs` | `enforce_result_invariants` | 432 | 16.0 | 53.333333333333336 | 42.017185185185184 |
| `crates/openwepp-input-contract/src/parsers/hbp/error.rs` | `HbpFormatErrorCode::as_str` | 25 | 15.0 | 35.294117647058826 | 75.95562792591082 |
| `crates/openwepp-input-contract/src/parsers/phosphorus.rs` | `PhosphorusParseError::fmt` | 145 | 9.0 | 0.0 | 90.0 |
| `crates/openwepp-input-contract/src/parsers/pmetpara.rs` | `PmetparaParseError::fmt` | 231 | 12.0 | 0.0 | 156.0 |
| `crates/openwepp-input-contract/src/parsers/snow.rs` | `SnowParseError::fmt` | 156 | 10.0 | 0.0 | 110.0 |
| `crates/openwepp-input-contract/src/parsers/snow.rs` | `enforce_invariants` | 445 | 10.0 | 39.39393939393939 | 32.26117928597267 |
| `crates/openwepp-input-contract/src/parsers/tcr.rs` | `TcrParseError::fmt` | 180 | 10.0 | 0.0 | 110.0 |
| `crates/openwepp-input-contract/src/parsers/watershed_structure.rs` | `parse_watershed_structure_from_str` | 348 | 43.0 | 94.05940594059405 | 43.387638175639935 |
| `crates/openwepp-input-contract/src/parsers/wepp_ui.rs` | `WeppUiParseError::fmt` | 135 | 6.0 | 0.0 | 42.0 |
| `crates/openwepp-landuse-migrate/src/cli.rs` | `run_cli_args` | 15 | 54.0 | 78.1456953642384 | 84.43676543755633 |
| `crates/openwepp-landuse-migrate/src/convert.rs` | `yearly_extension_to_yaml` | 500 | 6.0 | 0.0 | 42.0 |
| `crates/openwepp-landuse-migrate/src/lib.rs` | `LanduseMigrationError::fmt` | 520 | 17.0 | 44.776119402985074 | 65.6719343802263 |
| `crates/openwepp-landuse-migrate/src/lib.rs` | `authority_from_files` | 703 | 7.0 | 0.0 | 56.0 |
| `crates/openwepp-legacy-bridge/src/hbp.rs` | `HbpAdapterError::fmt` | 116 | 7.0 | 0.0 | 56.0 |
| `crates/openwepp-legacy-bridge/src/sidecar.rs` | `SidecarAdapterError::fmt` | 204 | 9.0 | 25.0 | 43.171875 |
| `crates/openwepp-management-schema/src/lib.rs` | `ManagementYamlError::fmt` | 415 | 9.0 | 23.076923076923077 | 45.86845698680018 |
| `crates/openwepp-management-schema/src/lib.rs` | `validate_management_yaml_document` | 498 | 24.0 | 73.33333333333333 | 34.92266666666667 |
| `crates/openwepp-management-schema/src/lib.rs` | `validate_schedule` | 788 | 20.0 | 70.0 | 30.800000000000004 |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7.0 | 0.0 | 56.0 |
| `crates/openwepp-runner/src/bin/open_wepp_runner.rs` | `run_hillslope_command` | 41 | 11.0 | 0.0 | 132.0 |
| `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` | `run` | 16 | 19.0 | 60.416666666666664 | 41.38949471932871 |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | `run` | 75 | 58.0 | 77.44565217391305 | 96.5964008531016 |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | `hillslope_area_m2_from_source_runfile` | 762 | 11.0 | 45.614035087719294 | 30.464617993120694 |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | `parse_watershed_runfile` | 992 | 66.0 | 69.36936936936937 | 191.18597779664248 |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | `validate_manifest_publication_metadata` | 1658 | 18.0 | 53.84615384615385 | 49.85434683659537 |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | `validate_manifest_per_ofe_wb13_publication_policies` | 1817 | 11.0 | 45.714285714285715 | 30.35717784256561 |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | `validate_manifest_mofe_hourly_carry_metadata` | 2023 | 17.0 | 51.724137931034484 | 49.51531428102833 |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | `execute_direct_publication_stream` | 118 | 17.0 | 53.535353535353536 | 45.99117076282671 |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | `write_laned_active_trace_output` | 989 | 8.0 | 5.343511450381679 | 62.27891308670334 |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | `direct_production_typed_growth_crop_authority` | 1875 | 27.0 | 80.0 | 32.831999999999994 |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` | `build_laned_shadow_lane_day_operands` | 1054 | 16.0 | 56.32183908045977 | 37.33206986148886 |
| `crates/openwepp-runner/src/hillslope/intake_lane_setup/runfile_helpers.rs` | `parse_runfile_execution_config` | 11 | 25.0 | 78.33333333333333 | 31.357060185185187 |
| `crates/openwepp-runner/src/hillslope/snowbench.rs` | `SnowbenchError::fmt` | 138 | 8.0 | 27.27272727272727 | 32.6190833959429 |
| `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs` | `read_canopy_series` | 315 | 13.0 | 45.0 | 41.11737500000001 |
| `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs` | `parse_forcing_line` | 400 | 21.0 | 59.34065934065934 | 50.64282463099032 |
| `crates/openwepp-runner/src/release.rs` | `validate_release_sidecar_unlocked` | 343 | 19.0 | 67.44186046511628 | 31.459079074798446 |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9.0 | 0.0 | 90.0 |
| `crates/openwepp-sim-contract/src/units_mod/registries.rs` | `validate_entry` | 783 | 19.0 | 50.617283950617285 | 62.4742520806637 |
| `crates/openwepp-summary-accumulator/src/lib.rs` | `Wb13DailyWaterBalanceRow::from_surface` | 228 | 33.0 | 77.77777777777779 | 44.95061728395059 |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs` | `Ws10ChannelImpoundmentKernel::ws11_muskingum_geometry_from_depth` | 29 | 15.0 | 52.307692307692314 | 39.40773782430587 |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs` | `Ws10ChannelImpoundmentKernel::compute_variable_muskingum_cunge_state` | 431 | 32.0 | 70.45454545454545 | 58.410217881292255 |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs` | `Ws10ChannelImpoundmentKernel::ws11_route_baseline_wave_series` | 362 | 44.0 | 85.1063829787234 | 50.39596235901486 |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | `WatershedNetworkFrameError::fmt` | 57 | 11.0 | 13.157894736842104 | 90.24582665111532 |
| `crates/openwepp-watershed-output/src/writers.rs` | `write_output_record_parquet_outputs` | 725 | 29.0 | 84.48275862068965 | 32.14224137931035 |
| `crates/openwepp-watershed-output/src/writers.rs` | `float64_value` | 2206 | 69.0 | 100.0 | 69.0 |
