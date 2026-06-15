# CQR04 CRAP Before

Ran:

```text
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/artifacts/lcov_before.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/artifacts/crap_before.json
```

Top target-file rows before refactor:

| Function | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `ws20_route_case12_segment_family` | 125 | 70.43740573152338 | 528.6896871629501 |
| `ws26_dcap` | 29 | 71.77914110429448 | 47.9019529425293 |
| `ws18_hydchn` | 21 | 64.1025641025641 | 41.399939311181924 |
| `ws23_detach_case4_iterative_closure` | 34 | 83.41968911917098 | 39.269092733581054 |
| `ws18_trncap` | 34 | 93.57798165137615 | 34.30617692731205 |

Baseline target-file functions with CRAP `> 30`: 5.

Raw artifacts:

- `lcov_before.info`
- `crap_before.json`
- `coverage_before_summary.json`
