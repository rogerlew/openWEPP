# Old Surface Inventory

Status: `QUEUED`

W4 execution must inventory current `WatershedWritebackSurface`,
`BoundarySymbol`, and `BoundaryValue` use before production edits.

Required classifications:

- production routing read:
- production routing write:
- production publication read:
- compatibility projection:
- replay/comparator/diagnostic edge:
- test-only protected behavior:
- obsolete-internal test:

The package cannot close complete while current-scope production routing or
publication still depends on the old symbol-map surface.
