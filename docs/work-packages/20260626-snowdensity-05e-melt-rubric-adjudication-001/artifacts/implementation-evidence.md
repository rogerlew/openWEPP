# SNOWDENSITY-05E Implementation Evidence

Evidence mode: Ran.

## Implemented

- Added diagnostic-only `openwepp-snowbench coe-melt`.
- Added `legacy_coe` and `coe_shortwave_albedo_v1` replay models.
- Reused the typed CoE melt helper for raw melt, redistributed melt, routed
  melt, SWE loss, albedo carry, and state mutation.
- Added `tools/snowfreeze_observed/coe_melt_adjudication.py` to run the
  five-site SNOTEL rubric profile.
- Added `tests/integration/snowdensity05e_melt_adjudication.rs`.
- Reran the non-SNOTEL rubric baseline after the 05D/05E snow-depth diagnostic
  and melt evidence were present.

## Boundaries

- No default activation.
- No production parser surface.
- No production output schema change.
- No coefficient fitting or SNOTEL-site constants.
- No shared radiation forcing adjustment.
- No `dense_slow_melt_v1` promotion.

## Correction During Review

The diagnostic replay initially reconstructed daily snow water input with a
fixed `0.1` factor. Current SNOTEL fixtures use `newsnw=100 kg/m3`, so the
initial and corrected profiles are numerically identical for this package, but
the bridge was still too fixture-specific. The final replay now carries
`snow_water_m` from the snowbench forcing CSV while still passing snowfall depth
to the typed snow helper.

## Source Scan Summary

Static scan command:

```text
rg -n "SnowMeltModel::CoeShortwaveAlbedoV1|SnowMeltModel::LegacyCoe|coe_shortwave_albedo_v1|dense_slow_melt_v1|snow_melt_model:" crates tools tests docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md -S
```

Result:

- Production direct day-input builder still seeds `SnowMeltModel::LegacyCoe`.
- `CoeShortwaveAlbedoV1` appears in the typed helper, 05C/05D tests, and the
  new diagnostic snowbench replay.
- `--model` is only on `openwepp-snowbench coe-melt`.
- `dense_slow_melt_v1` remains confined to snowbench physics-bulk diagnostics,
  tests, and contract negative-benchmark text.

## SNOTEL Adjudication

Artifact: `artifacts/snotel-adjudication.json`.

- Disposition: `PROMOTION-CANDIDATE`.
- Diagnostic legacy robust counts:
  `fail=13`, `marginal=14`, `pass=7`, `strong=11`, `unavailable=15`.
- Opt-in robust counts:
  `fail=10`, `marginal=8`, `pass=5`, `strong=22`, `unavailable=15`.
- Diagnostic legacy robust score: `61`.
- Opt-in robust score: `84`.
- H comparator context available:
  `legacy_as_built` and `openwepp_as_built` both reported
  `robust_fail_count=9`, `robust_ordinal_score=84`.

## Non-SNOTEL Baseline

Artifact: `artifacts/non-snotel-baseline.json`.

- Forcing-robust counts:
  `fail=9`, `marginal=7`, `pass=4`, `strong=20`, `unavailable=45`.
- Snow-control status:
  `SNOW_CONTROL_FAILED=3`,
  `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW=2`.
- `openwepp_defective_cells=0`.
- Next route remains snow-depth structural remediation before frost attribution.
