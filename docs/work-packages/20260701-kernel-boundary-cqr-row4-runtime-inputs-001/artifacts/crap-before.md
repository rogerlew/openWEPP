# CRAP Before

Ran: `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info --min 0 --format json --output /tmp/openwepp-crap-row4-before.json`

Row #4 extraction:

```text
file	function	line	cc	coverage	crap
00_core_types.rs	HillslopeRuntimeInputError::fmt_soil_core	499	14.0	0.0	210.0
00_core_types.rs	HillslopeRuntimeInputError::soil_core_code	385	14.0	0.0	210.0
00_core_types.rs	HillslopeRuntimeInputError::pl_projection_code	467	12.0	0.0	156.0
05_projection_helpers.rs	project_perennial_grazing_cycle_symbols	789	11.0	0.0	132.0
00_core_types.rs	HillslopeRuntimeInputError::fmt_slope_shape	720	10.0	0.0	110.0
00_core_types.rs	HillslopeRuntimeInputError::fmt_snow_frost_irrigation	1102	10.0	0.0	110.0
00_core_types.rs	HillslopeRuntimeInputError::fmt_soil_layer	567	10.0	0.0	110.0
00_core_types.rs	HillslopeRuntimeInputError::slope_shape_code	428	10.0	0.0	110.0
00_core_types.rs	HillslopeRuntimeInputError::snow_frost_irrigation_code	483	10.0	0.0	110.0
00_core_types.rs	HillslopeRuntimeInputError::soil_layer_code	403	10.0	0.0	110.0
00_core_types.rs	HillslopeRuntimeInputError::code	316	9.0	0.0	90.0
00_core_types.rs	HillslopeRuntimeInputError::fmt	1180	9.0	0.0	90.0
00_core_types.rs	HillslopeRuntimeInputError::fmt_slope_numeric	788	9.0	0.0	90.0
00_core_types.rs	HillslopeRuntimeInputError::slope_numeric_code	442	9.0	0.0	90.0
00_core_types.rs	HillslopeRuntimeInputError::fmt_management	862	8.0	0.0	72.0
00_core_types.rs	HillslopeRuntimeInputError::fmt_pl_projection_shape	956	8.0	0.0	72.0
00_core_types.rs	HillslopeRuntimeInputError::management_code	455	8.0	0.0	72.0
00_core_types.rs	HillslopeRuntimeInputError::fmt_soil_corrected	666	7.0	0.0	56.0
00_core_types.rs	HillslopeRuntimeInputError::soil_corrected_code	417	7.0	0.0	56.0
05_projection_helpers.rs	annual_extension_variant_name	494	7.0	0.0	56.0
06_simimpl28_hourly_forcing.rs	build_simimpl28_hourly_winter_forcing_typed	170	24.0	62.727272727272734	53.82606761833207
00_core_types.rs	HillslopeRuntimeInputError::fmt_pl_projection_payload	1034	6.0	0.0	42.0
06_simimpl28_hourly_forcing.rs	simimpl28_sunmap	390	23.0	67.91044776119402	40.48020991611335
06_simimpl28_hourly_forcing.rs	simimpl28_stmtim_hourly_partition_with_model	686	23.0	74.01574803149606	32.28081955376509
```

Static: Row #4 owned files `04_snow_frost_irrigation.rs` and
`07_series_helpers.rs` have no production function above CRAP 30 in this
baseline.
