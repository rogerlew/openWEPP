# Worker Handoff

Status: executed-hold.
Evidence mode: Static + Ran.

## Current State

R6 execution resumed after R5E and R6A completion. The publication operand
ledger is canonical in
`docs/architecture/array-native-runtime-specification.md` section `5.2.1`.
R6A supplied `DirectRunPublicationFrame` and direct projection consumers.

Current R6 added a guarded `DirectPublicationFrameCutover` candidate and CLI
flag `--direct-publication-frame-cutover`. The candidate builds direct
publication artifacts and reaches the output boundary, but it fails closed at
the first identity gate:

```text
R6-DIRECT-PUBLICATION-PARITY HBP byte identity failed:
direct=1654 bytes compatibility=1654 bytes
```

The production manifest writer also remains compatibility-provenance based.

## First Actionable Item

Close `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER`:

1. Populate `DirectRunPublicationFrame` from parity-grade typed direct run
   operands instead of skeleton/zero direct state.
2. Add anti-alias fixtures that distinguish HBP `peakro`, `watdur`, sediment,
   WAT water-balance fields, PASS volumes, loss static fields, and manifest
   provenance from compatibility aliases.
3. Add independent reconstruction for accepted HBP/WAT/PASS/loss operands.
4. Replace the manifest production provenance/checksum path with typed direct
   publication projection in cutover mode.
5. Re-run the cutover candidate until HBP, WAT, PASS, loss, and manifest gates
   pass, then run default-disabled and endpoint/RSS benchmarks.

## Blockers

- Current direct publication artifacts fail HBP byte identity.
- WAT/PASS/loss acceptance remains blocked behind HBP parity.
- Current fixture does not cover PASS parquet in the cutover candidate.
- Production manifest publication remains compatibility-provenance based.
