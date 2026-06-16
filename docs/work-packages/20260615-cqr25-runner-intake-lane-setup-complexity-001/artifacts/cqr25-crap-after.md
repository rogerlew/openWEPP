# CQR25 CRAP After

Status: complete-with-warnings.

Ran: regenerated final after coverage after the trace-config carry-through
cleanup:

```text
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001/artifacts/lcov_after.info
```

Ran: regenerated final after CRAP:

```text
cargo crap --workspace --lcov docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001/artifacts/crap_after.json
```

Ran: target-file after LCOV:

- Lines: `1134/1425`, `79.58%`.
- Functions: `75/123`, `60.98%`.

Ran: final target identity from `crap_after.json`:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `execute_hillslope_run` | 2292 | 12 | 85.71428571428571% | 12.4198250729 |

Ran: highest target-file rows after refactor:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `execute_hillslope_run` | 2292 | 12 | 85.71428571428571% | 12.4198250729 |
| `annotate_day_runtime_error` | 1901 | 3 | 0% | 12 |
| `build_static_runtime_surface_parts` | 1486 | 8 | 65% | 10.744 |
| `write_hillslope_optional_outputs` | 2070 | 7 | 58.333333333333336% | 10.5445601852 |
| `write_hillslope_run_manifest` | 2149 | 9 | 76.31578947368422% | 10.0761226126 |
| `execute_hillslope_climate_days` | 1631 | 9 | 85.86956521739131% | 9.22853440659 |

Ran: no target-file CRAP row is above `30`.

Static: `cargo crap` emitted LCOV source-map warnings for 126 workspace
test/support source files. The CQR25 source file has before/after LCOV and CRAP
entries and the warning does not affect target closure.
