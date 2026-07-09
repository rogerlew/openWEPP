# CRAP After

Status: `COMPLETE`

Target:
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`

Final CRAP command:

- `cargo crap --workspace --lcov /tmp/openwepp-cqr02-final-local-after.lcov --min 0 --format json --output /tmp/openwepp-cqr02-final-local-after-crap.json`
- Log: `artifacts/logs/final-local-crap.log`
- SHA-256:
  `4271b950b449ea18de1d61baf8c3884ef2c0ebaa5fa851e66e525f42f4d1f290`
- Exit code: `0`
- Artifact: `/tmp/openwepp-cqr02-final-local-after-crap.json`

Summary:

- Unique target rows: `33`.
- Rows above CRAP `30`: `0`.
- Rows above CRAP `25`: `0`.
- Max target CRAP: `20.816276483846725`.
- Lowest function coverage row:
  `sample_riser_unsubmerged_curve`, line `578`, coverage
  `76.36363636363637%`, CRAP `20.816276483846725`.

Top target rows after implementation:

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `sample_riser_unsubmerged_curve` | 578 | 17.0 | 76.36363636363637 | 20.816276483846725 |
| `derive_power_law_curve_coefficients` | 1431 | 18.0 | 84.44444444444444 | 19.219555555555555 |
| `rockfill_discharge_at_stage` | 1125 | 17.0 | 87.71929824561403 | 17.53526321189246 |
| `derive_culvert_like_active_projection` | 866 | 14.0 | 78.43137254901961 | 15.966634250778355 |
| `derive_ws12_impoundment_coefficients` | 61 | 13.0 | 98.36065573770492 | 13.000744555711712 |
| `derive_riser_apr_coefficients_from_points` | 521 | 13.0 | 100.0 | 13.0 |
| `perforated_riser_reference_discharge` | 1361 | 12.0 | 86.36363636363636 | 12.365138993238167 |
| `emergency_discharge_at_stage` | 1214 | 11.0 | 85.1063829787234 | 11.39974764743843 |
| `fit_quartic_least_squares` | 698 | 11.0 | 97.82608695652173 | 11.001243116626942 |
| `solve_linear_system_5x5` | 759 | 11.0 | 100.0 | 11.0 |
| `project_drop_spillway_function_families` | 148 | 11.0 | 100.0 | 11.0 |
| `project_emergency_function` | 374 | 10.0 | 100.0 | 10.0 |
| `project_culvert_function_families` | 259 | 10.0 | 100.0 | 10.0 |
| `collect_culvert_like_stage_thresholds` | 924 | 10.0 | 100.0 | 10.0 |
| `project_riser_functions` | 459 | 9.0 | 97.5 | 9.001265625 |
| `derive_ws12_active_structure_projection` | 803 | 9.0 | 100.0 | 9.0 |
| `culvert_pipe_discharge_at_stage` | 1073 | 9.0 | 100.0 | 9.0 |
| `compute_riser_qs` | 644 | 9.0 | 100.0 | 9.0 |
