# Line-Count Governance

Evidence mode: Ran.

Status: `PASS-WITH-WARN`.

Command:

```bash
wc -l \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs \
  crates/openwepp-hillslope-orchestrator/src/lib.rs \
  crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs \
  crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs \
  crates/openwepp-runner/src/hillslope/laned_active.rs \
  docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/run_shape_attribution_ladder.py \
  docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/analyze_day792_attribution.py
```

Result:

| File | Lines | Status |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 820 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 1262 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | 1350 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 183 | PASS |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 1456 | PASS |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2751 | WARN |
| `crates/openwepp-runner/src/hillslope/laned_active.rs` | 195 | PASS |
| `artifacts/run_shape_attribution_ladder.py` | 677 | PASS |
| `artifacts/analyze_day792_attribution.py` | 312 | PASS |

The existing builder module is above the 2000-line warning threshold but below
the 3000-line hard refactor threshold. This package made a narrow config
threading edit in that existing file to pass the trace-detail selector into the
runtime config. Refactoring the builder module is not part of this attribution
package; the appropriate follow-on is a mechanical builder split when broader
direct-publication builder work is next touched.
