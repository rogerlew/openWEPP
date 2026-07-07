# Implementation

Status: EXECUTED

Static:

- `DirectGrowthStateSurface` now carries daily `canopy_height_m`.
- `DirectGrowthInputs` now carries crop parameters `bbb` and `hmax`, projected
  from typed growth crop authority.
- PL growth computes daily `canopy_height_m` from the baseline equation
  `(1 - exp(-bbb * vdmt_next)) * hmax` and validates finite nonnegative values.
- The runner applies the growth surface into
  `DirectProductionEvapotranspirationAuthority`, then publishes the same value
  through `DirectDayFrame.evapotranspiration_compute_inputs.canopy_height_m`.
- Lane D active and shadow operand builders consume that post-growth day-frame
  canopy height when pairing `h_c` with post-growth LAI.
- The active route guard remains fail-closed when LAI is positive and
  post-growth `h_c` is missing, non-finite, or non-positive.
- Existing typed-management `canhgt` remains an initial seed/source projection;
  it no longer substitutes for same-day post-growth `h_c` in Lane D routing
  operands.

Ran:

- Added/updated focused tests for growth-state publication, R5D post-growth
  canopy-height computation, Lane D active dynamic operand sourcing, and runner
  source guards.
- Added an active-route regression vector proving post-growth day-frame
  `canopy_height_m` is consumed instead of static lane configuration height.
- Reverted an attempted Wave-1 erosion consumer change after
  `erosion_single_ofe_p61_sediment` showed a material p61 sediment change. The
  final implementation leaves that consumer unchanged.
