# Operand Reconstruction

Evidence class: Static/Ran.

Current status: closed `OPT-IN` with frost-depth divergence reclassified.

Required scope:

- Snow/frost-sensitive WAT operands, especially frost water/depth and snow
  water/routed melt surfaces.
- PASS/HBP runoff and erosion-sensitive operands when protected parity fails.
- Manifest checksum and provenance reconstruction from produced direct outputs.

Anti-tautology rule:

- Reconstruction must use produced outputs or independently authoritative typed
  operands. It must not merely restate the producer formula with the same
  internal operands.

Current reconstruction evidence:

- Produced-output checksums prove two stable pairs:
  direct default-candidate equals explicit direct for HBP/WAT/PASS/loss/plot,
  and default compatibility equals explicit rollback for HBP/WAT/PASS/loss/plot.
- Direct-vs-compatibility loss and plot outputs match exactly.
- Direct-vs-compatibility HBP, WAT, and PASS outputs differ.
- DuckDB row-difference reduction proves WAT and PASS are value mismatches, not
  Parquet metadata-only mismatches.
- WAT first material divergence above `1e-9` is Julian day 6 in frost/water
  state fields: direct has lower `frozwt` and `frdp`, with the corresponding
  water retained in `Total-Soil` / `SoilWaterTotal`.
- PASS residuals are `runvol`, `sbrunv`, and `peakro`; sediment class fields are
  clean. These PASS deltas are downstream hydrology consequences of the frost
  split, not independent sediment publication aliases.

Anti-alias disposition:

- The output evidence rejects a publication-writer alias explanation:
  direct-vs-direct and compatibility-vs-rollback are internally stable, and
  loss/plot match across modes.
- The output evidence rejects a sediment-publication alias explanation:
  PASS sediment fields are clean while hydrology/runoff fields are red.
- The remaining unclosed alias risk is inside typed direct frost state:
  compatibility and direct can have matching day-5 coarse WAT frost fields while
  retaining a fine/internal frost state difference that under-freezes on day 6.
  That requires the follow-up package to compare typed direct fine-layer carry
  against request-backed compatibility fine-layer writeback for the day-5 to
  day-6 transition.

Closure:

- Independent reconstruction is not marked green for default activation. The
  direct-vs-compatibility frost split is acknowledged as a contract-tracked
  `GAP-SNOWFREEZE-002` delta, and R7H remains opt-in.
- The next reconstruction target is frost-depth fidelity against observation
  fixtures, not compatibility frost output identity.
