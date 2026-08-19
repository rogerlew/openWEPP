# HOLD Legitimacy Audit

Historical evidence for commit `2465849c9`; the HOLD was subsequently lifted
by explicit non-defaulted stratum geometry and constitutive authority.

Evidence class: `Static`

## Named boundary

The package cannot release an executable owner because two required material
operands lack authority:

1. distinct root-tissue path `z3` for every required occupancy/OFE/layer;
2. current unsaturated soil conductivity for the root interface.

## Exact evidence

- `crates/openwepp-vegetation/src/config.rs:25` defines `RootLayer` with only
  identity, root fractions and `lateral_root_length_m`.
- `crates/openwepp-vegetation/src/occupancy_solver/evaluator.rs:798` passes that
  field as `dxroot_m`, while line 808 obtains `z3_m` separately from caller
  `SoilLayerForcing.root_path_length_mm`.
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_input_projection.rs:650`
  likewise obtains `z3_m` from caller forcing and line 651 obtains `dxroot_m`
  from immutable vegetation configuration.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs:742`
  copies `conductivity_m_s` from immutable inputs; lines 1332--1341 apply a
  separate moisture factor, and lines 1647--1649 call the aggregate saturated
  conductivity.
- CTSM Plant Hydraulics equation 2.11.18 and its variable definitions separate
  root-tissue path `z3` from interface distance `dxroot` and require
  moisture-dependent soil conductivity.

## In-envelope correction routes considered

- Alias `lateral_root_length_m` to `z3`: rejected because it is already
  `dxroot` and would collapse two series resistances.
- Derive `z3` from node depth, layer thickness or plant height: rejected because
  V1 explicitly requires a configured final path and no admitted geometry
  builder exists.
- Use fixture/cache values: rejected as caller-controlled physics and absent
  data authority.
- Use WB14 suction or effective conductivity: rejected because those belong to
  Green-Ampt infiltration and have different equations/ownership.
- Convert `DirectSubsurfaceLayerState.conductivity_m_s` directly: rejected
  because the field has saturated/base `ksat` lineage.
- Admit the CLM/Clapp-Hornberger unsaturated relation immediately: incomplete
  without authoritative saturated conductivity/retention parameter mapping,
  deterministic power semantics, vectors, and real path values.

## Why the package cannot close in-envelope

Adding a Rust field or JSON number would define storage, not data authority.
No existing repository input supplies non-invented values for the declared
rooted scenarios. The execution prompt explicitly permits HOLD when this exact
root-path supply contradiction or conductivity constitutive gap is proven.
Implementation would otherwise encode proxy physics, forbidden by repository
governance.

## First lift action

Admit and populate a real per-occupancy/per-layer root-tissue path input and the
unsaturated-conductivity parameter/equation boundary, then resume the contract
profile, independent vectors, reviews and implementation.
