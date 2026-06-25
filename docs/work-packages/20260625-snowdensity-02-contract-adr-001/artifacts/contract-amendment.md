# Contract Amendment

Static:

- `SC-SNOWFREEZE-001` header bumped from `contract_version: 74` to
  `contract_version: 75`.
- Added candidate authority anchors:
  - `REF-SNOWFREEZE-SNOWDENSITY01`
  - `REF-SNOWFREEZE-ANDERSON1976-CANDIDATE`
  - `REF-SNOWFREEZE-SNOBAL-CANDIDATE`
- Added candidate variables:
  - `snow_model`
  - `snow_cold_content`
  - `snow_liquid_water`
  - `snow_bulk_temperature`
  - `snow_cover_age`
- Added `INV-SNOWFREEZE-051`, binding
  `snow_model = legacy_wepp | physics_bulk`, `legacy_wepp` default status,
  opt-in `physics_bulk` status, candidate state/process envelope, no-site-
  tuning, `ssd` non-promotion, and diagnostic-only roles for PySnobal/legacy.
- Added invalid-state language blocking default activation, production coupling,
  and per-site fitted constants before promotion gates.
- Added `OBL-SNOWFREEZE-P-026`, requiring typed state, independent mass/energy
  closure, no-site-tuning proof, v74/v75 rubric profiles, and rollback.
- Added the Snow-Density Physics-Bulk Candidate Envelope Addendum.
- Added revision-history v75 row.

Disposition:

- Complete. This amendment creates governance authority for the candidate lane
  only. It does not ratify production equations/constants and does not change
  runtime behavior.
