# CRAP Before

Evidence mode: Ran.

Source:

- Reused final post-row-5 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row5-after.json`.

Extraction:

```text
jq -r '.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/direct_runtime/(growth|decomposition)\\.rs$")) | select(.crap > 30) | [.crap, .cyclomatic, (.coverage // -1), .file, .line, .function] | @tsv' /tmp/openwepp-crap-row5-after.json | sort -nr
```

Result:

- Row #6 duplicated report rows above CRAP 30: `4`.
- Row #6 unique production offenders above CRAP 30: `2`.

Offenders:

| Location | Function | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:426` | `DirectGrowthInputs::validate_schedule_domain` | 37.0 | 35.59 | 402.76 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs:518` | `DirectGrowthInputs::validate_equation_inputs` | 30.0 | 92.50 | 30.38 |

Disposition: baseline recorded.
