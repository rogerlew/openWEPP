# Source and consumer map

Status: `PHASE 1 COMPLETE — OBSERVE EXISTING HARNESS; NO SOURCE PATCH`

Evidence mode: `Static + Ran`

No Rust path was edited. The following exact source/consumer edges were used by
the fallback probe and cover the authentic producer-to-consumer path:

| Boundary | Exact source anchor | Authenticated observation |
|---|---|---|
| Test entry and original fixture | `crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs:835-1110` (`stage3_laned_release_one_ofe_positive_baseline_profile`) | Creates the existing one-OFE lane-D fixture at lines 846-850, authors the existing `CompleteOwner` seed at 861-869, and invokes the real runner at 880-888. |
| Runner intake/dispatch | `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:2160-2217` | `execute_hillslope_run_with_runtime_policy` loads the runfile, resolves output transaction targets, selects `DirectProductionExecutor`, and calls `execute_selected_hillslope_days`; no alternate replay selector is reachable. |
| Stage-3 day execution and committed snapshot | `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:112-121`, `:530-535` | Direct production day execution runs the retained transaction; test-only qualification snapshot is captured once after archive sealing, before output transaction finish. |
| Support chronology | `crates/openwepp-runner/src/hillslope/03_tests.rs:47-49`, `crates/openwepp-runner/src/hillslope/snow_stage3_v11_qualification_audit.rs:35-75` | The test module includes the two release files. The thread-local audit records day/support order and the committed snapshot, then is taken after the runner returns. |
| Adaptive parent lifecycle | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs:208-360`, `:482-515` | `AdaptiveParentTelemetryV1` records direct/composed trials, split children, accepted widths, rejection categories, fixed-point and phase timing, retained-owner bytes, and publication/reuse counters. The guard is result-blind and bounded by completed-parent count/time. |
| Release phase attribution | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_profile.rs:189-330`, `:344-405` | Thread-local nested profiler reports total, native-vegetation/ET envelope, Stage-3/LSE remainder, Lane-D, and remaining-runner durations; `take_release_qualification_telemetry_v1` charges the final scope and checks balance. |
| Terminal/provider lifecycle | `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs:405-730` | The retained provider builds beginning owners, exact support projections, typed child identity, complete Stage-3 stack, carrier physical/complete phases, and accepted transition; this is the real consumer path, not a shadow-only producer. |
| Covered carrier endpoint | `crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs:1208-1217`, `:1379-1405` | The accepted Stage-3 carrier projects the completed covered endpoint and retains the typed native carrier/soil custody used by the real terminal consumer. |
| Strict V8 entry | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint.rs:614-675` | The strict endpoint calls `project_v8_runtime_inputs_with_carriers`, then `project_native_frozen_litter_v3_solver_inputs`; no diagnostic producer is substituted. |
| V8 occupancy/soil projection | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_input_projection.rs:858-890`, `:991-1298` | The projection validates ordered occupancies and maps every configured stratum/occupancy one-for-one to covered solver rows, root identities, and soil layers; the live frame's six-layer topology is rebound here. |
| Native V3/V3 multi-tile adoption | `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v3_multitile_adoption.rs:1094-1165` | The accepted native V3 covered tile is selected for the forest-litter consumer and remains a typed pre-ingress candidate until adoption. |
| LSE transaction/solver consumer | `crates/openwepp-land-surface-energy/src/transaction.rs:515-540`, `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs:461-500` | The retained consumer calls the canonical covered solver; it has no component-replay implementation. The historical recovered v31 candidate (not retained or executed) derived replay topology from `occupancies.len()` and `ground.soil_nodes.len()` at `/tmp/v31_recover/...solver_covered_solve.rs:1100-1117`. |
| LSE/Jacobian lifecycle | `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs:202-322` and `crates/openwepp-land-surface-energy/src/solver_tests.rs:1191-1660` | Existing test-only probe audits distinguish complete evaluator calls from dependency-reused identity-anchor probes and forced-complete differential oracles. Current production release telemetry does not claim component replay counts. |
| Historical v31 audit-return boundary | Unstripped cached binary symbol `openwepp_land_surface_energy::solver::take_covered_component_dependency_replay_audit` with debugger return breakpoint; aggregator symbol `openwepp_runner::...::aggregate_component_replay_audit` | Read-only GDB capture decoded the authentic audit before the post-run assertion: 2,000 sweeps, all completed/not-failed, with `48/14/10/24` or `54/14/16/24` partitions and no target rows. A separate lifecycle capture distinguished the authentic take from the forced-complete oracle take. |
| Historical RSS endpoint | `crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs:1112-1118` | `release_probe_rss_kib()` reads `/proc/self/status`, finds `VmRSS:`, parses the first numeric field in KiB, and is called while report/telemetry/output locals remain live at JSON construction (`:1070-1107`), before `remove_dir_all` (`:1109`). |
| Scale RSS observer | `crates/openwepp-runner/src/hillslope/tests03/stage3_long_run_qualification.rs:444-505`, `:706-716`, `:924-973` | Existing scale harness samples current `VmRSS` every millisecond in a helper thread, captures process-lifetime `VmHWM` after the run, and reports the maximum of active samples and HWM. This is diagnostic scale infrastructure, not historical endpoint semantics. |
| Output/closure | `stage3_runner_qualification.rs:916-953` and `stage3_long_run_qualification.rs:1030-1175` | HBP/WAT/PASS/parquet/manifest outputs are parsed and exact Lane-D source/outlet/storage/clamp closure is asserted before RSS endpoint/cleanup. |

The source map resolves no permitted source edit: the existing ignored release
probe and scale sampler already expose the required current-run observations,
and adding a new observer would change the measurement treatment without
recoverable candidate source. Any future source patch must first rerun
`tools/agents/find-agents --for <exact path>` and amend the package write set;
this package performed no such edit.
