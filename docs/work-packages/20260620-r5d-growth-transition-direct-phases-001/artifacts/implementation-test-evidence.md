# Implementation And Test Evidence

## Implemented

- Added `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`.
- Added direct runtime constants and exports:
  - `DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT`
  - `DIRECT_R5D_ANNUAL_GROWTH_SPAN`
  - `DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT`
  - `DIRECT_R5D_PERENNIAL_GROWTH_SPAN`
- Added typed direct runtime growth surfaces:
  - `DirectGrowthActiveContext`
  - `DirectGrowthAction`
  - `DirectGrowthStateSurface`
  - `DirectGrowthInputs`
  - `DirectGrowthState`
  - `DirectGrowthDownstreamOperands`
  - `DirectGrowthShadowProjection`
  - `DirectGrowthSpanReport`
- Extended `DirectDayFrame` with separate annual and perennial growth input,
  state, downstream operand, and shadow projection fields.
- Wired direct executor order:
  - R5B normalization
  - R5B storage bounds
  - R5C decomposition
  - R5C residue partition
  - R5D annual growth
  - R5D perennial growth
  - R4 hydrology tail
- Changed phase lifecycle status so all 14 direct phases now report
  `Executed`.
- Added `growth_context_required` to R4N direct ET inputs, default-disabled for
  isolated R4N tests and enabled by R5D direct projection.

## Focused Test Coverage

- Annual growth computes, mutates state, emits downstream operands, shadow
  projects, and seeds R4N LAI/canopy/root-depth context.
- Perennial growth executes after annual phase identity and supports grazing
  action.
- Annual winter/rotation-boundary `gddmax=0` resolves through monthly climate
  sentinel integration.
- Missing upstream, missing active context, ambiguous context, annual grazing,
  nonfinite plant state, and perennial-before-annual phase order fail closed.
- R4N required growth context fails closed when absent and passes after R5D
  inactive projections.

