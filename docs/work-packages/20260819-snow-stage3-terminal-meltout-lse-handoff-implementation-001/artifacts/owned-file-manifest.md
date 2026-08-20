# Owned file manifest

Status: frozen before production edits

Evidence mode: Static

Implementation intent: fixed-authority mechanical default-off integration.

Contract authority:

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`
- `docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md`
- `SC-WATBAL-001.md` and `SC-RUNOFFPART-001.md` only if the new typed parcel
  needs an explicit binding beyond their existing meltwater chronology.
- `docs/specifications/science-contracts/index.md`

Production Rust candidate:

- Stage 3 persistent/result surfaces in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  and `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/**`.
- A bounded new terminal-handoff module plus exports under
  `crates/openwepp-hillslope-orchestrator/src/`; existing
  `v9_real_consumer_shadow.rs` only for the smallest required visibility and
  owner hooks.
- Partial-duration receiving support in
  `land_surface_energy_shadow/**` and `direct_runtime/surface_liquid_ingress.rs`
  only where the real consumer's existing exact-duration guards require it.
- `direct_runtime/03_executor.rs` for the outer Stage 3 + production + V10
  candidate transaction if runner-only composition cannot prove the actual
  scheduler path.
- Runner day-input/owner modules
  `00a_snow_frost_authority_impl.rs`, `00c_day_input_builder_impl.rs`, and
  `00_builders_and_authority.rs` only to replace external precommit state
  mutation with the owned transaction and internal evidence path.

Tests:

- a new package contract test under `tests/integration/`;
- existing terminal-event, V10 real-consumer/restart, land-surface real-
  hydrology, surface-liquid ingress, and runner direct-publication test modules;
- new focused unit/integration modules adjacent to the bounded handoff owner.

Package evidence, coordinator lifecycle, catalog, and roadmap remain owned.
No public WAT/HBP/PASS schema, production selector/default, fixture cohort,
external-authority suite posture, or observation is owned.
