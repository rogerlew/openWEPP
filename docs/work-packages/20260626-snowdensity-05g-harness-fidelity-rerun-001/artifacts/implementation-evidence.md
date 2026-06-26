# Implementation Evidence

Evidence class: Static + Ran.

## Contract

- `SC-SNOWFREEZE-001` advanced to v83.
- Added `INV-SNOWFREEZE-057`, `OBL-SNOWFREEZE-P-032`, and the
  SNOWDENSITY-05G addendum.
- The v83 contract records that 05G supersedes 05E's regime-limited
  promotion-candidate context with representative-regime `NON-PROMOTION`.

## Code

- `crates/openwepp-runner/src/hillslope/snowbench.rs`
  - `SnowbenchExportReport` now publishes
    `primary_canopy_cover_fraction`.
  - The value is sourced from generated openWEPP runtime surface symbol
    `cancov` and validated in `[0,1]`.
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
  - Diagnostic `coe-melt` replay now passes the configured canopy value into
    `DirectActiveSnowPartitionInputs.canopy_cover_fraction`.
  - Removed the old `DEFAULT_CANOPY_COVER_FRACTION = 0.0` path.
  - Report schema is `snowdensity05g-coe-melt-snowbench-v1`.
  - Reports now publish:
    - `canopy_source = generated_openwepp_runtime_surface.cancov`
    - `shortwave_source = pysnobal_bridge_inversion_of_openwepp_winter_hourly_rad_mj_m2`
    - `shortwave_bridge_identity`
    - `shortwave_bridge_like_for_like = true`
- `tools/snowfreeze_observed/coe_melt_adjudication.py`
  - Updated default output/schema/Markdown labels to SNOWDENSITY-05G.

## Representative Rerun

Ran:

```text
cargo build -q -p openwepp-runner --bin openwepp-snowbench
.venv/bin/python tools/snowfreeze_observed/coe_melt_adjudication.py \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowdensity05g_coe_melt_adjudication \
  --snowbench-binary target/debug/openwepp-snowbench
```

Result:

- `legacy_coe`: `robust_fail_count=9`, `robust_ordinal_score=84`.
- `coe_shortwave_albedo_v1`: `robust_fail_count=9`,
  `robust_ordinal_score=86`.
- Disposition: `NON-PROMOTION` because robust failures did not improve.
- All five `legacy_coe` and five `coe_shortwave_albedo_v1` site summaries
  reported `canopy_cover_fraction = 0.9` and
  `shortwave_bridge_like_for_like = true`.

