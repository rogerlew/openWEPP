# Pre-Implementation Contract Gate

Status: `complete`

Evidence mode: `Static`

Production edits are prohibited until canonical amendments and
contract-derived tests bind the provider, selectors, equations, guards,
signs, cadence, state mutation, and closure identities.

Static: `SC-SNOWENERGY-001` v2 and `SC-SNOWFREEZE-001` v118 were amended
before the production implementation. They bind the Stage 3 top layer as the
sole snow-surface thermal provider, independent default-off `L` and `S`
selectors, `T_c = T_a`, `R_a,min = 1e-9 MJ m^-2 d^-1`, typed polar-night
unavailability, exact-one vapor/latent composition, snow-state mutation, and
independent mass/energy closure. Contract-derived tests were authored in
`snow_surface_eb03_contract.rs` before the implementation gate was treated as
open.
