# Contract Disposition

Status: `SC-OFEROUTE-001` rev 44 applied
Evidence mode: Static.

## Changed Contract

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`

## Amendment

Rev 44 adds annual pass-sediment metric authority for active mesh-policy
evidence. The amendment touches the existing active mesh-policy surfaces:

- Active mesh policy row
- Active mesh-policy selector row
- `INV-OFEROUTE-013`
- Target-`dx` active-mesh-policy tolerance note
- Mesh-policy notes
- `OFEROUTE-ACTIVE-MESH-POLICY` Binding Exposure Index row
- Revision history

The amendment records that annual pass-sediment comparisons use a
material-year plus annual-vector rule:

- material member-years keep the named relative threshold;
- the whole annual vector is bounded by L1 relative to the reference annual
  vector at the same named threshold;
- low-contribution member-year relative excursions are reported but do not
  independently block promotion if the material-year and vector gates pass.

## Current Scope Legitimacy

The package changes metric authority only. It does not add a new contract
binding ID, remove a binding ID, change required-case bindings, change
external-authority suite posture, or change cohort fixtures.

The BEI checker reports `PASS-DEFERRED` because `SC-OFEROUTE-001` still has
existing `science-review-follow-on` BEI rows. That is the expected current
posture for this package: rev 44 updates the already-existing
`OFEROUTE-ACTIVE-MESH-POLICY` row and does not claim full BEI consolidation.

## Explicit Non-Changes

No production behavior is changed by the contract amendment:

- active production default remains fixed `10 cells/OFE`;
- `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M` remains diagnostic unless a later
  production-promotion package flips the default under contract authority;
- routed water, routed shape, storage, tail-fold, closure, active selector,
  shadow mesh, and sediment process-physics behavior are unchanged.

## Follow-On Authority Surface

The next production mesh-policy package may use rev 44 as authority for annual
pass-sediment evidence. It still must separately prove all other judged
surfaces and default/off protections before any production default change.
