# Contract Implementation Evidence

Status: complete

Evidence mode: Static

## SC-SNOWFREEZE-001 Amendment

Updated `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
on 2026-06-11:

- Added v54 FDHP01 revision-history entry for the attempted heat-flow
  implementation, v55 post-review amendment reopening
  `GAP-SNOWFREEZE-002` after cohort validation failed, v56 layered-store
  amendment rejecting scalar `frdp * theta` frozen-water stores, and v57 D3
  amendment requiring freeze/thaw energy to move the same fine-layer state that
  `frwatc` publishes. Increment A added v58, correcting `frwatc(1)` from
  ambiguous hourly entry to active-day hour-1 ingress and authorizing the
  behavior-preserving fine-state shadow aliases.
- Strengthened `INV-SNOWFREEZE-006` so executable frost-depth progression must
  derive from hourly signed heat flow and latent-heat increments, be bounded by
  physical soil profile depth, and must not use the retired
  `0.20 m * clamp(mean-temperature / 6 degC)` proxy.
- Added alias authority for runtime `frost.runtime_frdp_m` and WAT
  `hillslope_wat.frdp`.
- Added the FDHP01 frost depth heat-flow addendum.
- Re-scoped `GAP-SNOWFREEZE-002` to keep D3 active after the 2026-06-11
  layered-store cohort cleared D2 annual closure and `p2` but still failed
  depth/duration validation.
- Re-stated `GAP-SNOWFREEZE-002` after the D3 coarse-front attempt: a coarse
  scalar/per-layer front is insufficient; the remaining implementation must
  port the legacy fine-sublayer `frostn`/`frzng`/`mltbtm`/`frwatc` coupling.
- Updated `SC-WATBAL-001` on 2026-06-11 to clarify that additive WAT parquet
  extensions beyond canonical WB13 replay columns must be versioned. FDHP01
  assigns WAT dataset version `1.4` to the required `frdp` column.
- Added `SC-WATBAL-001` v150/v151/v152 amendments: v150 pins the legacy
  `Total-Soil + frozwt` storage term from `frwatc.for`/`watbalprint.for`;
  v151 binds WAT `frozwt` publication to
  `frost.runtime_frwatc_frozen_water_after_m` and rejects depth-derived
  publication; v152 binds that diagnostic and WAT `frozwt` to the layered
  legacy `Σ soilf(i)` store.
- Added `SC-SNOWFREEZE-001` v69 after Dk certification: `GAP-SNOWFREEZE-002`
  is closed/re-stated, the residue pre-check is recorded as clean for the
  frost projection seam, and MOFE is no longer blocked by FDHP01.

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
  `frost.runtime_frwatc_frozen_water_after_m`, which now resolves from
  per-layer frozen-depth/frozen-water state rather than scalar `frdp * theta`.
- Frozen-water storage exchange is locally bidirectional in the runtime state,
  and cohort validation shows annual additive closure is restored. The
  implementation is certified complete at the Dk single-OFE boundary.
- A D3 coarse-front production/test experiment was built and validated locally
  but did not satisfy the package phase boundary; those production/test edits
  were backed out. Later staged increments superseded that hold, through
  `SC-SNOWFREEZE-001` v69.
