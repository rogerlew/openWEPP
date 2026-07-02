# Source Guard Evidence

Status: `PASS`

Evidence class: `Static` and `Ran`.

Implemented guard:

- Test:
  `wshedw4_public_cli_handoff_uses_typed_network_and_publication_frames`.
- File:
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`.

Guard coverage:

- Requires public CLI markers:
  `WatershedNetworkFrame::from_parsed_inputs`,
  `network_frame.add_hillslope_contribution`,
  `execute_watershed_dispatch_with_frame`,
  `publish_typed_routing_report`, and `publication_frame_to_row_seed`.
- Forbids public CLI old-surface markers:
  `BoundarySymbol`, `BoundaryValue`, `WatershedWritebackSurface`,
  `compatibility_writeback_surface`, `harvest_compatibility_routing_report`,
  `execute_watershed_dispatch_with_kernel`, `.writeback_surface`,
  `state_surface.insert`, and `flux_surface.insert`.
- Extracts the typed dispatch body and forbids:
  `WatershedWritebackSurface`, `BoundarySymbol`, `BoundaryValue`,
  `KernelWritebackPayload`, `WatershedKernelRequest`, `state_surface`, and
  `flux_surface`.
- Scans `kernel/direct.rs` for the same forbidden old-surface/request markers.
- Requires direct physics call markers:
  `compute_muskingum_cunge_state`,
  `compute_variable_muskingum_cunge_state`,
  `route_impoundment_stage_over_duration`,
  `impoundment_outflow_at_stage`, `ws18_trncap`,
  `ws20_route_case12_segment_family_core`, and `direct_ws20_crfrac`.

Ran:

- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract
  wshedw4_public_cli_handoff_uses_typed_network_and_publication_frames --
  --nocapture`: passed.
- Final full behavior contract also passed all 24 tests.

Allowed old-surface paths:

- Legacy compatibility dispatch and old integration contracts remain in source
  as replay/contract surfaces.
- They are not on the public watershed CLI production route.
