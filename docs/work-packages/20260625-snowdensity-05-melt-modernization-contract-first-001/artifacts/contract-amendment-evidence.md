# Contract Amendment Evidence

Status: complete.
Evidence mode: Static.

`SC-SNOWFREEZE-001` was amended from v75 to v76 on 2026-06-26.

Added:

- `INV-SNOWFREEZE-052`
- `OBL-SNOWFREEZE-P-027`
- `SNOWDENSITY-05A CoE Melt Modernization Contract Addendum`
- `snow_melt_model = legacy_coe | coe_shortwave_albedo_v1`
- placeholder operands for shortwave source/provenance and albedo state
- signed `melt_bmelt_in` convention:
  `hrmelt_raw = 0.0254 * (amelt + melt_bmelt_in + cmelt + dmelt)`
- invalid-state and boundary-disposition rules for opt-in melt activation,
  radiation retuning, degree-day promotion, and `bmelt` sign flipping
- v76 revision-history row

Not added:

- no production formula implementation
- no albedo constants
- no radiation source selection
- no parser/output schema changes
- no default activation
