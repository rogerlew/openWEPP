# No-Compatibility Proof Checklist

Status: executed-hold.
Evidence mode: Static + Ran.

## Forbidden Accepted Direct-Publication Reads

- `SimulationOwnedWb13Row`
- `HillslopeWritebackSurface`
- `KernelWritebackPayload`
- `BoundarySymbol`
- `BoundaryValue`
- `SymbolRegistry`
- hot tables
- indexed compatibility surfaces
- dense refresh or dirty flush state
- stale logical output frames
- diagnostic compatibility ledgers as publication authority

## Required Proof

- Source scans for direct consumers and manifest cutover path.
- Runtime counters proving default compatibility does not build direct
  publication artifacts.
- Runtime counters or focused tests proving accepted cutover publication does
  not invoke compatibility publication readers.

## Gate

BLOCKED. No accepted direct-publication consumer path exists.

Static:

- Direct helper consumers read `DirectRunPublicationFrame`, not WB13 rows, but
  the frame is skeleton-populated and therefore not accepted.
- Cutover parity still compares against compatibility HBP/loss/WAT/PASS
  baselines.
- Production public writes remain guarded off while gates fail.
- Production manifest writing still uses compatibility provenance.

Ran: the CLI cutover failed before output files were written.
