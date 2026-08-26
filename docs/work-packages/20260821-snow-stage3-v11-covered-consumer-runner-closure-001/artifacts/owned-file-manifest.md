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

Restart V1 fail-closed and exact-head assurance reconciliation expansion:

Multi-lane Stage-3 parent qualification expansion:

- `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`,
  registry, and its integration guard: version-9 lane-keyed authority release.
- `snow_stage3_v11_attachment{.rs,_tests.rs}`: removal of the released
  one-resolved-lane cap and retained lifecycle coverage.
- `land_surface_energy_shadow/v8_input_projection.rs`,
  `v9_real_consumer_shadow.rs`, and `v11_covered/owner_finalization.rs`:
  exact multi-OFE ordinary-boundary, root/OFE, soil-temperature, and BGC
  identity corrections exposed by the real parent fixture.
- `v9_real_consumer_shadow{_tests.rs,_wb14_tests.rs}`: genuine two-OFE
  snow/snow-free and dual-resolved-snow parent fixtures, cadence, lane receipt,
  inactive-owner, topology, publication, and rollback evidence.

- `crates/openwepp-persisted-restart-v1/src/hydrology_restart.rs` and
  `tests/integration/direct_v10_real_consumer_checkpoint_v1_contract.rs`:
  reject both legacy and production Stage-3 owners at the frozen V1 boundary,
  validate explicit-null poison syntax, and execute complete generated-schema
  identity.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver{.rs,/persistent_state.rs}`:
  behavior-preserving persistent-state responsibility split required by the
  existing 3,000-line structural guard.
- Exact-head stale contract/registry/immutable-definition guards in
  `tests/integration/{snow_stage3_terminal_receiver_authority_contract.rs,snow_stage3_turbulent_operator_reconciliation_contract.rs,snow_stage3_wind_source_custody_contract.rs,snow_surface_eb03_contract.rs,v10_nighttime_authority_contract.rs,vegetation_boundary_authority_contract.rs}`.
  Historical V10/LSE authority bytes and digests remain unchanged.

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

BGC/OFE identity corrective expansion:

- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`, its
  lifecycle row, contract guard, and typed assurance source-adoption records:
  exact-one BGC-bearing-OFE authority for the existing hillslope-global BGC
  owner; no BGC state-schema or physics change.
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs`
  and package-owned orchestrator tests: explicit LSE
  `vegetation_tile_id` joins, stratum-wide OFE resolution, BGC transition/debit
  bijection and exact pool-delta checks, plus ambiguity/order/rollback poisons.
- `tests/integration/snow_stage3_terminal_receiver_authority_contract.rs`:
  lifecycle-only registry wording reconciliation from vegetation v26 to v27;
  terminal receiver authority and assertions remain unchanged.

BGC fold/scope corrective expansion:

- `crates/openwepp-biogeochemistry/src/lib.rs`: canonical pre-hash semantic
  ordering of the stratum-scoped mineral-N protocol before pool accumulation.
- `crates/openwepp-vegetation/src/v11.rs`: configuration-aware live and restore
  validation of BGC debit scope plus semantic transition-link fold ordering.
- `crates/openwepp-vegetation/src/v11/tests/v11_bgc_tests.rs`: extracted
  adversarial live/restore ordering and scope fixtures; keeps the production
  module below the 3,000-line closure ceiling.
- `crates/openwepp-persisted-restart-v1/src/vegetation_v11.rs`: carries the
  caller-derived BGC scope into V11 checkpoint custody validation.
- `tests/integration/snow_stage3_v11_constitutive_boundary_contract.rs`:
  source guard reconciliation to the scoped direct-consumer wrapper.
- Existing vegetation contract, registry, authority guard, covered-owner
  finalization, package tests/evidence, roadmap, campaign pointer, and catalog.

Persistent covered physical-custody expansion:

- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_precipitation.rs`
  and `snow_stage3_v11_snow_soil_heat.rs`: sealed phase-parcel and OFE/lane
  conductive-boundary receipts, including exact support, topology, beginning
  owner, physical operands, candidate endings, and reconstructable digests.
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/physical_outcome_ledger.rs`:
  independent postcandidate mass, vapor, and energy reconstruction that cannot
  feed solver operands or acceptance residuals.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/liquid_routing.rs`
  plus `v9_real_consumer_shadow_physical_custody_error.rs` and
  `v9_real_consumer_shadow_serialization.rs`: behavior-preserving
  same-module splits keeping touched production sources below the hard
  3,000-line ceiling.
