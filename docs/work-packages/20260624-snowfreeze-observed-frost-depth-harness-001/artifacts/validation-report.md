# Validation Report

Evidence class: Ran.

Status: harness validation complete; frost-defect attribution remains
`UNRESOLVED` pending modeled snow-depth diagnostics and direct-runtime surface
blocker follow-up for sites 3 and 4.

## Executed Comparisons

Commands:

```bash
.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  compare --site site1_sleepers_south_field_vt \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfreeze_observed_compare_site1_direct

.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  compare --site site2_sleepers_w9_hardwood_vt \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfreeze_observed_compare_site2_direct

.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  compare --site site3_scan_mandan_nd \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfreeze_observed_compare_site3_direct

.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  compare --site site4_ggd498_morris_mn \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfreeze_observed_compare_site4_direct

.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  compare --site site5_reynolds_creek_us_rls_id \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfreeze_observed_compare_site5_direct
```

Default runtime surface: `direct-production-executor`.

## Results

| Site | Method | Snow control | Matched obs | Frost residual rows | Isotherm upper-bound rows | Max abs frost residual (m) | Verdict |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| `site1_sleepers_south_field_vt` | frost tube | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | 392 / 392 | 392 | 0 | 0.2641958259 | `UNRESOLVED` |
| `site2_sleepers_w9_hardwood_vt` | frost tube | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | 200 / 200 | 200 | 0 | 0.3838127879 | `UNRESOLVED` |
| `site3_scan_mandan_nd` | soil-temperature isotherm | n/a | n/a | n/a | n/a | n/a | `HARNESS-SURFACE-MISMATCH` |
| `site4_ggd498_morris_mn` | frost tube | n/a | n/a | n/a | n/a | n/a | `HARNESS-SURFACE-MISMATCH` |
| `site5_reynolds_creek_us_rls_id` | soil-temperature isotherm | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | 4,356 / 4,356 | 0 | 4,356 | n/a | `UNRESOLVED` |

Direct-runtime blocker details:

- Site 3 failed at lane 1 day 487:
  `storage_reconciliation.frost_storage_projection_theta_m must be nonnegative`.
- Site 4 failed at lane 1 day 10727:
  `storage_reconciliation.frost_storage_projection_theta_m must be nonnegative`.

## Interpretation

- The harness now runs the direct-production executor by default; compatibility
  is not used as the acceptance target.
- Frost-tube sources produce direct magnitude residuals only where modeled rows
  align to observation dates.
- Soil-temperature sources produce upper-bound/timing metrics only; they do not
  publish max/mean frost-depth residuals.
- Right-censored sensor-depth caps are retained in the normalized corpus and
  excluded from magnitude/upper-bound residual metrics.
- The site1/site2 residuals and site5 upper-bound exceedances are intentionally
  not classified as `OPENWEPP-DEFECTIVE`: `INV-SNOWFREEZE-047` requires modeled
  snow-depth control before frost-depth disagreement can be attributed to
  openWEPP.
- WAT `Snow-Water` is SWE, not snow depth; all comparison reports retain
  `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC`.

## Follow-Up Need

Next frost-depth fidelity work should:

1. expose a contract-approved modeled snow-depth diagnostic for `TOL-SNOWFREEZE-009`;
2. resolve the direct-runtime storage-reconciliation guard failures for sites 3
   and 4 without changing observation harness semantics;
3. rerun all acquired sites through the same direct harness before assigning any
   `OPENWEPP-DEFECTIVE` verdicts.
