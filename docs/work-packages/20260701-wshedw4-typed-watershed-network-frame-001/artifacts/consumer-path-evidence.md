# Consumer-Path Evidence

Status: `EXECUTED-HOLD`

Evidence class: `Static:` plus `Ran:`

W4 consumer-path proof for the real public `openwepp-cli-watershed` runner.

- Producer source: `WatershedNetworkFrame::from_parsed_inputs` builds typed
  routing globals, channel controls, impoundment controls, and the frame object
  from parsed watershed inputs in
  `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`.
- Typed network frame object: the public CLI constructs `WatershedNetworkFrame`
  after parsing topology/channel/slope/impoundment/optional `chan.inp` and
  before pass-inventory validation.
- Typed hillslope contribution handoff: the public CLI reads validated
  `PassInventory` entries, validates MOFE metadata, builds
  `HillslopeContribution` records, and calls
  `network_frame.add_hillslope_contribution`.
- Downstream routing consumer: **held**. The public CLI currently calls
  `network_frame.compatibility_writeback_surface()` and then
  `execute_watershed_dispatch_with_kernel`. This is an explicitly named
  compatibility projection, not typed production routing completion.
- Typed publication frame object:
  `network_frame.harvest_compatibility_routing_report(&report)` harvests the
  compatibility kernel report into typed routed channel/impoundment state and
  returns `WatershedPublicationFrame`.
- Downstream publication consumer: the public CLI calls
  `publication_frame_to_row_seed(&publication_frame)` before
  `write_watershed_interchange_outputs`; it no longer calls
  `build_watershed_output_row_seed(&report)`.
- Output/API surface: the public `openwepp-cli-watershed` path and existing
  watershed interchange Parquet outputs remain unchanged.
- Positive evidence: `cargo test -p openwepp-runner --test
  watershed_cli_behavior_contract wshedw4 -- --nocapture` passed the W4 source
  guard; the full `watershed_cli_behavior_contract` suite passed `24` tests.
- Negative proof for publication: the public CLI no longer imports
  `BoundarySymbol`, `BoundaryValue`, `WatershedWritebackSurface`, or reads
  `.writeback_surface` for output publication.
- Negative proof for routing: **not satisfied**. The route stage still consumes
  the old map protocol through `compatibility_writeback_surface`,
  `execute_watershed_dispatch_with_kernel`, and the existing WS10 kernel
  request/writeback helpers.
- What still reads the old path:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
  compatibility projection/harvest, `src/lib_mod/dispatch.rs`, and
  `src/lib_mod/kernel/**`.

Consumer-path disposition: typed publication consumer is active in the real
public output writer path, but typed publication provenance is still harvested
from the compatibility report. Typed routing consumer is not complete. W4 must
hold until the production routing loop consumes `WatershedNetworkFrame`
directly and no longer uses `WatershedWritebackSurface`, `BoundarySymbol`, or
`BoundaryValue` for the W4 claim.
