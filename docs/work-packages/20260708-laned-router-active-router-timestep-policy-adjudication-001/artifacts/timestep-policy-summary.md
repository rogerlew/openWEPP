# Timestep Policy Run Summary

Status: RUN-COMPLETION-PASS. Evidence mode: Ran.

The status above means every requested process run completed. The day-792
timestep-policy verdict is recorded separately in
`timestep-policy-adjudication.md`.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `8876fa04ca520126b958d83a7c5777da6f793e51fba4c346432f065b31647aaa`
- Git HEAD: `07a12de694040e0e30edc714f297cfdc79a67674`
- Git status short:

```text
M crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 M crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs
 M crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs
 M crates/openwepp-hillslope-orchestrator/src/lib.rs
 M crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
 M crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
 M crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
 M crates/openwepp-runner/src/hillslope/laned_active.rs
 M docs/ROADMAP.md
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
 M docs/work-packages/README.md
?? docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/
```

Material run environment is recorded per rung in the JSON summary under
`material_environment`. The runner forces active routing, active trace
output, opt-in selected day/lane step trace, shadow profiling, rung-specific
`OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M`, and diagnostic
`OPENWEPP_LANED_ACTIVE_MAX_DT_S`; this package also supplied
`OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1` and
`OPENWEPP_LANED_ACTIVE_STEP_TRACE=1`.

| Member | Rung | dx m | max dt s | Status | Failure phase | Failure day | Clamp/source | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |
|---|---|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| mn_corn_h4 | dx1p25_dt300 | 1.25 | 300.0 | PASS | n/a | n/a | n/a | 0:05.20 | 5.18 | 149657 | 2557 | 4473.220365201316 | 37.24660078880961 | 9.371413833285201 | 0.0 |
| mn_corn_h4 | dx1p25_dt150 | 1.25 | 150.0 | PASS | n/a | n/a | n/a | 0:05.39 | 5.37 | 178103 | 2557 | 4473.209875833921 | 37.25709015620071 | 9.363971540085819 | 0.0 |
| mn_corn_h4 | dx1p25_dt75 | 1.25 | 75.0 | PASS | n/a | n/a | n/a | 0:06.20 | 6.17 | 242245 | 2557 | 4473.157108191842 | 37.30985779828603 | 9.337578661889781 | 0.0 |
| mn_corn_h4 | dx0p625_dt300 | 0.625 | 300.0 | PASS | n/a | n/a | n/a | 0:22.11 | 22.08 | 319784 | 2557 | 4473.1528822893215 | 37.31408370078956 | 9.35568629157479 | 0.0 |
| mn_corn_h4 | dx0p625_dt150 | 0.625 | 150.0 | PASS | n/a | n/a | n/a | 0:23.18 | 23.15 | 349886 | 2557 | 4473.168865098979 | 37.298100891148046 | 9.36201503173697 | 0.0 |
| mn_corn_h4 | dx0p625_dt75 | 0.625 | 75.0 | PASS | n/a | n/a | n/a | 0:24.04 | 24.01 | 408066 | 2557 | 4473.149797972949 | 37.317168017179355 | 9.371314723467988 | 0.0 |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/timestep-policy-summary.json`
