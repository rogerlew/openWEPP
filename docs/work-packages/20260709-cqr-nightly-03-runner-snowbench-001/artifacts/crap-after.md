# CRAP After

Evidence label: Static/Ran.

Status: `FOCUSED-PASS`; full-workspace after-metrics delegated to
`comparator_suite_runner`.

Focused artifacts:

- `/tmp/openwepp-cqr-nightly-03-snowbench-focused.lcov`
- `/tmp/openwepp-cqr-nightly-03-snowbench-focused-crap.json`

Focused command:

```sh
cargo llvm-cov clean --workspace &&
cargo llvm-cov -p openwepp-runner --bin openwepp-snowbench --lcov --output-path /tmp/openwepp-cqr-nightly-03-snowbench-focused.lcov &&
cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-03-snowbench-focused.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-03-snowbench-focused-crap.json
```

Outcome: exit `0`. `cargo crap` warned that non-target workspace files had no
matching LCOV entries because the measurement was intentionally focused on the
`openwepp-snowbench` bin target.

## Target Rows

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `run_snowbench_command` | `50` | `13.0` | `97.77777777777777` | `13.001854595336077` |
| `apply_common_snowbench_flag` | `125` | `13.0` | `100.0` | `13.0` |
| `apply_jennings_phase_flag` | `303` | `11.0` | `100.0` | `11.0` |
| `run_export_pysnobal` | `174` | `4.0` | `41.66666666666667` | `7.175925925925925` |
| `main` | `10` | `2.0` | `0.0` | `6.0` |
| `run_jennings_phase_validation_args` | `328` | `5.0` | `100.0` | `5.0` |
| `run_with_args` | `21` | `4.0` | `70.0` | `4.432` |
| `run_coe_melt` | `229` | `3.0` | `47.61904761904761` | `4.29348882410107` |
| `run_physics_bulk` | `201` | `3.0` | `47.61904761904761` | `4.29348882410107` |
| `parse_jennings_phase_args` | `290` | `4.0` | `100.0` | `4.0` |
| `parse_common_snowbench_args` | `112` | `4.0` | `100.0` | `4.0` |
| `classify_top_level_command` | `36` | `4.0` | `100.0` | `4.0` |
| `run_coe_bound_density` | `253` | `2.0` | `84.21052631578947` | `2.015745735529961` |
| `parse_max_rows` | `319` | `2.0` | `100.0` | `2.0` |
| `run_jennings_phase_args` | `275` | `2.0` | `100.0` | `2.0` |
| `parse_coe_model` | `155` | `2.0` | `100.0` | `2.0` |
| `parse_physics_bulk_variant` | `146` | `2.0` | `100.0` | `2.0` |
| `run` | `17` | `1.0` | `0.0` | `2.0` |
| `print_help` | `365` | `1.0` | `100.0` | `1.0` |
| `next_path` | `356` | `1.0` | `100.0` | `1.0` |

Rows are duplicated by cargo-crap entry format for this binary target; duplicate
rows are omitted from the table above.

Disposition: every eligible production function in the target module is below
CRAP `30`. The former high rows closed from `run = 930.0` and
`run_jennings_phase_args = 306.0` to `run = 2.0` and
`run_jennings_phase_args = 2.0`.
