# CRAP After

Ran: `cargo crap` exited `0`; target has `0` rows above `30`. Maximum target
CRAP is `25.625` (`wat_values_from_columns`). Selected rows changed:

```text
cargo crap --workspace \
  --lcov /tmp/openwepp-cqr-20260711-t02-characterized.lcov \
  --min 0 --format json \
  --output /tmp/openwepp-cqr-20260711-t02-characterized-crap.json
```

| Function | Before | After |
|---|---:|---:|
| `WatershedWatPublicationError::fmt` | 72.000 | 8.000 |
| `build_watershed_daily_rows_from_wat` | 110.000 | 10.000 |
| `read_wat_file_into` | 56.000 | 7.323 |

JSON SHA-256:
`186b93b45f68fb49490e9c82e3a2ce3a8e5a4beb2a0e3cbdaf430a0faeed3237`.
