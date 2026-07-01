# CRAP Before

Evidence mode: Ran.

Source:

- Reused final post-row-6 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row6-after.json`.

Extraction:

```text
jq -r '.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/direct_runtime/(subsurface|03_executor)\\.rs$")) | select(.crap > 30) | [.crap, .cyclomatic, (.coverage // -1), .file, .line, .function] | @tsv' /tmp/openwepp-crap-row6-after.json | sort -nr
```

Result:

- Row #8 duplicated report rows above CRAP 30: `4`.
- Row #8 unique production offenders above CRAP 30: `2`.

Offenders:

| Location | Function | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs:107` | `maybe_write_r7h_percolation_trace` | 8.0 | 5.71 | 61.64 |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs:188` | `maybe_write_r7h_subsurface_saturation_trace` | 8.0 | 7.02 | 59.45 |

Disposition: baseline recorded.
