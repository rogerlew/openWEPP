# Simimpl01 pipeline gap audit

Status: phase-c-complete
Evidence mode: Static + Ran

## Static
- Audit scope: production hillslope path `cli -> runner -> simulation -> orchestration`.
- Objective: determine whether the production runner path executes typed scheduler/kernel lifecycle before output publication, and whether `wepp_ui` branch intent controls lane selection.

## Ran
- CLI/runner probes:
  - `rg -n "execute_hillslope_run" crates/openwepp-runner/src/bin/openwepp-cli-hill.rs crates/openwepp-runner/src/lib.rs`
  - `sed -n '1180,1560p' crates/openwepp-runner/src/lib.rs`
  - `sed -n '2145,2436p' crates/openwepp-runner/src/lib.rs`
- Orchestrator probes:
  - `rg -n "HillslopePhaseScheduler|execute_with_kernel|Wb11HydrologyKernel" crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - `sed -n '8879,9371p' crates/openwepp-hillslope-orchestrator/src/lib.rs`
- Branch/authority probes:
  - `rg -n "requested_mode|effective_mode|wbk09_hourly_qcap_policy" /workdir/wepp-forest/fpm-src/watbal_*.f90 /workdir/wepp-forest/fpm-src/watbal_process_kernels.f90`

## Stage-by-stage pipeline findings
| Stage | Observed production behavior | Gap statement | Evidence anchors |
|---|---|---|---|
| CLI entry (`openwepp-cli-hill`) | CLI forwards request to `execute_hillslope_run` and returns report | No primary gap at CLI argument transfer layer | `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs:3`, `:86` |
| Runner parse/assembly | Runner parses core files and sidecars, merges runtime surfaces | Parse/assembly works, but mode/result objects are not used to drive execution lane | `crates/openwepp-runner/src/lib.rs:1220-1455`; `wepp_ui` parse assigned to `_wepp_ui` at `:1278-1289` and `:1373-1384` |
| Runner execution/publication | Runner builds pass/WAT outputs from `build_first_day_wat_projection` synthesis | Production publication is projection-first rather than scheduler-executed simulation output | `crates/openwepp-runner/src/lib.rs:1463-1493`; helpers `:2145-2268`, `:2355-2431` |
| Orchestrator capability | Typed scheduler and kernel execution APIs exist with writeback governance | Capability exists, but no evidence of production runner invoking `execute_with_kernel` | scheduler API `crates/openwepp-hillslope-orchestrator/src/lib.rs:8879-9371`; runner imports only writeback surface `crates/openwepp-runner/src/lib.rs:14`, merge helper `:2137-2139` |

## Explicit branch closure finding (`wepp_ui`)
- Legacy authority expects runtime branch selection between daily and hourly paths (`watbal` -> `watbal_hourly`).
- In current runner path, `wepp_ui` parsing is validated but parse output is not propagated into runtime lane selection.
- Consolidated candidate architecture expresses explicit `requested_mode`/`effective_mode` mode fields and hourly policy guards, reinforcing the missing runtime closure in openWEPP runner.

Evidence:
- Legacy branch: `/workdir/wepp-forest_260430_baseline/src/watbal.for:253`
- Runner parsed-but-unconsumed mode: `crates/openwepp-runner/src/lib.rs:1278-1289`, `:1373-1384`
- Consolidated candidate mode-bearing adapters:
  - `/workdir/wepp-forest/fpm-src/watbal_daily_adapter.f90:222-223`
  - `/workdir/wepp-forest/fpm-src/watbal_hourly_adapter.f90:299`, `:325-326`, `:475-477`
  - `/workdir/wepp-forest/fpm-src/watbal_process_kernels.f90:148`, `:2071`, `:2077`

## Pipeline gap IDs confirmed
- `GAP-SIMPIPE-001`: runner path does not execute scheduler/kernel lifecycle before output emission.
- `GAP-SIMMODE-001`: `wepp_ui` requested/effective mode not propagated to runtime lane selection.
- `GAP-SIMOUT-001`: WB13/H.wat publication remains projection/synthesis-first.
- `GAP-SIMREPLAY-001`: replay key mismatch remains expected under projection-first publication.

## Phase C conclusion
- openWEPP contains typed kernel/scheduler components, but production hillslope output publication is not yet connected to that execution path.
- This is a wiring and authority-ownership closure problem, not merely a parser availability problem.
- SIMIMPL01 queue sequencing correctly prioritizes contract amendments/tests and runner-orchestrator integration before hourly closure and replay re-run.
