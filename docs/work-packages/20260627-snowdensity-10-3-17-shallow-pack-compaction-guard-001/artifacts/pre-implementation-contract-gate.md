# Pre-Implementation Contract Gate

Status: complete
Evidence mode: Static

Static:

- Canonical contract authority was amended before production code edits.
- Contract-derived tests are authorized for:
  - selector isolation and fail-closed behavior;
  - opt-in `physics_bulk_shallow_guard_v1`;
  - authority-derived `0.25 m` shallow threshold;
  - SWE identity, density-cap preservation, and melt/liquid invariance;
  - coupled WAT/trace gate evidence.
- Production code edits may proceed; the contract and contract-derived test file
  are in place.
