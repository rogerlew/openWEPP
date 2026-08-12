# Contract Amendment Evidence

Status: `reviewed / promotion active`

Evidence mode: `Static + Ran`

- `SC-VEGETATION-001@6` now binds V2 occupancy state, conditional area,
  same-tile routing, local nonlinear consumers, water identity, warm starts,
  migration, local/stand closure, typed failures, invariants 073--079, tests,
  unit map, gaps, and change history.
- New shared `SC-VEGETATIONTRANSACTION-001@1` binds the exact occupancy water
  key, stand-area amount basis, proportional same-layer arbitration, one-time
  area conversions, finalized-use-only debit, energy operand identity,
  independent receiving-owner reconstruction, duplicate/swap rejection, and
  all-owner atomicity. `SC-VEGETATION-001@6` consumes it through
  `REF-VEGETATION-030` and `INV-VEGETATION-077`.
- Existing `SC-LANDSURFACEENERGY-001@2` and `SC-WATBAL-001@170` remain
  byte-identical because they are draft/in-review contracts. The canonical
  shared transaction contract supplies the bounded cross-domain identity both
  owner candidates must consume.
- After both independent science roles passed the corrected content, the
  contract index promoted both amended contracts to `approved/active`. This is
  implementation authority only; no runtime activation or cutover is claimed.
- V1 definition SHA-256 remains unchanged.

The new rule is an `OPENWEPP_CANONICAL_SELECTION` over the already admitted
CLM5 E04 constitutive operator; no new empirical constant or proxy equation was
introduced.
