# Owned-file manifest

Initial declared write set:

- `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- affected files under `crates/openwepp-hillslope-orchestrator/`
- affected files under `crates/openwepp-vegetation/`
- affected files under `crates/openwepp-runner/`
- package-owned tests discovered during contract-first implementation

Exact source edits admitted in this increment:

- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs`:
  sealed provider-day binding, exact-one Stage-3 snow owner, covered-support
  routing, and per-support Stage-3/covered-V11/carrier forcing projections.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/09_snow_free_half_hour_forcing.rs`:
  read-only provider cursor accessors and GSI receipt digest projection.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/snow_stage3_v11_scheduler.rs`:
  runner-facing provider-day installation seam.
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`:
  shared V11 resource/owner finalization, the unchanged snow-free guard, and
  the distinct typed `DirectV11SnowCoveredRealConsumerStack` with its
  `DirectV11SnowCoveredSegmentInput`, owner-derived carrier operands,
  per-lane carrier receipts, and shared-carrier/Stage-3 persistent-support
  boundary.
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/mod.rs` and
  `v11_covered/owner_finalization.rs`: behavior-preserving extraction of the
  covered carrier/iteration/receipt/imported-stack owner and shared final-owner
  lineage helpers; all active files are below the 3,000-line closure threshold.
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs`:
  sealed covered forcing and typed Stage-3 boundary-receipt inputs.
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_open_boundary.rs`:
  destination-specific sealed open exposure/forcing and the pure open-snow
  turbulent, optical, and longwave boundary producer.
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/{v8_input_projection.rs,strict_v8_endpoint.rs,multi_tile_runtime.rs}`:
  Stage-3-owned open-snow pass-through projection, zero-demand transaction
  identity, unchanged LSE/soil state, and zero local LSE energy ownership.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`:
  boundary reconciliation projection.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`:
  exact boundary consumption and Stage-3 result diagnostics.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs`:
  covered-support boundary admission and duration join.
- `crates/openwepp-land-surface-energy/src/solver.rs`:
  explicit `V11SnowCovered` authority identity and the covered lower-boundary
  operator that suppresses snow-free ground water, sensible/vapor, and soil
  storage terms.
- `crates/openwepp-land-surface-energy/src/covered_output.rs` and
  `crates/openwepp-land-surface-energy/src/transaction.rs`: typed lower-boundary
  energy receipt, covered operator validation, and held Stage-3 energy-custody
  seam; released Stage-3 shortwave/advection/soil projections remain open.
- `crates/openwepp-hillslope-orchestrator/src/v8_input_projection.rs`,
  `crates/openwepp-hillslope-orchestrator/src/strict_v8_endpoint.rs`, and
  `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/`:
  lower-boundary routing into the strict runtime and covered energy join.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`:
  support-duration-bound Stage-3 reconciliation validation for the admitted
  1,800-second persistent-support API.
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_tests.rs`:
  two-day provider-bound capability, GSI/cursor sequence poison regressions,
  and the persistent covered V11/Stage-3 shared-carrier test.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`: public typed exports.
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`:
  approved v15 Option-A OFE-ground Stage 3 lane area authority, complete typed
  tile closure, terminal identity, and restart topology rule admitted by direct
  user decision and dual review/verification on 2026-08-22.
- `docs/specifications/science-contracts/index.md` and
  `tests/integration/snow_stage3_shared_carrier_authority_contract.rs`: v15
  lifecycle registry and contract-derived binding guards.
- `tools/release/authority-policy/impact-map.json`: atomic
  current-contract bindings for the Stage 3 terminal handoff, covered parent
  attachment, component carrier, and post-finalization coupled-time owner join.
- `assurance/v2/identity.lock.json`, affected assurance review locks, and typed
  `assurance/v2/transactions/*.json`: planner-authorized source-adoption chain;
  no assurance identity was hand-edited.

`Static:` Covered execution is admitted only through the distinct typed
adopter and covered segment input. It derives live carrier operands internally,
uses the carrier as the exact Stage-3 boundary, and retains the existing
snow-free adopter and its snow-present guard unchanged. The generic covered
LSE endpoint still contains lower-surface physics that must be replaced or
explicitly separated before closure. Terminal chronology, runner-owned
physical support construction, restart, and scenario qualification remain
outside this increment.

Provider-atmosphere checkpoint additions:

Current-state projector/HOLD additions modify only the already owned Stage 3
hydrology support/solver, open-boundary, terminal-handoff, covered-execution,
attachment, exports, package regression, and package evidence files. No
selector, default, production output, precipitation, terminal-liquid,
snow-soil-heat, restart, or cutover file is added.

WB14 authority checkpoint additions are limited to the prospective,
unreachable state machine and vectors in
`direct_runtime/surface_liquid_wb14.rs`, the explicitly unreleased amendment
in `SC-SURFACELIQUID-001.md`, its unchanged-v7 registry disposition, package
evidence, and `docs/ROADMAP.md`. No production ingress, attachment, selector,
output, restart, precipitation, terminal-liquid, snow-soil heat, or Richards
source is modified.

Complete surface-liquid WB14 child-slab integration checkpoint expansion:

- `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`,
  `docs/specifications/science-contracts/index.md`, and package-local v8 cycle
  artifacts: formal in-review authority and lifecycle evidence.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/{surface_liquid_wb14.rs,surface_liquid_ingress.rs,surface_liquid_owner.rs}`:
  immutable identity, exact child support, receipt reconstruction, complete
  surface transaction staging, and final-only persistent continuation.
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/{mod.rs,covered_derived_ingress.rs,strict_v8_endpoint.rs}`:
  explicit parent-local child context carried through the existing unified
  hydrology candidate.
- `crates/openwepp-hillslope-orchestrator/src/{snow_stage3_v11_attachment.rs,v11_covered/*.rs}`:
  dynamic coupled cadence, complete-owner child joins, parent finalization,
  rollback injections, and package-owned fixtures.
- Package-owned unit/integration tests, contract guards, and evidence files
  required by the exact terminal diff. Restart/provider/GSI or assurance files
  may be added only after a package-manifest amendment naming the discovered
  direct dependency.

- `crates/openwepp-hillslope-orchestrator/src/stage3_parent_atmosphere.rs`:
  provider-derived parent-atmosphere receipt and canonical digest.
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/{receipt_sets,execution,fixed_point,regime,open_snow}.rs`:
  behavior-preserving decomposition of the former 2,992-line covered module;
  `mod.rs` is now include-only wiring and every extracted file is below 2,000
  lines.
- `tests/integration/paradigm2_stage0_surface_energy_balance_contract.rs` and
  `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs`:
  exact source-path and current v16 contract-registry reconciliation exposed
  by the mechanical split/frost profile; authority anti-evasion remains
  unchanged and was rerun.
