# Surface-owner V2 current-ingress/WB14 adapter evidence

Status: `PASS — BOUNDED ADAPTER SLICE`

Evidence mode: `Static + Ran`

## Implemented boundary

- `DirectSurfaceLiquidResourceCandidateV2` binds the immutable beginning V2
  owner, phase-adjusted V2 owner, phase closure, and the existing liquid-only
  ingress arithmetic.
- `DirectSurfaceLiquidIngressCandidateV2` reconstructs the ending V2 owner
  from actual ingress/WB14 receipts. Only liquid can enter the existing WB14
  arithmetic; litter ice is carried bitwise and cannot become supply,
  infiltration, runoff, `frozwt`, or current-ingress donation.
- `DirectWb14ParentWorkingStateV2` persists the V2 beginning/candidate owners
  and the exact existing WB14 parent arithmetic. Canonical restart replay binds
  the V2 configuration and model identities and rejects mixed-owner state.
- The adapter invokes
  `execute_surface_liquid_ingress_with_parent_state_and_coupled_binding`; it
  does not duplicate or replace WB14 arithmetic.
- The internal V1-shaped value is an unpublished arithmetic carrier only. It
  is never serialized or exposed as a V1 owner/checkpoint/migration, and the
  production path never projects or downgrades a V2 owner to V1.

No `SurfaceLiquidCompleteOwnerProjectionV3`, production real-consumer, or
cutover claim is made by this bounded slice.

## Closure and poison evidence

The V2-focused tests prove:

- current ingress reaches the actual WB14 implementation with liquid-only
  eligibility and matches legacy V1 liquid arithmetic and receipts;
- ending liquid and enthalpy reconstruct from retained-surface receipts while
  litter ice survives bitwise;
- phase omission, double debit, mixed V1 identity, invalid WB14 input, and
  open-parent omission fail closed;
- a rejected child leaves the V2 parent restart bytes unchanged; and
- two child intervals survive exact restart/replay and close only at parent
  finalization.

Legacy owner, ingress, and WB14 focused suites remained unchanged and passed.

## Commands and results

- `cargo nextest run -p openwepp-hillslope-orchestrator --lib surface_liquid_owner::v2_ingress_tests --no-fail-fast`
  — PASS, 3/3 (terminal run `eb897bca-65b1-4f36-bed3-f49ffb4848c0`).
- `cargo nextest run -p openwepp-hillslope-orchestrator --lib surface_liquid_owner --no-fail-fast`
  — PASS, 56/56 (`5a33ae32-3926-477c-9830-5d4df3e0028a`).
- `cargo nextest run -p openwepp-hillslope-orchestrator --lib surface_liquid_ingress --no-fail-fast`
  — PASS, 54/54 (`2f287e5b-b93b-42d9-acad-d8445093daf4`).
- `cargo nextest run -p openwepp-hillslope-orchestrator --lib surface_liquid_wb14 --no-fail-fast`
  — PASS, 20/20 (`0da846cf-6d19-4b16-a9a9-0126a6276c25`).
- `cargo check -p openwepp-hillslope-orchestrator` — PASS.
- warnings-denied Clippy for the shared crate — FAIL from concurrent/unowned
  diagnostics; the terminal log is
  `/tmp/frozen_litter_boundary1_clippy_terminal.log` with SHA-256
  `8a80ab0f4f19d5fde8ed20a40a3a38df45d41406a5371dd23c6fc8def6f9423a`.
  An exact-path scan reports zero diagnostics in this slice's three Rust
  files.
- v14 authority scan
  `version_14_binds_frozen_litter_surface_owner_v2_before_production` — expected
  next-boundary RED (run `c97bd34d-3a27-442e-8b19-ad2048a8cbd7`), solely because
  the explicitly excluded `SurfaceLiquidCompleteOwnerProjectionV3` production
  consumer is not yet installed.

## Handoff API

The successor complete-owner projection may consume the crate-private module
`direct_runtime::surface_liquid_owner::v2_ingress_adapter` through:

- `prepare_surface_liquid_resource_candidate_v2`;
- `execute_surface_liquid_ingress_v2`;
- `execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding`;
- `DirectSurfaceLiquidResourceCandidateV2`;
- `DirectSurfaceLiquidIngressCandidateV2`; and
- `DirectWb14ParentWorkingStateV2`.

The successor remains responsible for the real LSE V3 receipt join, complete
owner projection, atomic commit/publication, and the terminal v14 authority
gate.
