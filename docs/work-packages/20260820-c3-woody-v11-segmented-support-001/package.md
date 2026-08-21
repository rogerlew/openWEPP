# Implement C3 Woody V11 Segmented Support

Status: `COMPLETE / Child 2B released / Child 2C authorized`

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

## Included Scope

- Successor authority in `SC-VEGETATION-001` and
  `SC-VEGETATIONTRANSACTION-001`, schemas, vectors, independent reference, and
  contract-derived tests.
- Additive V11 configuration/state/migration, segment execution, staged
  resource custody, restart, parent finalization, and receipts.
- The actual default-off orchestrator V11 transaction consumer and exact V10
  full-support comparison.

## Excluded Scope

- V10 mutation, snow-covered turbulent science, Richards/Lane D physics,
  calibration, production selection/defaults, deployment, release, and push.
- Approximate/scaled partial-support results or caller-owned duration overrides.

## Dependencies

- Execution/diff base `d59ba76f7f514a98ba0f67f764b289206f9f94b9`.
- Scaffold identity `3bc8562bff700722e928e631280cf13a8b171ee9`.
- Released `SC-COUPLEDTIME-001@2` and
  `OPENWEPP_COUPLED_TIME_SUPPORT_V1` at Child 2A commit
  `d75fc548482606eab46c603497c77f0ea7b32e49`.
