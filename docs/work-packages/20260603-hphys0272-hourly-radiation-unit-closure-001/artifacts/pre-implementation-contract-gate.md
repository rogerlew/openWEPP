# Pre-Implementation Contract Gate

Status: completed
Evidence mode: ran

Static:

- Contract amendments and contract-derived tests were authored before
  production code edits.

Ran:

- Command:
  `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion --lib -- --nocapture`
- Result: failed as expected before the production correction.
- Failure excerpt: `hourly radiation must be MJ-scale; max=30.504601472037276,
  daily_radmj=8.368`.
- Interpretation: the red gate proves the existing seam emits Langley-scale
  radiation under `winter.hourly.rad_mj_m2_####`.
