# HPHYS0236 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: mixed (`Static` + `Ran`)

## Contract-Derived Test Updates

Static:
1. Replaced hourly-lane regression test in
   `tests/integration/wb18_percolation_physics_kernel_contract.rs`:
   - old: `wb18_contract_conformance_hourly_lane_substeps_attenuate_per_layer_flux`
   - new: `wb18_contract_conformance_hourly_lane_substeps_execute_iterative_recompute`
2. Added independent projection helper
   `project_hourly_iterative_percolation_surface(...)` that computes expected
   `24`-substep iterative WB18 results from the seeded state surface.
3. Added explicit anti-regression assertion that hourly output must **not**
   collapse to divisor-only single-pass behavior (`pei_daily / 24`).

## Executed Evidence

Ran:
- `cargo test --workspace`
- Result: pass, including
  `tests/integration/wb18_percolation_physics_kernel_contract.rs` (`10/10`
  tests passed).

## Outcome

Contract-derived executable coverage now guards both:
1. conformance to iterative hourly recompute behavior, and
2. hard-fail prevention of divisor-only single-pass regression.
