# CRAP Before

Evidence label: Static/Ran.

Source artifact: `/tmp/openwepp-cqr-nightly-crap.json`

Filter:

```sh
jq -r --arg t 'crates/openwepp-runner/src/bin/openwepp-snowbench.rs' '.entries | map(select(.file | endswith($t))) | sort_by(.crap) | reverse | .[] | [.function,.line,.cyclomatic,.coverage,.crap] | @tsv' /tmp/openwepp-cqr-nightly-crap.json
```

Ran: command above, exit `0`.

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `run` | `17` | `30.0` | `0.0` | `930.0` |
| `run` | `17` | `30.0` | `0.0` | `930.0` |
| `run_jennings_phase_args` | `189` | `17.0` | `0.0` | `306.0` |
| `run_jennings_phase_args` | `189` | `17.0` | `0.0` | `306.0` |
| `run_export_pysnobal` | `88` | `4.0` | `0.0` | `20.0` |
| `run_export_pysnobal` | `88` | `4.0` | `0.0` | `20.0` |
| `run_coe_melt` | `143` | `3.0` | `0.0` | `12.0` |
| `run_physics_bulk` | `115` | `3.0` | `0.0` | `12.0` |
| `run_coe_melt` | `143` | `3.0` | `0.0` | `12.0` |
| `run_physics_bulk` | `115` | `3.0` | `0.0` | `12.0` |
| `run_coe_bound_density` | `167` | `2.0` | `0.0` | `6.0` |
| `main` | `10` | `2.0` | `0.0` | `6.0` |
| `run_coe_bound_density` | `167` | `2.0` | `0.0` | `6.0` |
| `main` | `10` | `2.0` | `0.0` | `6.0` |
| `print_help` | `255` | `1.0` | `0.0` | `2.0` |
| `next_path` | `246` | `1.0` | `0.0` | `2.0` |
| `print_help` | `255` | `1.0` | `0.0` | `2.0` |
| `next_path` | `246` | `1.0` | `0.0` | `2.0` |

Rows are duplicated by cargo-crap entry format for this binary target. The owned
closure target is every eligible production function above CRAP `30`, namely:

- `run` (`930.0`)
- `run_jennings_phase_args` (`306.0`)
