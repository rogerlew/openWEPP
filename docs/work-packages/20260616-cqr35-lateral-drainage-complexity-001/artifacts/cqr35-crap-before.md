# CQR35 CRAP Before

Status: complete.

Ran:
`cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260616-cqr35-lateral-drainage-complexity-001/artifacts/lcov_before.info`

Ran:
`cargo crap --workspace --lcov docs/work-packages/20260616-cqr35-lateral-drainage-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260616-cqr35-lateral-drainage-complexity-001/artifacts/crap_before.json`

Ran: target-file LCOV for
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`:

| Metric | Covered | Total | Percent |
| --- | ---: | ---: | ---: |
| Lines | 1698 | 2122 | 80.02% |
| Functions | 79 | 87 | 90.80% |

Ran: highest unique target-file CRAP rows:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `Wb11HydrologyKernel::wb19_lateral_transfer_inputs` | 172 | 18.0 | 70.23809523809523% | 26.541362973760947 |
| `Wb11HydrologyKernel::wb19_drainage_inputs` | 1409 | 17.0 | 78.3132530120482% | 19.947685064543265 |
| `Wb11HydrologyKernel::wb19_lateral_layer_parameters` | 378 | 14.0 | 70.12987012987013% | 19.22356981861114 |
| `Wb11HydrologyKernel::wb19_lateral_lane_config` | 267 | 9.0 | 60.0% | 14.184000000000001 |
| `Wb11HydrologyKernel::wb14_effective_ks_9002_exponent` | 2457 | 6.0 | 44.73684210526316% | 12.075885697623558 |

Ran: target-file CRAP rows over `30`: `0`.

Warning: `cargo crap` reported 126 source files with no matching LCOV entry.
The target file was represented in LCOV.
