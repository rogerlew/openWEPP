# Observed Run Evidence

Evidence mode: Ran.

Fresh output root:
`target/snowfrost_fidelity_a_observed_compare/`.

## Commands

Build:

```bash
cargo build -p openwepp-runner --bin openwepp-cli-hill
```

Observed comparisons:

```bash
.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare \
  --site site1_sleepers_south_field_vt \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfrost_fidelity_a_observed_compare/site1_sleepers_south_field_vt

.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare \
  --site site2_sleepers_w9_hardwood_vt \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfrost_fidelity_a_observed_compare/site2_sleepers_w9_hardwood_vt

.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare \
  --site site3_scan_mandan_nd \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfrost_fidelity_a_observed_compare/site3_scan_mandan_nd

.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare \
  --site site4_ggd498_morris_mn \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfrost_fidelity_a_observed_compare/site4_ggd498_morris_mn

.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare \
  --site site5_reynolds_creek_us_rls_id \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfrost_fidelity_a_observed_compare/site5_reynolds_creek_us_rls_id
```

All five comparison commands exited `0` and emitted
`comparison_report.json`.

## Report Summary

| Site | Harness verdict | Snow-control status | Matched rows | Frost residual rows | Max abs frost residual (m) | Isotherm upper-bound rows | Isotherm exceedances |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| `site1_sleepers_south_field_vt` | `UNRESOLVED` | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | 392 | 392 | 0.2641958258624707 | 0 | 0 |
| `site2_sleepers_w9_hardwood_vt` | `UNRESOLVED` | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | 200 | 200 | 0.3838127878666539 | 0 | 0 |
| `site3_scan_mandan_nd` | `UNRESOLVED` | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | 10643 | 0 | n/a | 10583 | 3452 |
| `site4_ggd498_morris_mn` | `UNRESOLVED` | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | 83 | 83 | 0.990389751515789 | 0 | 0 |
| `site5_reynolds_creek_us_rls_id` | `UNRESOLVED` | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | 4356 | 0 | n/a | 4356 | 104 |

## Evidence Interpretation

- The prior site3/site4 storage-reconciliation blocker is not present in this
  run; both sites emit metric-bearing reports.
- The harness remains intentionally `UNRESOLVED` because modeled snow depth is
  absent.
- WAT `Snow-Water` is SWE and is not a snow-depth diagnostic; it cannot satisfy
  `TOL-SNOWFREEZE-009`.
- The site3/site5 isotherm exceedances are timing/upper-bound signals, not
  frost-depth magnitude failures.
- No site is eligible for `OPENWEPP-DEFECTIVE` frost attribution in this
  package.
