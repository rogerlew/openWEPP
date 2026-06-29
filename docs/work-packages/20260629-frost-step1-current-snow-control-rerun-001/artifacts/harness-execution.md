# Harness Execution Evidence

Evidence mode: Ran.

## Build

- Command: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- Result: pass.
- Git revision: `663292a850a555a1264b9da49866ac8c877cda68`
- Binary: `target/release/openwepp-cli-hill`
- SHA-256: `9f12ff55a50faaa90664cee1d5a169680caa2588bdd7b6ea0f9115b6ed3050bc`

## Corpus Validation

- Command: `.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate`
- Result: pass.

## Current Default Snow Surface

No snow selector environment overrides were supplied. The direct-production
no-env snow default is the activated bundle recorded in
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-075`:

- `snow_melt_model = coe_liquid_holding_capacity_v1`
- `snow_density_model = physics_bulk_density_compaction_v1`
- `snow_phase_partition_model = harder_pomeroy_hourly`

## Site Runs

All commands used `--binary target/release/openwepp-cli-hill` and the
direct-production executor default in `observed_harness.py compare`.

| Site | Output directory | Result |
| --- | --- | --- |
| `site1_sleepers_south_field_vt` | `target/frost_step1_current_snow_control/site1` | pass |
| `site2_sleepers_w9_hardwood_vt` | `target/frost_step1_current_snow_control/site2` | pass |
| `site3_scan_mandan_nd` | `target/frost_step1_current_snow_control/site3` | pass |
| `site4_ggd498_morris_mn` | `target/frost_step1_current_snow_control/site4` | pass |
| `site5_reynolds_creek_us_rls_id` | `target/frost_step1_current_snow_control/site5` | pass |

Compact comparison reports were copied into `artifacts/site_reports/`.

## Analysis Commands

- Legacy scalar audit:
  `.venv/bin/python tools/snowfreeze_observed/snow_depth_audit.py ...`
- Legacy residual classifier:
  `.venv/bin/python tools/snowfreeze_observed/classify_residuals.py ...`
- Step 1 forcing-robust routing:
  `.venv/bin/python artifacts/route_current_snow_control.py --reports-root target/frost_step1_current_snow_control ...`

## Primary Result

The `INV-SNOWFREEZE-048` scalar gate still fails at three paired sites, but
`INV-SNOWFREEZE-050` routes those failures by attribution risk:

- `FORCING-LIMITED`: Sleepers South and Sleepers W9.
- `BLOCKED`: Morris.
- `INCONCLUSIVE-NO-PAIRED-SNOW`: Mandan and Reynolds Creek.
