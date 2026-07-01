# Disposition

Status: `EXECUTED-HOLD-TYPED-ROUTING-KERNEL-WRITEBACK-REMAINS-COMPATIBILITY-EDGE`

Final disposition must be one of:

- `EXECUTED-COMPLETE-WSHED-W4`
- `EXECUTED-HOLD-<BLOCKER>`

Do not mark complete while any current-scope exit criterion is missing,
blocked, failed, unsupported by direct evidence, or deferred into W5/W6.

## Disposition

Result:
`EXECUTED-HOLD-TYPED-ROUTING-KERNEL-WRITEBACK-REMAINS-COMPATIBILITY-EDGE`

W4 landed a typed `WatershedNetworkFrame` and
`WatershedPublicationFrame` handoff in the real public
`openwepp-cli-watershed` path. The CLI now builds typed network/contribution
records from parsed watershed inputs and validated pass inventory, and
the public output writer consumes `WatershedPublicationFrame` rather than
directly reading `WatershedKernelExecutionReport.writeback_surface`.

The package cannot close complete because production routing still calls
`WatershedNetworkFrame::compatibility_writeback_surface()` and then
`execute_watershed_dispatch_with_kernel`. The WS10 dispatch/kernel path still
uses `WatershedWritebackSurface`, `BoundarySymbol`, `BoundaryValue`, and
`KernelWritebackPayload` for routing reads/writes. Typed publication is also
not final provenance: the typed frame is currently harvested from the
compatibility report and still inherits compatibility zero-default behavior for
missing routed operands.

## Completed

- Pre-edit old-surface inventory recorded.
- Pre-edit publication operand lineage recorded.
- Typed network frame, channel controls, impoundment controls, hillslope
  contributions, routed state harvest, and publication frame implemented.
- Public CLI no longer imports or directly manipulates
  `WatershedWritebackSurface`, `BoundarySymbol`, or `BoundaryValue`.
- Public publication path consumes typed publication operands.
- Independent reviewers completed and approved only the held disposition.
- W4 source guard added and passed.
- W2/W3 watershed CLI behavior and worker-pool output identity tests passed.

## Hold Blocker

First actionable follow-up item:

`close WSHED-W4 typed routing kernel/writeback blocker by replacing
execute_watershed_dispatch_with_kernel + KernelWritebackPayload application with
a WatershedNetworkFrame-native dispatch function that reads typed channel,
impoundment, dependency, and hillslope contribution records and writes typed
RoutedChannelState / RoutedImpoundmentState directly.`

Acceptance for hold lift:

- Public CLI routes without `compatibility_writeback_surface`.
- Production routing loop no longer reads/writes `WatershedWritebackSurface`,
  `BoundarySymbol`, or `BoundaryValue`.
- Remaining old-surface code is path-scoped as replay/comparator/diagnostic or
  obsolete-test code.
- Typed frame builders enforce the same fail-closed runtime-input domain
  guards currently enforced by the compatibility projection, or those guards
  are centralized and covered by typed tests.
- Typed publication harvest no longer silently defaults missing routed operands
  to zero unless canonical output authority explicitly permits that behavior.
- Committed-fixture protected output identity or contract-governed deltas are
  recorded.
- Independent reconstruction and conservation/magnitude audit are recorded for
  publication operands.
