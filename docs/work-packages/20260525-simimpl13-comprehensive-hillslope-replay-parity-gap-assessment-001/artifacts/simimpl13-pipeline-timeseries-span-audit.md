# Simimpl13 pipeline timeseries span audit

Status: phase-b-complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- Phase B objective for this artifact is end-to-end span assessment across
  `cli -> runner -> simulation/orchestrator` and explicit current-vs-required
  closure framing.
- Audit scope: `cli -> runner -> scheduler/kernel lifecycle -> WB13/H.wat`
  publication and replay comparability span.

## Ran
- Read runner CLI entrypoint and launch argument ingestion:
  - `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
- Read runner execution/publishing path:
  - `crates/openwepp-runner/src/lib.rs` (mode ingestion, lifecycle call,
    WB13/H.wat publication, manifest write)
- Read SIMIMPL11 replay evidence:
  - `candidate/openwepp_hillslope_run_manifest.json`
  - `candidate/H5.wat.dat`
  - `candidate/H5.wat.parquet`
  - `suite_dat/investigation/h5_wat_strict_comparator.json`
  - `suite_dat/investigation/h5_wat_semantic_comparator.json`
- Span probes:
  - baseline keyed rows: `1095`
  - candidate keyed rows: `1`
  - candidate parquet rows (`duckdb`): `1`

## Pipeline baseline map (Phase B)
| Stage | Observed behavior | Evidence |
|---|---|---|
| CLI command ingestion | `run-hillslope` parses explicit flags for binary, run dir/file, output dir, sidecar policy, and manifest path. | `crates/openwepp-runner/src/bin/open_wepp_runner.rs:40` |
| Sidecar/mode ingestion | `wepp_ui` is parsed and normalized into requested/effective mode plus selected lane provenance. | `crates/openwepp-runner/src/lib.rs:1524`; `crates/openwepp-runner/src/lib.rs:2410` |
| Lane selection/timestep policy | Effective `ui_run` maps to daily/hourly lane with guard checks; manifest records requested/effective/scheduler mode. | `crates/openwepp-runner/src/lib.rs:2469`; `...:2490`; manifest `mode_selection` + `timestep_policy` |
| Scheduler/kernel lifecycle | Runner calls canonical scheduler lifecycle and records `scheduler_kernel_executed=true`. | `crates/openwepp-runner/src/lib.rs:1716`; `...:2530`; manifest `execution_provenance` |
| WB13 row assembly | Publication path builds a single simulation-owned WB13 row from current runtime surface (`build_simulation_owned_wb13_row`). | `crates/openwepp-runner/src/lib.rs:2908` |
| H.wat publication span | Both `.dat` and `.parquet` writers consume `vec![single_row]`; no observed per-day row accumulation in this path. | `crates/openwepp-runner/src/lib.rs:2813`; `...:2816`; `...:1745` |
| Comparator-observed span result | Baseline 1095 keyed rows vs candidate 1 keyed row yields `common_row_count=0`. | strict + semantic comparator JSON in SIMIMPL11 replay bundle |

## Current vs required span behavior
| surface | current behavior | required promotable behavior | gap id |
|---|---|---|---|
| Candidate WB13 keyed trajectory | 1 row (`OFE=1,J=1,Y=2000`) | Full replay window keyed rows with overlap against baseline (`common_row_count>0`) | `SIMIMPL13-SPAN-001`, `SIMIMPL13-SPAN-002` |
| Candidate `.dat`/`.parquet` span parity | `.dat` conversion emits 1 row, `.parquet` stores 1 row | Span-equivalent dat/parquet candidate surfaces for strict + semantic lanes | `SIMIMPL13-SPAN-001`, `SIMIMPL13-COMP-004` |
| Runtime-to-publication linkage | Scheduler lifecycle executes, but kernel shim writeback yields single-row publication semantics | Lane execution must publish replay-length trajectory, not one-day projection-shaped output | `SIMIMPL13-SPAN-003` |

## Span gap register
| gap_id | statement | severity | owner surface | status |
|---|---|---|---|---|
| `SIMIMPL13-SPAN-001` | Candidate publication span is single-row (`count=1`) while baseline replay target span is multi-year daily trajectory (`1095` keyed rows). | blocker | runner WB13 publication lifecycle | open |
| `SIMIMPL13-SPAN-002` | Row-key domain mismatch prevents any overlap (`common_row_count=0`), including candidate key `(1,1,2000)` vs baseline simulation-year keys `(1,*,1..3)`. | blocker | runner key semantics + comparator key-policy | open |
| `SIMIMPL13-SPAN-003` | Current runner kernel shim (`RunnerDailyPhaseKernel`) returns empty writeback payload, leaving publication driven by projection-style single-row assembly instead of replay-length trajectory accumulation. | blocker | runner/orchestrator integration path | open |

## Phase B conclusion
- Mode/lane provenance and simulation-owned publication guards are present in
  manifests, but replay comparability remains blocked by span and key-domain
  non-overlap.
- Closure requires runtime publication span expansion plus key-domain alignment
  before comparator-level numeric parity can be interpreted.
