# Implementation and Test Evidence

Evidence mode: Static + Ran.

Static: code changes are diagnostic-only. They touch the snowbench exporter,
the PySnobal harness, focused tests, and package/docs surfaces. They do not
change production snow, frost, hydrology, erosion, runtime activation, or
science-contract thresholds.

Static: `openwepp-snowbench export-pysnobal` now writes current openWEPP snow
comparison rows by executing the generated diagnostic run through the existing
compatibility WAT publication path, extracting WAT `Snow-Water` and
`Snow-Depth`, and mapping `sim_day_index` to the external climate dates used by
the PySnobal forcing.

Static: `tools/snowfreeze_observed/pysnobal_compare.py` now supports:

- `--site`;
- `--lane`;
- `--start-date` and `--end-date`;
- `--reuse-existing`;
- `--route-policy all-lanes`;
- `--route-policy site-sane`.

Ran:

```text
cargo build -p openwepp-runner --bin openwepp-snowbench
Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.81s
```

Ran:

```text
cargo test -p openwepp-runner snowbench::tests
running 4 tests
date_continuity_accepts_leap_day_sequence ... ok
date_continuity_rejects_invalid_calendar_day ... ok
date_continuity_rejects_non_uniform_daily_step ... ok
openwepp_snow_projection_uses_climate_date_for_sim_day_index ... ok
```

Ran:

```text
cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract
running 2 tests
g1_openwepp_snow_projection_extracts_wat_swe_and_physical_depth ... ok
g0_exporter_emits_pysnobal_schema_and_required_anti_alias_lineage ... ok
```

Ran:

```text
.venv/bin/python -m py_compile tools/snowfreeze_observed/pysnobal_compare.py
```

Ran: generated fresh G1 exports for all five frost sites under
`target/snowfrost_fidelity_g1/site{1..5}`. Export sizes:

```text
site1: 333120 hourly rows across 3 lanes
site2: 394488 hourly rows across 3 lanes
site3: 298056 hourly rows across 3 lanes
site4: 394488 hourly rows across 3 lanes
site5: 394488 hourly rows across 3 lanes
```

Ran: every G1 `openwepp_snow.csv` row date parses after the `sim_day_index` to
climate-date correction:

```text
site1 bad= []
site2 bad= []
site3 bad= []
site4 bad= []
site5 bad= []
```

Ran: site-sane PySnobal summary:
`artifacts/pysnobal_site_sane_summary.{json,md}`. Route:

```text
PROCEED-SNOWFROST-FIDELITY-G1-SANE-SITE-LANES
```

Selected-lane results:

```text
site1 PASS max SWE 158.297797 kg/m2 max depth 0.983461 m observed MAE 0.219126 m Py-openWEPP MAE 0.229419 m
site2 PASS max SWE 170.679186 kg/m2 max depth 1.022643 m observed MAE 0.342636 m Py-openWEPP MAE 0.267813 m
site3 PASS max SWE 68.914979 kg/m2 max depth 0.578651 m observed MAE unavailable Py-openWEPP MAE 0.037192 m
site4 PASS max SWE 82.799097 kg/m2 max depth 0.571892 m observed MAE 0.015391 m Py-openWEPP MAE 0.044129 m
site5 PASS max SWE 102.472565 kg/m2 max depth 0.825613 m observed MAE unavailable Py-openWEPP MAE 0.056184 m
```

Ran: focused Morris `tg_neg0p5c_zg0p10m` full-lane run reproduced the original
PySnobal C-layer abort. Artifact:
`artifacts/pysnobal_morris_failed_lane_summary.{json,md}`.

```text
[pysnobal/c_snobal/libsnobal/sati.c:17] ERROR: Input temperature (tk): -153.450833 is less than zero
```

Ran: focused Morris `tg_neg0p5c_zg0p10m` window
`1980-01-01` through `1980-01-31` passed. Artifact:
`artifacts/pysnobal_morris_failed_lane_window_1980_01.{json,md}`.

```text
PROCEED-SNOWFROST-FIDELITY-G
max SWE 10.125579 kg/m2; max depth 0.106678 m
```

Disposition: the Morris `Tg=-0.5 degC` full-run lane remains a failed
sensitivity probe in PySnobal under the constant-ground-temperature proxy. It
does not block G1's site-sane comparator route because every site has a passing
selected lane, and PySnobal is diagnostic hypothesis evidence rather than
correctness authority.
