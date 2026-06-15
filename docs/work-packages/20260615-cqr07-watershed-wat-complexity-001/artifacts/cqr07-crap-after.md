# CRAP After

Ran: after metrics were generated with:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/artifacts/lcov_after.info && cargo crap --workspace --lcov docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/artifacts/crap_after.json
```

Ran: command exit code was `0`. `cargo crap` emitted the known workspace warning
about `124` source files with no matching LCOV entry.

Current-scope rows:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `read_batch_into` | 482 | 4.0 | 100.0 | 4.0 |
| `read_wat_file_row` | 497 | 6.0 | 100.0 | 6.0 |
| `positive_area_m2` | 513 | 3.0 | 100.0 | 3.0 |
| `day_key_from_columns` | 531 | 7.0 | 95.0 | 7.006125 |
| `wat_values_from_columns` | 554 | 25.0 | 90.0 | 25.625 |
| `WatBatchColumns::load` | 264 | 3.0 | 100.0 | 3.0 |
| `WatIdentityColumns::load` | 273 | 10.0 | 100.0 | 10.0 |
| `WatValueColumns::load` | 289 | 25.0 | 100.0 | 25.0 |

Pre-existing out-of-scope WARN rows:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `build_watershed_daily_rows_from_wat` | 319 | 10.0 | 0.0 | 110.0 |
| `WatershedWatPublicationError::fmt` | 46 | 8.0 | 0.0 | 72.0 |
| `read_wat_file_into` | 417 | 7.0 | 0.0 | 56.0 |

Static: raw data is in `crap_after.json`.

Disposition: scoped CRAP target closed; broader module CRAP WARN remains.
