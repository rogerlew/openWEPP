# Implementation

Status: EXECUTED
Evidence mode: Static.

## Contract Amendment

`SC-OFEROUTE-001` is amended to rev 41:

- Algorithm item 5 now binds conservative stage-face limiting and final TVD
  scaling as the positivity-preserving solver behavior.
- The KWE/TVD Branch and Guard row records that material negative depths are
  prevented without adding external clamp mass.
- Test-vector obligations and BEI rows name the constructed over-drain vector
  and WA fixed10/dx5 active evidence.
- Rev 40's active clamp-source publication guard remains live.

## Code Change

`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
changes:

- adds a reusable `face_flux` scratch buffer,
- adds `limit_stage_face_fluxes`,
- rewrites predictor/corrector stage updates through conservative face fluxes,
- books actual outflow from the limited predictor/corrector boundary faces,
- scales the final TVD face correction when needed to preserve non-negative
  final depths, and
- adds constructed regressions for both stage-face limiting and final TVD
  scaling.

No active executor, source producer, coefficient, route configuration, mesh
policy, or downstream erosion consumer was changed.

## Line-Count Governance

Checked:

- `kinematic_wave.rs`: 1771 lines
- `laned_active.rs`: 1288 lines
- `03_executor.rs`: 1257 lines

No touched Rust file crosses the 2000-line warning threshold.
