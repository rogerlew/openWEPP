# CRAP Before

Ran: before metrics were generated with:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/artifacts/lcov_before.info && cargo crap --workspace --lcov docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/artifacts/crap_before.json
```

Ran: command exit code was `0`. `cargo crap` emitted the known workspace warning
about `124` source files with no matching LCOV entry.

Target rows:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `read_batch_into` | 380 | 69.0 | 0.0 | 4830.0 |
| `build_watershed_daily_rows_from_wat` | 216 | 10.0 | 0.0 | 110.0 |
| `WatershedWatPublicationError::fmt` | 46 | 8.0 | 0.0 | 72.0 |
| `read_wat_file_into` | 314 | 7.0 | 0.0 | 56.0 |
| `f64_column_any` | 573 | 5.0 | 0.0 | 30.0 |
| `optional_int16_column_any` | 558 | 5.0 | 0.0 | 30.0 |

Static: raw data is in `crap_before.json`.

Disposition: baseline target hotspot was `read_batch_into`.
