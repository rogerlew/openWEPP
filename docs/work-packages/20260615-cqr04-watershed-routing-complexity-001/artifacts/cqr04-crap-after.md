# CQR04 CRAP After

Ran:

```text
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/artifacts/lcov_after.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/artifacts/crap_after.json
```

After summary:

- Unique target-file CRAP rows: 66.
- Maximum target-file CRAP: 30.0.
- Target-file rows with CRAP `> 30`: 0.
- Target-file rows with CRAP exactly `30.0`: 2.

Highest target-file rows after refactor:

| Function | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `ws20_case3_xdbeg_value` | 5 | 0.0 | 30.0 |
| `ws26_dcap_low_width_shear_outcome` | 5 | 0.0 | 30.0 |
| `ws20_flow_partition` | 10 | 51.162790697674424 | 21.648030990981923 |
| `ws20_case3_dl_lbs_s_ft2` | 4 | 0.0 | 20.0 |
| `ws20_case3_next_fluxes` | 4 | 0.0 | 20.0 |
| `ws20_case12_class_update` | 17 | 88.1578947368421 | 17.479937399766733 |
| `ws20_segment_hydraulics` | 14 | 74.19354838709677 | 17.36853412104327 |
| `ws23_detach_case4_iterative_closure` | 16 | 90.36144578313254 | 16.22923221409371 |
| `ws18_trncap_terminal_result` | 16 | 93.47826086956522 | 16.071011753102656 |
| `ws18_hydchn` | 11 | 65.82278481012658 | 15.830536732388312 |

Disposition: CRAP target pass. The two exactly-30 zero-covered rows are not
threshold failures but are recorded as residual test debt in the coverage hold.
