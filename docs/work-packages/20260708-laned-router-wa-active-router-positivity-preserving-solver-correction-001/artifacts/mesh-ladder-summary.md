# Mesh Ladder Summary

Status: PASS. Evidence mode: Ran.

Release binary:

- Build: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`
- Git HEAD: `3b01de4e6e957c4a0905d86d0267dac5a5b73524`
- Git status short:

```text
M crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
 M docs/work-packages/README.md
?? docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/
```

| Member | Rung | Status | Failure phase | Failure day | Clamp/source | Wall | User | Solver steps | Trace rows | Outlet m3 | End storage m3 | Tail fold m3 | Pass tdet sum |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| wa_cascades_forest_h1 | baseline_fixed10 | PASS | n/a | n/a | n/a | 0:16.01 | 15.93 | 3284995 | 10960 | 860565.5924347457 | 4835.117072647566 | 8875.01105406181 | 35134.70249005277 |
| wa_cascades_forest_h1 | dx5 | PASS | n/a | n/a | n/a | 1:02.82 | 62.77 | 4891877 | 10960 | 860530.122226401 | 4870.587280992438 | 8913.530961605135 | 35147.595053719335 |

Comparisons:

| Member | Role | Candidate | Reference | Outlet L1 rel | Shape max L1 | Shape >0.05 | End storage rel | Tail fold rel | Annual sed max rel |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| wa_cascades_forest_h1 | fine_reference_adequacy: dx2p5 or dx1p25 reference rung failed or lacks trace output | SKIPPED | SKIPPED | n/a | n/a | n/a | n/a | n/a | n/a |

Detailed JSON:

- `/home/workdir/openWEPP/docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/mesh-ladder-summary.json`
