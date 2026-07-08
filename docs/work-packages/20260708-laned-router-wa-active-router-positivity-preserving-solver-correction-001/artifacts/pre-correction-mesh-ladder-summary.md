# Mesh Ladder Summary

Status: PASS_EXPECTED_FAIL. Evidence mode: Ran.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `2331d10073cc4c0428d12b8a717d6e934e5eff14ba5fff07e56daa4a2b236579`
- Git HEAD: `3b01de4e6e957c4a0905d86d0267dac5a5b73524`
- Git status short:

```text
M docs/work-packages/README.md
?? docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/
```

| Member | Rung | Status | Failure phase | Failure day | Clamp/source | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| wa_cascades_forest_h1 | baseline_fixed10 | FAIL | laned_active_clamp_exceeds_source | 1418 | 14.291141234409194 | 0:09.82 | 9.81 | n/a | n/a | n/a | n/a | n/a | n/a |
| wa_cascades_forest_h1 | dx5 | FAIL | laned_active_clamp_exceeds_source | 1167 | 11335.893753002358 | 0:35.21 | 35.19 | n/a | n/a | n/a | n/a | n/a | n/a |

Comparisons:

| Member | Role | Candidate | Reference | Outlet L1 rel | Shape max L1 | Shape >0.05 | End storage rel | Tail fold rel | Annual sed max rel |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| wa_cascades_forest_h1 | fine_reference_adequacy: dx2p5 or dx1p25 reference rung failed or lacks trace output | SKIPPED | SKIPPED | n/a | n/a | n/a | n/a | n/a | n/a |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/mesh-ladder-summary.json`
