# Correction Authority Envelope

Status: `EXECUTED`

Defect ID: `WSHED-W4-HOLD-001`.

Observed failure:

- Public watershed CLI routed through
  `WatershedNetworkFrame::compatibility_writeback_surface()`.
- Production dispatch called `execute_watershed_dispatch_with_kernel`.
- Publication harvested compatibility report state instead of typed routed
  frame state.

Correction:

- Added `execute_watershed_dispatch_with_frame`.
- Added direct typed WS10/WS11/WS12/WS18/WS20 kernel execution over
  `WatershedNetworkFrame`.
- Refactored WS20 segment routing into a shared core used by both legacy and
  direct typed routes.
- Added `publish_typed_routing_report` with fail-closed missing-routed-state
  checks.
- Switched public watershed CLI to the frame-native dispatch and publication
  path.

Protected boundaries:

- No output schema redesign.
- No silent domain clamp or guard loosening.
- No W5 deletion claim for old compatibility code.
- No carnivorous-adobo output identity claim, because that fixture is not a
  current CLI E2E fixture with TOML/HBP bindings.

Acceptance:

- Public CLI routes without `compatibility_writeback_surface`.
- Direct production route does not use old surface/request/writeback markers.
- Public output behavior tests and full workspace gates passed.
