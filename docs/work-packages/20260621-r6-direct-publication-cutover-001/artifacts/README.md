# R6 Artifacts

Artifacts for R6 direct publication cutover.

Current disposition:
`HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER`.

R6 resumed after R5E completion, promoted the PERFDEEP06 publication operand
ledger into canonical architecture authority, consumed the R6A
`DirectRunPublicationFrame`, and added a guarded
`DirectPublicationFrameCutover` candidate path.

The candidate is intentionally fail-closed. It builds direct publication
artifacts and routes the output boundary through them only after parity gates
pass. Current evidence shows the first gate fails:
`R6-DIRECT-PUBLICATION-PARITY HBP byte identity failed: direct=1654 bytes
compatibility=1654 bytes`.

R6 is therefore not complete. The next work must populate the direct frame from
parity-grade typed direct run operands and cut the production manifest writer
over to direct publication provenance before public output cutover can close.
