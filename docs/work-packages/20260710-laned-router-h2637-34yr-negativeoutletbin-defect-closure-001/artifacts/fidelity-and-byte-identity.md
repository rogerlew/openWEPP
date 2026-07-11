# Fidelity and Byte Identity

Status: `EXECUTED`

Evidence mode: `Static + Ran`

## Real Consumer Path

The accepted endpoint used `openwepp-cli-hill`, which selected the
`direct-production-executor`. Static inspection and the executed manifests
establish this exact downstream chain:

1. `resolve_laned_active_enabled` selects complete native route-coefficient
   authority, and `execute_direct_publication_stream` installs the resulting
   `DirectLanedActiveConfig` in `DirectRunFrame.laned_active`
   (`openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`).
2. `DirectFrameExecutor::run_publication_stream_with_interleaved_day_inputs_and_day_frames`
   observes that config and calls `run_laned_active_publication_stream`
   (`direct_runtime/03_executor.rs`). The in-memory `DirectDayFrame` supplies
   `runoff_shadow_projection.q_runoff_m`, `wb14_hourly_excess_m`,
   `subsurface_compute_shadow_projection.hourly_saturation_carry_m`, and
   `snow_coupling_downstream_operands.hourly_routed_melt_m` to
   `laned_active_lane_source`.
3. The executor passes each `DirectDayFrame` and `LanedActiveLaneSource` to
   `laned_active_route_lane`. That call converts the source depths to seam
   rates and invokes `route_single_ofe_with_step_trace`
   (`direct_runtime/laned_active.rs`).
4. `route_single_ofe_with_step_trace` constructs `KinematicWaveSolver` and
   calls `run_with_options_and_step_trace`; the returned
   `RoutingResult.outlet_bin_outflow_m2` is both the conservative
   `UpstreamHandoff` for the next lane and the source for
   `DirectLanedActiveDayRouting` plus `laned_active_routed_erosion_weights`
   (`ofe_routing/cascade.rs` and `direct_runtime/laned_active.rs`).
5. `laned_active_route_lane` sets
   `DirectErosionHydrographShapeAuthority::RoutedHydrograph` and writes the
   routed weights to
   `DirectDayFrame.erosion_inputs.routed_hydrograph_runoff_fraction`.
   `DirectDayFrame::r7d8_surface_hourly_weights` therefore dispatches to
   `r7d8_routed_hydrograph_hourly_weights`, which validates and consumes that
   array (`direct_runtime/erosion.rs`).
6. Only after active day-closure succeeds does the executor build and consume
   `DirectPublicationDayRow`, commit the frame, and feed the runner's HBP,
   loss-JSON, pass-parquet, plot-parquet, and water-parquet streaming outputs.

Static old-path check: `laned_active_assert_no_dc01_surface_feed` fails an
active lane if the old DC01 surface runon is nonzero, while the authority enum
forces the erosion consumer onto `RoutedHydrograph` rather than
`Dc01SourceShape`. The upstream `dc01_surface_runoff_hourly_weights` helper is
used only to reconstruct the authorized physical source-time distribution; it
is not the downstream routed-shape consumer. Executed negative proof: both
accepted manifests select `direct-production-executor`, carry an active
summary and one publication capture, and report `skeleton_runs = 0`,
`compatibility_edge_invocations = 0`, and no projection fallback.

This is downstream-consumer evidence, not a producer, counter, shadow, or
skeleton-only check.

## Independent Closure Evidence

The focused regression reconstructs final storage as `sum(h_i * dx)` from
the committed cell depths and subtracts the independently captured initial
storage. It then evaluates

`inflow + lateral source + clamp - outflow - reconstructed storage change`.

The result is required within `1e-15 m2`, separately from exact outlet-bin
sum versus booked scheme outflow. It does not reuse the solver ledger's
storage-change field. The rejected aliases and alternative formulas are
enumerated in `operand-lineage.md`.

On the full 34-year endpoint, both effective `wepp_ui` modes produced the
same active-routing closure block:

| Operand | Value |
| --- | ---: |
| source | `9359675.308593621 m3` |
| routed outlet | `9293061.586496634 m3` |
| end-window storage | `66613.72209678387 m3` |
| clamp | `1.2648059757016023e-12 m3` |
| maximum supply reconstruction residual | `1.3705073708810095e-15` |
| maximum daily cascade residual | `2.7267270905263873e-12` |
| maximum daily seam residual | `4.088168641495673e-13` |
| maximum daily identity residual | `2.736550138392646e-12` |

The two modes also produced identical HBP, pass-parquet, and water-parquet
bytes. Their loss and plot artifacts differ only across the deliberately
different UI run metadata/mode surfaces and are not used as a cross-mode
identity claim.

## Disabled-Path Byte Identity

The discovery audit's exact config-B staging was recovered. Its pre-fix
outputs were copied before rerunning the same TOML and inputs with the
post-fix binary and `OPENWEPP_LANED_ACTIVE_DISABLE=1`. The post-fix manifest
shows the active provenance block absent and `wepp_ui` requested/effective
`0`.

All five required outputs compare equal with `cmp` and retain the same
SHA-256 hashes:

| Output | SHA-256 | Result |
| --- | --- | --- |
| HBP | `8889869001979a9af2b1e047a526b95a15267d4aa9644435384b8dce5f753e98` | byte-identical |
| loss JSON | `fdc2c844162fa63dd5a916372b29abb0884ec1ac75d14cdaca7f1bf429ef1d61` | byte-identical |
| pass parquet | `30075819e24f7bc909de48951f08c9f2a5b1837440aacdb251cb93fdf7403586` | byte-identical |
| plot parquet | `17cd72b571f13daaf61ded2880bba09dc212ca8492fbb7a5fc909e59270e331b` | byte-identical |
| water parquet | `de734bc116223340b4f1dfd07067d3c57bc13da16512dd91d9a788509b0f2f3c` | byte-identical |

The disabled-path rerun exited `0` (user `39.16 s`, wall `39.28 s`, maximum
RSS `68860 KiB`). The correction is therefore isolated from the protected
daily/off runtime.
