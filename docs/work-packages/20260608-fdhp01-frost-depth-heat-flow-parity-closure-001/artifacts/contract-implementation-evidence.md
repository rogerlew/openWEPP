# Contract Implementation Evidence

Status: executed-hold

Evidence mode: Static

## SC-SNOWFREEZE-001 Amendment

Updated `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
on 2026-06-11:

- Added v54 FDHP01 revision-history entry for the attempted heat-flow
  implementation, then v55 post-review amendment reopening
  `GAP-SNOWFREEZE-002` after cohort validation failed.
- Strengthened `INV-SNOWFREEZE-006` so executable frost-depth progression must
  derive from hourly signed heat flow and latent-heat increments, be bounded by
  physical soil profile depth, and must not use the retired
  `0.20 m * clamp(mean-temperature / 6 degC)` proxy.
- Added alias authority for runtime `frost.runtime_frdp_m` and WAT
  `hillslope_wat.frdp`.
- Added the FDHP01 frost depth heat-flow addendum.
- Re-scoped `GAP-SNOWFREEZE-002` back to active defect after the 2026-06-11
  cohort run failed `p2`, annual closure, and depth-envelope validation.
- Updated `SC-WATBAL-001` on 2026-06-11 to clarify that additive WAT parquet
  extensions beyond canonical WB13 replay columns must be versioned. FDHP01
  assigns WAT dataset version `1.4` to the required `frdp` column.
- Added `SC-WATBAL-001` v150/v151 amendments: v150 pins the legacy
  `Total-Soil + frozwt` storage term from `frwatc.for`/`watbalprint.for`;
  v151 binds WAT `frozwt` publication to
  `frost.runtime_frwatc_frozen_water_after_m` and rejects depth-derived
  publication.

## Boundary

The amendment preserves the existing CLIM06 frozen-soil kfactor rule by
retaining `0.20 m` only as the tilled-layer conductivity depth scale. It no
longer authorizes `0.20 m` as a model-depth cap.

## Implementation Binding

Static:

- `coupling.rs` implements the FDHP01 heat-flow depth progression and publishes
  finite hourly `Qsrf`, `Quf`, and `Ksrf` diagnostics.
- `FrostCouplingOutcome` carries physical profile depth so writeback and
  publication use the same bound.
- WAT `frdp` publication is required, unit-registered, versioned as dataset
  `1.4`, and profile-bound.
- WAT `frozwt` publication requires
  `frost.runtime_frwatc_frozen_water_after_m`. Cohort evidence shows this is
  not yet sufficient for closure because that diagnostic aliases the
  depth-derived store.
- Frozen-water storage exchange is locally bidirectional in the runtime state,
  but cohort validation shows annual closure is not conserved; the
  implementation remains held pending FDHP01 correction.
