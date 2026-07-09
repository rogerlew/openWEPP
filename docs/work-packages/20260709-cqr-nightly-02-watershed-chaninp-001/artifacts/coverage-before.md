# Coverage Before

Ran: coverage values copied from target CRAP rows in
`/tmp/openwepp-cqr-nightly-crap.json`; raw LCOV path is
`/tmp/openwepp-cqr-nightly.lcov`.

Target:
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`

LCOV source-file summary from the baseline artifact:

- `LF: 1214`
- `LH: 308`
- Line coverage: `25.370675453047777%`
- `BRF: 0`
- `BRH: 0`

Target CRAP rows by function coverage:

| Function | Line | Coverage reported by cargo-crap |
|---|---:|---:|
| `sample_riser_unsubmerged_curve` | 573 | `0.0` |
| `rockfill_discharge_at_stage` | 1072 | `0.0` |
| `derive_riser_apr_coefficients` | 514 | `0.0` |
| `derive_ws12_active_structure_projection` | 799 | `41.5929203539823` |
| `fit_quartic_least_squares` | 693 | `0.0` |
| `solve_linear_system_5x5` | 754 | `0.0` |
| `emergency_discharge_at_stage` | 1161 | `0.0` |
| `perforated_riser_reference_discharge` | 1308 | `18.181818181818183` |
| `compute_riser_qs` | 639 | `0.0` |
| `culvert_pipe_discharge_at_stage` | 1020 | `0.0` |
| `project_culvert_function_families` | 260 | `14.285714285714285` |
| `interpolate_rating_curve_discharge` | 1213 | `0.0` |
| `project_emergency_function` | 375 | `18.0` |
| `filter_barrier_discharge_at_stage` | 1265 | `0.0` |
| `project_riser_functions` | 460 | `20.0` |
| `project_drop_spillway_function_families` | 149 | `32.608695652173914` |
| `derive_power_law_curve_coefficients` | 1378 | `60.0` |
| `project_rockfill_function` | 331 | `20.0` |

ADR-0021 coverage closure is required if this package adds or materially changes
characterization tests. The package tier is `science` because the target owns
WS12 impoundment runtime coefficient projection.
