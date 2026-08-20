# Implement C3 Woody V11 Segmented Support

Status: `queued / coupled-time prerequisite satisfied / contract-first gate next`

Date: `2026-08-20`

Package ID: `20260820-c3-woody-v11-segmented-support-001`

Plan class: `Critical vegetation authority and implementation`

Execution base: `d59ba76f7f514a98ba0f67f764b289206f9f94b9`

## Objective

Release `OPENWEPP_C3_WOODY_V11` as the first physical adopter of
`SC-COUPLEDTIME-001@2`. Import the complete immutable V10 constitutive stack and
supersede only transaction-time integration and receipt chronology. Prove exact
full-support V10 physical compatibility, ordered unequal-support execution,
one parent transaction increment, and one atomic complete-owner commit.

## Protected Boundaries

- Do not edit V10 behavior, configuration identity, state, or frozen vectors.
- Do not alter DirectV10 restart V1 or coupled-time restart V2 bytes.
- Do not decide snow-covered turbulent equations; those belong to Child 2C.
- Keep V11 default-off; do not change production selectors or deployment.

## Required Authority

Define V11 configuration, state, migration, segment forcing, resource staging,
parent finalization, receipts, restart, and error precedence. Migration converts
V10 `dt_s` to exact nanoseconds and must round-trip to identical binary64 bits.
Physical V10 state migrates identity-only.

Every duration-sensitive V11 operation consumes the exact `duration_s_bits`
from an admitted coupled-time slab. Classify algebraic rates, support
integrals, sequential state, once-per-parent operations, event transitions,
and accepted-only diagnostic reductions. No owner commits per segment.

## Acceptance Population

Required positive cases: 1800; 600+1200; 1200+600; 1 ns+remainder;
remainder+1 ns; three unequal segments; events at parent start/end;
zero-remainder receiver skip; mid-parent restart; consecutive segmented
parents. Segment forcing must differ so order is observable.

Required poisons: scaled V10 output; shortened cloned V10 configuration;
independent tick conversion; segment 1 from parent beginning; two transaction
increments; per-segment commit; gap/overlap; wrong slab or participant set;
cross-segment water/nitrogen overbooking; scheduled-once per segment;
zero-duration slab; event rate integration; rejected-segment mutation; and
restart replay.

## Full-Support Compatibility Gate

One V11 segment spanning the nominal interval must exactly reproduce every
non-identity V10 physical payload: requests/final amounts, radiation, gas
exchange, hydraulic operands, canopy liquid, C/N pools, T10, phenology,
material transfers, beginning/ending owner amounts, diagnostics, and branches.
Only V11 model/config/state identities and coupled-time receipts may differ.
Maintain a generated field ledger for all comparisons and permitted identity
differences.

## Phases

1. Inventory V10 duration, transaction, owner, restart, and receipt surfaces.
2. Author the V11 successor contract, schemas, vectors, independent reference,
   and contract-derived tests.
3. Complete two independent authority reviews, disposition, and two
   verifications; commit the exact authority checkpoint before Rust edits.
4. Implement V11 migration, staged segment execution, parent resource
   arbitration, restart, and one atomic finalization/commit.
5. Prove full-support compatibility and the unequal-support/poison population
   through the actual V11 vegetation transaction consumer.
6. Run three implementation reviews, disposition all findings, heavy gates,
   dual terminal verification, and release default-off.

## Progress

- [x] (2026-08-20) Coupled-time prerequisite released and lifecycle reconciled.
- [x] (2026-08-20) Scaffolded Child 2B at exact base.
- [ ] Complete V10 surface inventory and freeze intent.
- [ ] Release V11 contract authority through dual review and verification.
- [ ] Implement and validate the actual V11 consumer.
- [ ] Complete final review, verification, and Child 2C handoff.

## Release Boundary

`COMPLETE / OPENWEPP_C3_WOODY_V11 segmented-support authority and
implementation released / full-support V10 compatibility proven / default-off
only`.