- Immutable V10 vegetation and transaction authority.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`
- `docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md`
- `docs/specifications/science-contracts/index.md`
- additive `crates/openwepp-vegetation/src/v11_*.rs`, `src/lib.rs`, and
  `src/config.rs` only where V11 types/access require it
- `crates/openwepp-hillslope-orchestrator/**` only for the actual default-off
  V11 consumer
- `crates/openwepp-persisted-restart-v1/**` only for additive V11 restart types
- `tests/integration/c3_woody_v11_*`, `Cargo.toml`, `Cargo.lock`, this package
  tree, and truthful campaign/roadmap/catalog lifecycle files

Any expansion requires a prospective scope, risk, and gate amendment before
edits. V10 behavior and frozen wires remain protected even inside listed files.

## Deliverables

1. Dual-reviewed and dual-verified V11 successor authority.
2. Independent schemas, vectors, reference calculator, and compatibility ledger.
3. Additive V11 implementation and exact migration.
4. Actual-consumer proof for full and segmented support, restart, custody, and
   atomic parent commit.
5. Three implementation reviews, finding disposition, heavy gates, dual
   terminal verification, and concrete Child 2C handoff.

## Required Authority

Define V11 configuration, state, migration, segment forcing, resource staging,
parent finalization, receipts, restart, and error precedence. Migration converts
V10 `dt_s` to exact nanoseconds and must round-trip to identical binary64 bits.
Physical V10 state migrates identity-only.

Every duration-sensitive V11 operation consumes the exact `duration_s_bits`
from an admitted coupled-time slab. Classify algebraic rates, support
integrals, sequential state, once-per-parent operations, event transitions,
and accepted-only diagnostic reductions. No owner commits per segment.

Authority release is blocked until it decides every resource chronology:

- Water requests, authorizations, and finalized uses are segment-local against
  the current staged owner; cumulative parent debits are independently
  reconstructed and cannot overbook or commit live per segment.
- Potential/final nitrogen demand retains NH4/NO3 identity and authorizes
  against staged inventories; later segments cannot reuse parent beginnings.
- Canopy liquid, T10, NSC, tissue pools, phenology timers, hydraulics warm
  starts, and all C/N state advance sequentially from each staged ending.
- Material/litter authority must choose and test either per-segment proposals
  accumulated once at parent finalization or a once-only final-state proposal;
  implementation may not choose this chronology implicitly.
- Inventory and classify GSI receipt consumption, phenology edges, management
  events, calendar transitions, daily initialization, material finalization,
  and transaction increment as segment, event, or scheduled-once operations.

## Positive-Support Authority

Coupled-time nanosecond chronology remains structurally admissible. The actual
V11 covered-forest LSE adopter has a sealed, deterministic positive-support
domain with minimum `600000000` ns. A support at or above that boundary carries
`LseSupportAdmissibilityReceiptV1`, bound to parent/segment/slab/absolute
support, duration bits, LSE and soil-thermal beginning identities, model and
configuration identities, tolerance/numerical policies, and its digest. A
support one tick below the boundary rejects before Newton with no owner,
candidate, receipt, or checkpoint mutation. The domain is scoped to the
executed covered-forest adopter profile; other fixture profiles require their
own authority cycle. No hidden floor, scaling, tolerance relaxation, or V10
change is permitted.

## Restart Authority

Define and preserve additive `OPENWEPP_C3_WOODY_V11_RESTART_V3` before production code. It
persists parent coupled-time identity, accepted segment/slab cursor, staged V11
state, staged water/BGC/energy/soil-thermal candidates where applicable,
accepted resource and scheduled-once receipts, and the parent-beginning
complete owner set. Prove fresh-object, mid-segment, and event-boundary restore;
rejected-attempt absence; exact parent abort; and no replay. DirectV10 restart
V1 and coupled-time restart V2 bytes remain unchanged.

## Acceptance Population

Required positive cases: 1800; 600+1200; 1200+600; three unequal segments;
coupled-time 1 ns structural identity; physical support exactly at the admitted
minimum; events at parent start/end;
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
   arbitration, sealed positive-support receipts, restart, and one atomic
   finalization/commit.
5. Prove full-support compatibility and the unequal-support/poison population
   through the actual V11 vegetation transaction consumer.
6. Run three implementation reviews, disposition all findings, heavy gates,
   dual terminal verification, and release default-off.

## Validation And Exit Criteria

Contract/profile/schema gates, independent reference populations, exact V10
compatibility, unequal-support and poison tests, restart/rollback tests,
resource conservation reconstruction, focused crate/consumer tests, formatting,
Clippy with warnings denied, cargo-deny, full-workspace correctness, exact diff,
line counts, three reviews, disposition, and dual terminal verification must
all receive truthful current-tree evidence. Self-consistency alone does not
accept conservation or output claims.

## Security And Data Impact

Local repository files only. No credentials, external connectivity, messages,
deployment, release, or production activation. Canonical JSON/wire admission is
closed and fail-closed; no untrusted caller may forge support or owner custody.

## Calibration And Evidence Posture

This is architecture and exact successor-compatibility work, not calibration.
V10 is the physical compatibility authority for one full support; coupled time
owns chronology. Comparator agreement is diagnostic beyond the exact V10 gate.

## Review And Subagent Authorization

Required independent roles: two authority reviewers and two authority
verifiers before Rust; three implementation reviewers; one
`comparator_suite_runner` for heavy gates; and two terminal verifiers. This
package explicitly authorizes those bounded subagents. Reviewers and verifiers
are read-only except their named artifacts; the runner writes only ignored logs
and bounded package evidence.

## Line-Count Governance

Apply repository WARN at 2,000 lines and block at 3,000 lines unless an approved
generated/fixture exception records owner and sunset. Record all touched Rust
file counts before disposition.

## HOLD Legitimacy

A HOLD is valid only for an exact authority, wire, dependency-cycle, or
owner-atomicity contradiction after safe in-scope routes are exhausted.
Implementation volume, refactoring, failing tests, schema size, or heavy-gate
cost are not HOLD reasons.

## Progress

- [x] (2026-08-20) Coupled-time prerequisite released and lifecycle reconciled.
- [x] (2026-08-20) Scaffolded Child 2B at exact base.
- [x] (2026-08-20) Complete V10 surface inventory and freeze intent.
- [x] (2026-08-20) Release positive-support V11/LSE authority through dual review and verification.
- [x] (2026-08-20) Complete and terminally validate the actual V11 consumer,
  including sealed receipt custody.
- [x] (2026-08-20) Complete three implementation reviews, dual terminal
  verification, and the Child 2C handoff.

## Release Boundary

`COMPLETE / OPENWEPP_C3_WOODY_V11 segmented-support authority and
implementation released / full-support V10 compatibility proven / default-off
only`.

## Outcomes & Retrospective

Released default-off. The physical adopter retains a deterministic
600000000 ns minimum positive-support domain for the executed covered-forest
LSE profile; coupled-time 1 ns remains structural chronology, not a promise
that every constitutive solver can resolve a storage lattice at one tick.
Full-support V10 compatibility, ordinary unequal support, staged owner
custody, sealed support receipts, rollback, and Restart V3 retention are
terminally verified. Child 2C owns the independently reviewed event-boundary
coalescing and snow-covered shared-carrier authority; no snow equations or
production selector changed in this package.