- The affected Stage-3 reconciliation, covered execution/fixed-point/owner,
  LSE boundary, runner input, tests, contract, registry, authority guards, and
  package evidence files shown by the terminal diff. No selector, default,
  production output, CoE, frozen restart wire, or publication behavior changes.

Terminal chronology correction expansion:

- `docs/specifications/science-contracts/contracts/{SC-SNOWENERGY-001,SC-LANDSURFACEENERGY-001,SC-SNOWFREEZE-001,SC-COUPLEDTIME-001}.md`
  and `docs/specifications/science-contracts/index.md`: coordinated prospective
  authority for read-only covered discovery, exact per-trial covered carrier
  reconstruction, terminal snow--soil endpoint custody, canonical snow-owner
  pending parcels, exact event mutation/ordinal authority, and reconstructable
  terminal receipts.
- Contract-derived integration guards for the four successor versions and
  their no-raw-carrier, no-fabricated-temperature, owner-custody, mutation-set,
  ledger, and receipt-chain obligations.
- Existing covered Stage-3 terminal solver, V11 covered carrier, coupled clock,
  V11 parent transaction, canonical snow-owner, attachment/receipt, and
  responsibility-specific terminal test modules needed to implement those
  reviewed contracts. The attachment must be decomposed below 3,000 lines
  before terminal PASS.
- This expansion does not admit runner-owned 48-support construction,
  terminal-liquid receiver consumption, additive restart, selectors/defaults,
  activation, CoE retirement, or production cutover.

## 2026-08-25 real terminal DAE exploration exact write set

The terminal diff from base `64fdeb02942f62efd92428ef538440596b90668f`
contains only:

- `crates/openwepp-hillslope-orchestrator/src/terminal_candidate_numerics.rs`:
  test-only exact-support, scaling, validation, conditioning, reference-method,
  and package-Python parity corrections.
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs`:
  exact immutable-fixture cardinality assertions.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00h_snow_stage3_evaluation_trace.rs`:
  test-only workspace-compile correction; the external fixture obtains a
  legitimate result from the existing dormant evaluator instead of directly
  constructing a struct with a private ending-joint field. No runner
  production path or public API changes.
- `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/package.md`:
  owner-directed implementation intent and red-line reconciliation.
- `artifacts/required-reading-map.md` and
  `artifacts/post-v21-numerical-exploration-progress.md`: intake and superseded
  planning-inventory corrections.
- `artifacts/README.md`: pointer to the current Child-1 real-DAE disposition.
- `artifacts/terminal_receipt_dag_v7_tool/src/main.rs` plus generated
  `artifacts/terminal-receipt-dag-v7-evidence.{json,md}`: nine node-local
  poison constructions through canonical Rust framing.
- `artifacts/real-terminal-dae-defect-hold.md`: concrete source-resolved
  `x/z/q/p` layout, actual graph/SCC correction, and stop-condition proof.
- This manifest, `implementation-and-test-evidence.md`, `gate-results.md`, and
  the review/disposition artifacts added for terminal reconciliation.
- `artifacts/real-terminal-dae-{numerical-review,science-ownership-review,review-disposition,verification}.md`
  and `artifacts/heavy-{orchestrator-20260825-235333,orchestrator-20260826-000033,workspace-20260826-000829}.log`:
  independent review/verification/disposition, initial/final affected-suite
  receipts, and the first clean-SHA workspace compile-failure receipt.

No other crate, contract, registry, assurance, dependency, public API,
production execution, or output file is modified.

Line-count governance: the test-only numerical module is 927 lines and the
package receipt tool is 273 lines. The touched
`v9_real_consumer_shadow_wb14_tests.rs` is a pre-existing 2,306-line test
fixture, above the 2,000-line warning threshold but below the hard 3,000-line
limit. This increment changes only four existing cardinality assertions there;
decomposing the large shared fixture would be unrelated architectural churn
for a defect-shaped numerical HOLD. Follow-up decomposition remains advisable
when substantive fixture structure next changes.
The runner trace file is 1,627 lines after its test-only compiler correction.
