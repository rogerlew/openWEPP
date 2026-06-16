# CQR25 CRAP Before

Status: complete.

Ran: generated baseline coverage:

```text
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001/artifacts/lcov_before.info
```

Ran: generated baseline CRAP report:

```text
cargo crap --workspace --lcov docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001/artifacts/crap_before.json
```

Ran: target-file baseline LCOV:

- Lines: `622/929`, `66.95%`.
- Functions: `22/75`, `29.33%`.

Ran: live baseline target identity from `crap_before.json`:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `execute_hillslope_run` | 764 | 113 | 75.29722589167768% | 305.483748671 |

Ran: next highest target-file rows were already below `30`:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `ExecutionLane::parse` | 612 | 4 | 62.5% | 4.84375 |
| `TimestepPolicy::scheduler_mode` | 654 | 4 | 83.33333333333334% | 4.07407407407 |
| `TimestepPolicy::policy_name` | 645 | 4 | 100% | 4 |
| `TimestepPolicy::timestep_seconds` | 663 | 4 | 100% | 4 |

Static: `cargo crap` reported duplicate rows for the target function in the
JSON. Values are identical; closure uses the duplicated value conservatively.
