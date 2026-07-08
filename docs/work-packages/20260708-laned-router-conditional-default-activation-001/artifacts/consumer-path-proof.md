# Consumer Path Proof

Status: `COMPLETE`
Evidence mode: Static + Ran.

## Static Path

1. `DirectProductionSeedAuthority` constructs per-scheduled-lane
   `DirectProductionLaneDayInputAuthority`.
2. The native management projection populates `lane.ofe_routing:
   Option<DirectProductionOfeRoutingCoefficientAuthority>`.
3. Rev 46 default eligibility reads that exact scheduled-lane authority:
   `DirectProductionDayInputBuilder::laned_active_default_eligibility()`.
4. When eligibility is complete, `execute_direct_publication_stream()` attaches
   `frame.laned_active = Some(Box::new(day_input_builder.laned_active_config()?))`.
5. `laned_active_config()` calls `laned_geometry_with_selector(...)`, which
   consumes `lane.ofe_routing` and fails closed if any scheduled lane is
   missing native routing authority.
6. The active executor then owns the day loop and emits the `laned_active`
   manifest summary.

## Default-Active Runtime Proof

The all-coefficients default/no-env H2637 run emitted the active block with:

- `days_seen = 731`
- `days_routed = 610`
- `mesh_policy.target_dx_m = 5.0`
- `max_supply_reconstruction_rel = 7.31201193525081e-16`
- `max_day_cascade_residual_rel = 2.2762831518726353e-13`
- `max_day_seam_residual_rel = 5.0415846159888125e-14`
- `max_day_identity_residual_rel = 2.1906143827108124e-13`

The same run byte-matched explicit active for `H2637.hbp` and
`H2637.pass.parquet`.

## No-Fallback / No-Double-Feed Proof

Malformed schedule-crop authority fails during typed seed construction. Mixed
lane authority fails before active config or streaming with the
`laned_active_default_eligibility` guard. There is no per-lane fallback branch
inside active execution.

When active ownership is selected, this package does not modify the rev-27
active owner: DC01 surface runon admission remains disabled by the existing
executor path, the active closure hard-fail remains live, and Wave-1 consumes
the routed hydrograph shape through the existing D13 active producer.

When no scheduled lane has coefficients, active config is not attached and the
manifest contains no `laned_active` block.
