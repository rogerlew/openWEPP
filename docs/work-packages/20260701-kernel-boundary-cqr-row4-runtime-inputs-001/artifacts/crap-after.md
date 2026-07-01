# CRAP After

Ran:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info --min 0 --format json --output /tmp/openwepp-crap-row4-after-final.json
```

Result:

- Coverage/test phase passed.
- LCOV written to `lcov.info`.
- CRAP JSON written to `/tmp/openwepp-crap-row4-after-final.json`.
- `cargo crap` warned about 132 test/artifact files with no LCOV entry; row #4
  extraction uses production runtime-input source files only.

Row #4 extraction above CRAP 30:

```text
0
```

Delta:

- Before: 24 unique row #4 production offender entries above CRAP 30, duplicated
  to 48 rows by current `cargo crap` report shape.
- After: 0 row #4 entries above CRAP 30.
- ADR-0021 disposition: PASS, no complete-with-warnings exception needed.

Top remaining row #4 scores:

```text
file	function	line	cc	coverage	crap
05_projection_helpers.rs	growth_equation_parameter_values	211	22.0	78.57142857142857	26.762390670553938
05_projection_helpers.rs	project_annual_extension_controls	521	9.0	40.0	26.496
06_simimpl28_hourly_forcing.rs	simimpl28_sunmap	451	21.0	78.86178861788618	25.16527134932266
06_simimpl28_hourly_forcing.rs	build_simimpl28_hourly_winter_forcing_typed	170	24.0	93.63636363636364	24.148435762584523
04_snow_frost_irrigation.rs	project_typed_frost_runtime	102	16.0	72.22222222222221	21.48696844993142
05_projection_helpers.rs	project_burn_annual_extension_controls	569	4.0	0.0	20.0
05_projection_helpers.rs	project_perennial_cutday_symbols	772	4.0	0.0	20.0
06_simimpl28_hourly_forcing.rs	simimpl28_stmtim_hourly_partition_with_model	731	18.0	83.87096774193549	19.35947098116881
06_simimpl28_hourly_forcing.rs	simimpl28_relative_humidity_from_dewpoint_saturated	930	6.0	31.25	17.6982421875
00_core_types.rs	HillslopeRuntimeInputError::soil_core_code	385	14.0	93.75	14.0478515625
```
