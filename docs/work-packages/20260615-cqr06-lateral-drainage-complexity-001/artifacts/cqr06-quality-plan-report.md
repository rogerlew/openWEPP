# CQR06 Quality Plan Report

Evidence class: Static + Ran

Quality dimension: CRAP / cyclomatic-complexity reduction for
`hydrology_phase_lateral_drainage.rs`.

Closure target: every eligible target-module function has CRAP `<= 30`.

Result:

- Before max CRAP: `300.2455501433063`
- After max CRAP: `26.541362973760947`
- Status: passed

Implementation strategy:

- Kept the target in one Rust module per package scope.
- Introduced private context/result structs for lateral inputs, lane config,
  lateral layer state, drainage geometry, drainage layer slices, and WB14
  ksat-adjustment metrics.
- Split `run_lateral_transfer` into input loading, lane config loading, layer
  state loading, substep execution, diagnostic/writeback construction, and final
  flux construction.
- Split `run_drainage` into input loading, drainage substep loop, geometry
  loading, water-table/depth helpers, drainage potential, and response
  construction.
- Split WB14 ksat-adjustment helpers into layer loading, layer validation,
  aggregation, final metric validation, solwpv dispatch, and conversion helpers.

Warnings:

- The file is `2527` lines after refactor, above the `2000` line-count WARN
  threshold and below the `3000` block threshold.
- Coverage improved but remains below the science-tier `>= 90%` target.
