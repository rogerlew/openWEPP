# ow-lanuse Canonical Consensus Addendum

Status: post-closure consensus recorded.
Evidence class: Static operator/Codex consensus.

## Consensus

Do not add another coefficient sidecar to solve the legacy cropland coefficient
assignment problem. Sidecars are operator-error prone: forgetting or mismatching
one can make the same legacy management file run with different physics.

The preferred direction is to make `ow-lanuse-1` the canonical production input
surface for new openWEPP physics.

## Production Boundary

- `ow-lanuse-1` is the production-authoritative datver for native landuse
  physics and Lane D `routing_coefficients`.
- WEPPpy owns producer-side resolution from Disturbed/native classes to explicit
  route coefficients.
- WEPPpy writes the five Lane D route coefficients into the native `.man`.
- openWEPP consumes explicit native operands from the management file.
- Legacy datvers remain compatibility inputs for legacy single/MOFE drivers,
  validation, rollback, and regression diagnosis.

## Non-Goals

- No runfile disturbed-class sidecar for coefficient authority.
- No optional routing-coefficient sidecar whose absence changes physics.
- No implicit `lanuse=1` legacy cropland default that silently becomes Lane D
  coefficient-complete.
- No projection from `rrc`, `rrough`, row/rill geometry, cover, residue,
  aggregate friction, erosion delivery, or diagnostics.

## Follow-On Package Shape

The next authority package should be an `ow-lanuse-1`
canonicalization/migration package, not a legacy-field projection package. It
should:

- ratify `ow-lanuse-1` as canonical for new production physics;
- define legacy datver retention as compatibility/validation/rollback only;
- require WEPPpy producer migration to native files with embedded
  `routing_coefficients`;
- add openWEPP eligibility guards that keep legacy datvers legacy/off and fail
  mixed native coefficient authority closed;
- update end-user docs to make native datver production requirements explicit.
