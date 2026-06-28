# Local Review

Evidence mode: `Static`

## Scope Reviewed

- `openwepp_meteorology::surface_energy` public API and typed local wrappers.
- Error handling and finite/domain validation.
- Unit tests for flux reference values, closure, latent<->mass coupling,
  turbulent stability, and domain errors.
- Integration guard proving Stage 0 remains pure and unwired.
- Clean-room provenance and CC0 evidence.

## Findings

No unresolved findings remain.

Accepted fixes during verification:

- Updated the turbulent zero-gradient expectation to reflect libsnobal's
  dry-adiabatic potential-temperature correction at measurement height.
- Replaced hand midpoint expressions in the Monin-Obukhov psi functions with
  `f64::midpoint` to satisfy workspace clippy.
- Made package/provenance boundary strings explicit so the Stage 0 guard checks
  stable evidence markers.

## Boundary Check

- No production snow/frost/runtime source calls the new `surface_energy`
  module.
- No selector, default, parser/runfile/user control, fixture, output schema,
  density cap, or frost behavior changed.
- Existing `openwepp-meteorology` dependencies in runtime crates remain from the
  Harder-Pomeroy phase work; this package adds no new runtime use.
