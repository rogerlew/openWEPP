# Implementation Evidence

Status: `EXECUTED`

Implementation files:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`

Changes:

- Added `depth_pow_3_2(h) = h * sqrt(h)`.
- Replaced perturbed-depth celerity with one rev-47 local numerics call per
  cell per step.
- Added local friction derivatives for Manning, skin, form, wave, and
  vegetation.
- Added closed-form Manning and pure-skin hydraulics.
- Added explicit pure-skin discontinuity-gap selection from the pre-step
  Reynolds branch.
- Added bounded log-Newton for additive menus, with finite denominator,
  residual, and celerity guards.
- Made active vegetation derivative failures return
  `RoutingError::NonFiniteState` instead of collapsing active non-finite math to
  the absent-canopy zero branch.
- Kept typed `RoutingError::NonFiniteState` / `CflViolation` fail-closed
  behavior.
- Updated the existing cascade vegetation regression margin for rev-47 true
  celerity while retaining the monotone storage signature.
- Did not touch mesh policy, active selection policy, sediment, watershed, or
  wepppy concerns.

No `Re^0.45` approximation was implemented.
