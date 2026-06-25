# SNOWFROST-FIDELITY-E Closure Evidence

Evidence mode: Ran.

## Implementation Evidence

- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-048` and contract version `72`.
- Added WAT `Snow-Depth` variable authority as physical snowpack depth from
  `snow.runtime_depth_m`; WAT `Snow-Water` remains SWE and is invalid as a
  snow-depth proxy.
- Extended observed comparison reports with:
  - full paired snow-depth residual rows;
  - mean/median/min/max signed residuals;
  - modeled-over/under-observed counts;
  - adjacent-day timing/stage rescue checks;
  - depth-vs-SWE anti-alias residuals.
- Added `tools/snowfreeze_observed/snow_depth_audit.py`, bound to
  `SC-SNOWFREEZE-001 INV-SNOWFREEZE-048`.
- Added regression coverage for the new invariant, route labels, and signed
  snow-depth diagnostics.
- No production snow/frost physics constants, process equations, runtime
  activation defaults, `Qwet`, or `frzftp` terms were added or changed.

## Observed Rerun Evidence

Command family:

```bash
cargo build -p openwepp-runner --bin openwepp-cli-hill
.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate \
  --observations-dir tests/fixtures/snowfreeze_observed/observations
.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare \
  --site <site> \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-dir target/snowfrost_fidelity_e_observed_compare/<site> \
  --binary target/debug/openwepp-cli-hill
.venv/bin/python tools/snowfreeze_observed/classify_residuals.py \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-json docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/artifacts/residual_classification.json \
  --output-md docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/artifacts/residual_classification.md \
  target/snowfrost_fidelity_e_observed_compare/*/comparison_report.json
.venv/bin/python tools/snowfreeze_observed/snow_depth_audit.py \
  --observations-dir tests/fixtures/snowfreeze_observed/observations \
  --output-json docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/artifacts/snow_depth_audit.json \
  --output-md docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/artifacts/snow_depth_audit.md \
  target/snowfrost_fidelity_e_observed_compare/*/comparison_report.json
```

Residual classification:

- Site count: `5`
- Defect-attribution eligible sites: `0`
- `OPENWEPP-DEFECTIVE`: `0`
- Primary classifications:
  - `SNOW-CONTROL-FAILED`: `3`
  - `INCONCLUSIVE`: `2`

Snow-depth audit:

| Site | Route | Direction | Pairs | Failures | Timing rescues | SWE alias better | Mean signed m | Max abs m |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `site1_sleepers_south_field_vt` | `SNOW-DEPTH-FIDELITY-ISSUE` | `dominant-modeled-over-observed` | `384` | `322` | `4` | `298` | `0.41081660940626946` | `1.596821792509187` |
| `site2_sleepers_w9_hardwood_vt` | `SNOW-DEPTH-FIDELITY-ISSUE` | `dominant-modeled-over-observed` | `193` | `143` | `5` | `109` | `0.32327623539008665` | `1.059919954616471` |
| `site3_scan_mandan_nd` | `INSUFFICIENT-PAIRED-SNOW-DATA` | `no-paired-residuals` | `0` | `0` | `0` | `0` | `n/a` | `n/a` |
| `site4_ggd498_morris_mn` | `SNOW-DEPTH-FIDELITY-ISSUE` | `dominant-modeled-over-observed` | `83` | `28` | `2` | `39` | `0.0672051635094675` | `0.392372927299844` |
| `site5_reynolds_creek_us_rls_id` | `INSUFFICIENT-PAIRED-SNOW-DATA` | `no-paired-residuals` | `0` | `0` | `0` | `0` | `n/a` | `n/a` |

Disposition:

- Snow-depth correspondence is sufficiently proven for Sites 1, 2, and 4:
  source provenance identifies physical snow depth, modeled lineage is WAT
  `Snow-Depth` from `snow.runtime_depth_m`, adjacent-day timing does not rescue
  failures, and SWE remains an invalid depth proxy.
- The paired-snow sites route to snow-depth fidelity, not frost heat-flow,
  frozen-K/SFCC, impedance, or migration/fringe work.
- Sites 3 and 5 remain unusable for snow-control until paired observed
  snow-depth rows exist.

## Validation

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate --observations-dir tests/fixtures/snowfreeze_observed/observations`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/observed_harness.py tools/snowfreeze_observed/classify_residuals.py tools/snowfreeze_observed/snow_depth_audit.py`
- `cargo test --test snowfreeze_observed_frost_depth_contract`
- `cargo fmt`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`
- `git diff --check`
- `rg -n "qwet|Qwet|frzftp" crates || true` produced no matches.

Line-count governance:

- `tests/integration/snowfreeze_observed_frost_depth_contract.rs`: `283` lines.
  No touched `.rs` file exceeds the warning threshold.
