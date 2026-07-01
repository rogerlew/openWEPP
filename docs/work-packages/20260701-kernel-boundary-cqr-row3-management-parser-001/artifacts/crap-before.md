# CRAP Before

Evidence mode: Ran.

Source:

- Reused final post-row-8 full-workspace LCOV + CRAP JSON:
  `/tmp/openwepp-crap-row8-after.json`.

Extraction:

```text
jq -r '.entries[] | select(.file | test("/crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management\\.rs$")) | select(.crap > 30) | [.crap, .cyclomatic, (.coverage // -1), .file, .line, .function] | @tsv' /tmp/openwepp-crap-row8-after.json | sort -nr
```

Result:

- Row #3 duplicated report rows above CRAP 30: `2`.
- Row #3 unique production offenders above CRAP 30: `1`.

Offenders:

| Location | Function | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs:833` | `project_primary_drain_controls` | 9.0 | 35.56 | 30.68 |

Disposition: baseline recorded.
