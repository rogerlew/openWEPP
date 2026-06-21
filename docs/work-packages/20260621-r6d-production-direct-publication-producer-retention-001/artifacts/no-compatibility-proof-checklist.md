# No-Compatibility Proof Checklist

Evidence mode: Static + Ran.

Accepted retained direct publication producers must not read:

- `SimulationOwnedWb13Row`;
- compatibility `HillslopeWritebackSurface` publication values;
- `KernelWritebackPayload`;
- runtime symbols as direct publication authority;
- stale logical state;
- wrappers around those structures.

## Source Scans

Ran:

- `awk '/fn retain_direct_publication_day_rows/,/^    fn publish_persistent_day_result/' crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs | rg -n "SimulationOwnedWb13Row|HillslopeWritebackSurface|KernelWritebackPayload|runtime_surface|wb13|writeback|stale|compat"` -> PASS, no matches.
- `awk '/fn build_retained_direct_publication_frame/,/^fn annotate_day_runtime_error/' crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs | rg -n "SimulationOwnedWb13Row|HillslopeWritebackSurface|KernelWritebackPayload|runtime_surface|wb13|writeback|stale|compat"` -> PASS, no matches.
- `sed -n '2204,2218p' crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs | rg -n "DirectRunFrame::skeleton|DirectFrameExecutor|run_publication_capture|wb13|runtime_surface|writeback|KernelWritebackPayload"` -> PASS, no matches in the cutover branch.
- `sed -n '2219,2240p' crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs | rg -n "DirectRunFrame::skeleton|DirectFrameExecutor|run_publication_capture"` -> PASS, matches only in `DirectPublicationFrameShadow`, not cutover.

## Static Finding

The retained R6D producer path reads parsed climate/calendar fields and slope
geometry. It does not read compatibility WB13 rows, compatibility runtime
publication surfaces, writeback payloads, stale logical state, or wrappers
around those structures.

The cutover artifact path consumes `execution.retained_direct_publication` and
does not construct a skeleton direct frame or run post-hoc publication capture.
