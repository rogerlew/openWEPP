# Full-Support V10 Compatibility Ledger

Status: exhaustive projection inventory frozen; values pending implementation

The implementation generator recursively walks both serialized object graphs.
Every map key, sequence index, enum discriminant and scalar leaf must land in
exactly one path rule below. An unmatched, multiply matched, missing, or extra
leaf fails. Arrays compare in order; maps use canonical key order.

## Permitted successor-identity projection

Only root model/config/state digests; parent/segment/slab/event identities; and
successor receipt/schema discriminators may differ. Removing those leaves must
make the projected graphs exactly equal.

## Exhaustive nonidentity projection roots

- `configuration.{area_m2,timestamp,topology_tiles[*],strata[*]}` recursively,
  excluding V10 `dt_s` and V11 `nominal_cadence_ns` after exact migration;
- `state.strata[*]` recursively: every C/N tissue pool, NSC, XS,
  retranslocation, T10, phase, timer, GSI, area, occupancy, canopy liquid,
  warm start, pending transfer and diagnostic;
- water request, authorization and final-use arrays recursively, including
  source layer/OFE/tile/occupancy, amount bits, reason and owner endings;
- radiation, gas, interception, vapor, hydraulics, energy, carbon, nitrogen,
  turnover and mortality results recursively, including operands, caps,
  selected branches, convergence counts and closure residuals;
- material proposals recursively: ordered source/sink, C, N, dry matter and
  closure ledger;
- complete beginning/ending vegetation, water, NH4, NO3, BGC, energy and
  thermal owner candidates recursively;
- diagnostics/reductions recursively, using accepted slabs only.

The generated ledger records V10 path, V11 path, classification, type, exact
canonical value, equality result, source file and field declaration for every
leaf. A test adds a synthetic unknown leaf and proves fail-closed behavior. A
second mutates one leaf in every projection root and proves detection.
