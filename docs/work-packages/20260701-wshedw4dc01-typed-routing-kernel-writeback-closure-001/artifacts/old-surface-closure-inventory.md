# Old Surface Closure Inventory

Status: `EXECUTED`

Current classifications:

- Production routing read: none in public watershed CLI direct path.
- Production routing write: none in public watershed CLI direct path.
- Production publication read: typed routed frame state through
  `publish_typed_routing_report`.
- Compatibility/replay/comparator edge:
  `execute_watershed_dispatch_with_kernel`,
  `WatershedWritebackSurface`, and old request/writeback protocol remain for
  legacy contracts and compatibility tests.
- Diagnostic edge: none added by W4DC01.
- Test-only protected behavior:
  legacy WS10/WS11/WS12 integration contracts still exercise the old boundary.
- Obsolete-internal test: none reclassified in this package.

Production closure evidence:

- Public watershed CLI source contains no `compatibility_writeback_surface`,
  `harvest_compatibility_routing_report`, or
  `execute_watershed_dispatch_with_kernel`.
- Direct kernel source contains no `WatershedWritebackSurface`,
  `BoundarySymbol`, `BoundaryValue`, `KernelWritebackPayload`, or
  `WatershedKernelRequest`.
- W4 source guard enforces those conditions.

W5 boundary:

- This package does not delete the old compatibility code. W5 owns final
  deletion or retirement once remaining replay/contract surfaces migrate.
