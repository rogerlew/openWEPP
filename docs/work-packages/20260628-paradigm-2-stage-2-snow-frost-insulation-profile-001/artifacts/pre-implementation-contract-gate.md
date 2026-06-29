# Pre-Implementation Contract Gate

Status: `PASS`

Evidence class: `Static`

The contract-first amendment was completed before production frost-coupling
edits.

## Contract Amendment

- `SC-SNOWFREEZE-001` header bumped from v108 to v109.
- Added `REF-SNOWFREEZE-PARADIGM2-STAGE2`.
- Added variables:
  - `snow_frost_insulation_model`;
  - `snow_layer_insulation_resistance`;
  - `snow_frost_effective_density`.
- Added `INV-SNOWFREEZE-079`.
- Added `OBL-SNOWFREEZE-P-054`.
- Added a Boundary Disposition row for the Stage 2 candidate.
- Added the Paradigm 2 Stage 2 addendum and v109 revision-history row.

## Design Binding

The amendment chooses the rollback-compatible equivalent-density route: compute
the prior-day layer-stack snow thermal resistance with the Sturm et al. 1997
snow-density-to-conductivity relation already mirrored by the WEPP frost heat
path, then pass an insulation-equivalent density through the existing
`DirectFrostThermalInputs` bulk fields.

This keeps the public snow density output unchanged and preserves the existing
bulk handoff as the absent-selector rollback.
