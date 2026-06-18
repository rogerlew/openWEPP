# PERFARCH03 Logical-Free Proof

Evidence class: Static + Ran.

Status: pass for the measured hot loop.

## Static boundary

The prototype still uses logical maps and symbols for setup, production-baseline
execution, validation, and one-shot materialization. Those uses are intentional
and are outside the timed array floor loop.

The measured hot loop is:

- `time_array_hot_loop`
- `array_runoff_physics`
- `OutputPlan::write_outputs`
- branch helper functions such as `compute_interval_infiltration_depth`,
  `compute_canopy_interception_depth`, and
  `resolve_interception_rainfall_scale`

That hot loop uses dense `SymbolId` slots and `Vec<Option<BoundaryValue>>`
surfaces. It does not build a logical payload, use `BoundarySymbol` lookups, use
`BTreeMap`, call `format!`, or call `from_logical_payload`.

## Perf command

```bash
perf record -o /tmp/perfarch03-array-only.data --call-graph fp \
  docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/artifacts/perfarch03-floor-prototype/target/release/perfarch03-floor-prototype \
  array-only 10000000

perf report -i /tmp/perfarch03-array-only.data \
  --stdio --no-children --percent-limit 0.01 --sort symbol \
  > /tmp/perfarch03-array-only.limit001.report
```

Raw report copied to `perfarch03-array-only-perf-report.txt`.

`perf` reported:

```text
Total Lost Samples: 0
Samples: 53K of event 'cycles:P'
```

Kernel address maps were restricted by `kptr_restrict`, so kernel samples are
shown as unresolved addresses. User-space hot-loop symbols are resolved.

## Thresholded profile

Top user-space rows in the thresholded report:

| Symbol | Overhead |
|---|---:|
| `perfarch03_floor_prototype::time_array_hot_loop` | 81.02% |
| unresolved kernel sample bucket under the same call stacks | 11.91% |
| `perfarch03_floor_prototype::array_runoff_physics` | 2.76% |
| `perfarch03_floor_prototype::compute_interval_infiltration_depth` | 1.77% |
| `perfarch03_floor_prototype::compute_canopy_interception_depth` | 1.38% |
| `perfarch03_floor_prototype::resolve_interception_rainfall_scale` | 0.67% |

Logical-symbol scan on the thresholded report:

```bash
rg -n "BTreeMap|from_logical_payload|format!|BoundarySymbol|alloc::collections::btree|__memcmp" \
  /tmp/perfarch03-array-only.limit001.report
```

Result: no matches.

The unthresholded report contained only 0.00% setup/noise entries for unrelated
runtime symbols. No logical map, symbol-formatting, or logical-payload
conversion symbol appeared in the measured hot loop at or above the 0.01%
reporting threshold.
