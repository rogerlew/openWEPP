# Shape Attribution Run Summary

Status: RUN-COMPLETION-PASS. Evidence mode: Ran.

The status above means every requested process run completed. The day-792
classification verdict is recorded separately in
`day792-attribution.md`.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `319fbe119e89193018ce9b2894dc7dab56babb7fee2543a0ec9f06f62674b56c`
- Git HEAD: `69813293686fcbdb7d46cfab02b5daa5d500d5d6`
- Git status short:

```text
M crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
 M crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs
 M crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs
 M crates/openwepp-hillslope-orchestrator/src/lib.rs
 M crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
 M crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs
 M crates/openwepp-runner/src/hillslope/laned_active.rs
 M docs/ROADMAP.md
 M docs/work-packages/README.md
?? docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/
```

Material run environment is recorded per rung in the JSON summary under
`material_environment`. The runner forces active routing, active trace
output, shadow profiling, and rung-specific
`OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M`; this package also supplied
`OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1`.

| Member | Rung | Status | Failure phase | Failure day | Clamp/source | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| mn_corn_h4 | dx2p5 | PASS | n/a | n/a | n/a | 0:01.38 | 1.35 | 71475 | 2557 | 4473.470955961825 | 36.996010028298144 | 9.33378447223084 | 0.0 |
| mn_corn_h4 | dx1p25 | PASS | n/a | n/a | n/a | 0:05.09 | 5.07 | 149657 | 2557 | 4473.220365201316 | 37.24660078880961 | 9.371413833285201 | 0.0 |
| mn_corn_h4 | dx0p625 | PASS | n/a | n/a | n/a | 0:22.07 | 22.05 | 319784 | 2557 | 4473.1528822893215 | 37.31408370078956 | 9.35568629157479 | 0.0 |

Comparisons:

| Member | Role | Candidate | Reference | Outlet L1 rel | Shape max L1 | Shape >0.05 | End storage rel | Tail fold rel | Annual sed max rel |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| mn_corn_h4 | fine_reference_adequacy | dx2p5 | dx1p25 | 5.85386e-05 | 0.0201805 | 0 | 5.55576e-05 | 8.34268e-06 | 0 |
| mn_corn_h4 | fine_reference_adequacy_dx1p25_vs_dx0p625 | dx1p25 | dx0p625 | 2.80622e-05 | 0.0209449 | 0 | 1.49614e-05 | 3.4869e-06 | 0 |
| mn_corn_h4 | candidate_vs_dx1p25_reference | dx2p5 | dx1p25 | 5.85386e-05 | 0.0201805 | 0 | 5.55576e-05 | 8.34268e-06 | 0 |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/shape-attribution-summary.json`
