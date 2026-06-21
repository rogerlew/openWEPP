# Discovery

## Static Authority
- R5D is the next uncompleted row in `docs/work-packages/r5-burndown-execplan.md`.
- `SC-PLANT-001` defines the growth-transition authority for active crop slot resolution, reset actions, direct plant-growth equations, root-depth/state mutation, and fail-closed domain handling.
- R4N direct evapotranspiration/root uptake already has typed inputs for LAI, canopy cover, root depth, plant tolerance, and transpiration demand; R5D must make those fields consume growth shadow context when required.

## Existing Direct Runtime Shape
- R5A, R5B, and R5C direct spans live under `crates/openwepp-hillslope-orchestrator/src/direct_runtime/`.
- `DirectDayFrame` owns typed inputs/state/downstream/shadow fields for each direct phase.
- `DirectFrameExecutor::run_day_spans` is the phase-order authority for the direct runtime skeleton.
- `PhaseLifecycleStatus` still reports annual/perennial growth transitions as pending before R5D.

## Initial Risk
- Growth transitions are wider than the previous R5 phases because they bridge plant state, management schedule semantics, and R4N hydrology demand inputs.
- Public output identity must remain protected because R5D is still a shadow-only direct runtime slice.

