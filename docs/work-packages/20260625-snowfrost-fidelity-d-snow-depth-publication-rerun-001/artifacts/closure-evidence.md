# SNOWFROST-FIDELITY-D Closure Evidence

Evidence mode: Ran.

## Implementation Evidence

- Added WAT `Snow-Depth` as a nullable diagnostic column with `mm` units.
- Compatibility publication maps `Snow-Depth` from existing
  `snow.runtime_depth_m * 1000`.
- Direct publication maps `Snow-Depth` from existing winter-column
  `snow.runtime_depth_m * 1000`.
- WAT `Snow-Water` remains required SWE and is not used as a snow-depth proxy.
- No production snow/frost physics constants, Qwet/frzftp terms, or runtime
  activation defaults were changed.

## Observed Rerun Evidence

Command:

```bash
.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare \
  --site <site> \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfrost_fidelity_d_observed_compare/<site>
.venv/bin/python tools/snowfreeze_observed/classify_residuals.py \
  target/snowfrost_fidelity_d_observed_compare/*/comparison_report.json \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-json docs/work-packages/20260625-snowfrost-fidelity-d-snow-depth-publication-rerun-001/artifacts/residual_classification.json \
  --output-md docs/work-packages/20260625-snowfrost-fidelity-d-snow-depth-publication-rerun-001/artifacts/residual_classification.md
```

Results:

- Site count: `5`
- Modeled snow-depth diagnostic present: `5/5`
- `OPENWEPP-DEFECTIVE`: `0`
- Defect-attribution eligible sites: `0`
- Primary classifications:
  - `SNOW-CONTROL-FAILED`: `3`
  - `INCONCLUSIVE`: `2`

Site summaries:

| Site | Classification | Snow status | Snow pairs | Snow failures | Max snow residual m |
| --- | --- | --- | ---: | ---: | ---: |
| `site1_sleepers_south_field_vt` | `SNOW-CONTROL-FAILED` | `SNOW_CONTROL_FAILED` | `384` | `322` | `1.596821792509187` |
| `site2_sleepers_w9_hardwood_vt` | `SNOW-CONTROL-FAILED` | `SNOW_CONTROL_FAILED` | `193` | `143` | `1.059919954616471` |
| `site3_scan_mandan_nd` | `INCONCLUSIVE` | `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` | `0` | `0` | `n/a` |
| `site4_ggd498_morris_mn` | `SNOW-CONTROL-FAILED` | `SNOW_CONTROL_FAILED` | `83` | `28` | `0.392372927299844` |
| `site5_reynolds_creek_us_rls_id` | `INCONCLUSIVE` | `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` | `0` | `0` | `n/a` |

Disposition:

- SNOWFROST-FIDELITY-D clears the missing modeled snow-depth diagnostic blocker.
- Field frost-depth/isotherm residuals are still not eligible for
  openWEPP-defect attribution because paired snow control either fails or is
  unavailable.
- Next work must adjudicate snow-depth fidelity before heat-flow, frozen-K,
  SFCC, or Qwet/migration changes are authorized from these field residuals.

## Validation

One initial `cargo test --workspace` run failed because synthetic WB13
publication probe surfaces did not seed the newly required
`snow.runtime_depth_m` diagnostic. The shared probe seed was corrected with a
neutral snow-depth value, and the final gate set below passed.

- `cargo fmt`
- `cargo fmt --check`
- `cargo test -p openwepp-hillslope-output hillslope_wat`
- `cargo test --test snowfreeze_observed_frost_depth_contract`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/observed_harness.py tools/snowfreeze_observed/classify_residuals.py`
- `cargo test -p openwepp-runner wbval06_hillslope_wat_row_publishes_daily_interception_flux`
- `cargo test -p openwepp-runner r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss`
- `cargo test -p openwepp-runner direct_publication`
- `cargo test -p openwepp-runner --lib`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`
- `rg -n "qwet|frzftp" crates || true` produced no matches.
