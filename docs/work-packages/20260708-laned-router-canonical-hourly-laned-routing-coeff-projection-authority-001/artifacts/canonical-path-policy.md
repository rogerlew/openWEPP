# Canonical Path Policy

Status: queued placeholder.

Proposed policy to adjudicate:

- Hourly water balance plus Lane D active routing is the canonical production
  water/sediment path for single-OFE and MOFE.
- Non-hourly water balance, DC01-only surface routing, and non-Lane-D MOFE stay
  in the codebase for legacy validation, comparator work, protected rollback,
  and regression diagnosis.
- No new production consumers may be added to the retained legacy/reference
  paths after this policy is ratified.
- Explicit disable/rollback selectors remain protected and byte-identity tested
  where already required.
