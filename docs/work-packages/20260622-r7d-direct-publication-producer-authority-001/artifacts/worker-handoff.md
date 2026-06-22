# Worker Handoff

Status: executed-held.

## Handoff

Blocker:
`HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT`.

Defect to close:
`R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY`.

What is proven:

- Production direct public outputs are written from
  `DirectRunPublicationFrame`; the direct-production branch does not use
  `execution.wb13_rows` as the consumer source.
- Focused one-OFE fixture is parity-clean for HBP, WAT, PASS, and loss.
- H2637 direct production emits direct-source rows with
  `compatibility_edge_invocations=0` and correct row counts.

What remains broken:

- H2637 HBP, WAT, and PASS are not parity-clean.
- `DirectProductionExecutor` builds topology/area-only direct lane frames.
- `DirectPublicationDayInputBuilder` seeds day inputs from one aggregate
  `HillslopeWritebackSurface`, so multi-OFE runs alias lane seed state.
- Per-OFE static surfaces exist in `OfeLanePersistentStateSequence`, but there
  is no direct typed constructor bridge from those surfaces into
  `DirectLaneConstructorInputs` or production direct day inputs.

First actionable implementation step:

1. Close defect `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY` by adding a typed
   lane-seed authority builder that converts parsed per-OFE soil, slope,
   management, PMET, snow, frost, layer, ET, transfer, geometry, and
   publication operands into `DirectLaneConstructorInputs` and lane-indexed
   day-input producers for `DirectProductionExecutor`.

Required follow-up gates:

- Add a multi-OFE anti-alias fixture that proves lane 1 and lane 2 direct seed
  operands differ before execution.
- Prove production direct day inputs no longer clone a single aggregate
  runtime surface.
- Re-run focused and H2637 HBP/WAT/PASS/loss/manifest parity.
- Re-run direct counter proof, including
  `compatibility_edge_invocations=0`.
