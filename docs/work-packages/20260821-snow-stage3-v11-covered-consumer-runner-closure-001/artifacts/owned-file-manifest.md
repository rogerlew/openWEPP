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
