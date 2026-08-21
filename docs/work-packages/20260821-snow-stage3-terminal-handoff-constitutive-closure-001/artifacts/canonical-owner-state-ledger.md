# Canonical owner state ledger

Status: `PARTIAL TYPED PROJECTION / OWNER CUSTODY BLOCKED`.

`Static:` Stage-3 bytes use
`Wb11HydrologyKernel::serialize_stage3_persistent_state`. The real consumer
exposes typed canonical projections for vegetation, LSE, surface liquid,
hydrology, BGC, and soil thermal through
`DirectV10RealConsumerShadow::canonical_owner_state_bytes`; no `Debug` string
is used. The coupled clock uses typed `OwnerState` values.

`Static:` The new receipt records owner bytes after its current parent loop,
but the loop cannot reach a snow-covered parent and the ledger currently uses
a structural complete-owner digest as a provisional/final slab ledger. It is
not evidence of seven actual ending owners for the covered path. No atomic
installation of a complete terminal candidate is claimed.
