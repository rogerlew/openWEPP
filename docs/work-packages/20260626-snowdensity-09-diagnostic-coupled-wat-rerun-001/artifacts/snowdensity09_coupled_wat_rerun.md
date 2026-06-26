# SNOWDENSITY-09 Diagnostic Coupled WAT Rerun

- Disposition: `COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-BLOCKED`
- Blocker: `NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`
- SNOTEL density gate cleared: `True`
- Coupled opt-in WAT path available: `True`
- Opt-in snow-control passed: `False`
- Frost attribution authorized: `False`
- Default snow-control counts: `{'MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW': 2, 'SNOW_CONTROL_FAILED': 3}`
- Default snow-control gate counts: `{'SNOW_CONTROL_FAILED': 3}`
- Opt-in snow-control counts: `{'MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW': 2, 'SNOW_CONTROL_FAILED': 3}`
- Opt-in snow-control gate counts: `{'SNOW_CONTROL_FAILED': 3}`
- Diagnostic-only out-of-gate sites: `['site3_scan_mandan_nd', 'site5_reynolds_creek_us_rls_id']`
- Trace proof: `{'default_trace_row_count': 75610, 'opt_in_trace_row_count': 75610, 'default_models': {'legacy_wepp': 75610}, 'opt_in_models': {'physics_bulk_density_compaction_v1': 75610}, 'default_trace_legacy_count': 75610, 'default_trace_opt_in_count': 0, 'opt_in_trace_selected_count': 75610}`

## Site Deltas

| Site | Default status | Opt-in status | Default mean snow residual m | Opt-in mean snow residual m |
|---|---|---|---:|---:|
| site1_sleepers_south_field_vt | `SNOW_CONTROL_FAILED` | `SNOW_CONTROL_FAILED` | 0.41081660940626946 | 0.29639692828086844 |
| site2_sleepers_w9_hardwood_vt | `SNOW_CONTROL_FAILED` | `SNOW_CONTROL_FAILED` | 0.32327623539008665 | 0.21025928372492458 |
| site4_ggd498_morris_mn | `SNOW_CONTROL_FAILED` | `SNOW_CONTROL_FAILED` | 0.0672051635094675 | 0.058572644425492884 |
| site3_scan_mandan_nd | `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` | `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` | None | None |
| site5_reynolds_creek_us_rls_id | `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` | `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW` | None | None |
