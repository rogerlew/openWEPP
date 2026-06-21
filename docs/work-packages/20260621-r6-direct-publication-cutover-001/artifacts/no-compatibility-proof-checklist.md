# No-Compatibility Proof Checklist

Status: executed-hold.
Evidence mode: Static + Ran.

## Forbidden Direct-Publication Reads

After cutover, direct publication must not read:

- `HillslopeWritebackSurface`
- `KernelWritebackPayload`
- `BoundarySymbol`
- `BoundaryValue`
- `SymbolRegistry`
- `HotSymbolTables`
- `IndexedWritebackSurface`
- dense refresh state
- dirty flush state
- stale logical output frames
- compatibility diagnostic ledgers as publication authority

## Current Proof

Ran:

- `cargo test -p openwepp-runner r6_ -- --nocapture`: PASS. The focused
  cutover-candidate test builds direct publication artifacts and records
  `publication_capture_runs == 1`, `skeleton_runs == 0`, and
  `compatibility_edge_invocations == 0` inside the direct publication executor.
- `cargo test -p openwepp-runner r6a_ -- --nocapture`: PASS. Existing R6A
  tests still prove direct projection consumers read `DirectRunPublicationFrame`
  operands and default compatibility does not enter publication capture.

Static:

- Direct output projection helpers consume `DirectRunPublicationFrame`, not
  `SimulationOwnedWb13Row`, `HillslopeWritebackSurface`, or runtime symbols.
- The cutover candidate output branch is guarded by
  `DirectPublicationFrameCutover`.
- The parity gate intentionally reads compatibility publication products only
  as validation evidence before any direct public output write. This is not a
  production cutover proof.

## Remaining Compatibility Reads

BLOCKED:

- Main run execution still runs the compatibility scheduler before direct
  publication artifacts are built.
- HBP parity rebuilds compatibility HBP from `execution.wb13_rows` and
  `execution.runtime_surface`.
- WAT parity rebuilds compatibility rows from `execution.wb13_rows`.
- PASS parity compares to `execution.pass_rows`.
- The production manifest writer still constructs provenance and checksums
  through compatibility-oriented manifest structures.

## Gate

BLOCKED. R6 has direct projection consumers and a fail-closed writer boundary,
but it does not yet have a production no-compatibility proof for HBP/WAT/PASS/
loss/manifest publication.
