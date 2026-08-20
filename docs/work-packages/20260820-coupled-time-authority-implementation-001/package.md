# Implement Coupled Time Authority V1

Status: `queued / Child 2A next`

Date: `2026-08-20`

Package ID: `20260820-coupled-time-authority-implementation-001`

Plan class: `Critical contract-first cross-domain authority and implementation`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Objective

Admit `SC-COUPLEDTIME-001` / `OPENWEPP_COUPLED_TIME_SUPPORT_V1` and implement a
reusable subsystem that gives modeled time exactly one staged owner across a
sealed parent forcing interval. Prove exact integer support identity, contiguous
physical segmentation, common accepted slabs, failed-attempt rollback,
restartable adaptive control, atomic multi-owner acceptance, and parent-buffered
publication through an orchestrator-level reference consumer. A type-only
crate, vegetation-only override, or clock that owners can bypass does not pass.

## Rationale

The terminal snow HOLD showed that duration is currently configuration identity,
forcing cadence, transaction identity, and an implicit numerical operand.
Future snow events, surface-routing CFL limits, adaptive Richards steps,
soil--plant iterations, boundary-mode events, and mid-interval restart all need
the same chronology. A shared authority must precede V11 vegetation and must be
reused by `RichardsCoupledV1`; neither adopter may invent a private clock.

## Authority To Establish

The contract must distinguish nominal forcing intervals, physical regime
segments, accepted coupled slabs, provisional numerical attempts, and
zero-duration event instants. Canonical identity uses unsigned integer
nanosecond ticks and half-open `[start,end)` support; binary64 seconds are a
derived numerical operand only. Operations are classified as `AlgebraicRate`,
`SupportIntegral`, `SequentialStateTransition`, `ThresholdEvent`,
`ScheduledOnce`, or `DiagnosticReduction`, with explicit segmentation/retry
semantics.

## Included Scope

- New `SC-COUPLEDTIME-001` contract, registry row, invariants, typed guard/error
  precedence, schemas, deterministic vectors, and independent reference model.
- A focused reusable Rust crate, provisionally `openwepp-coupled-time` (an
  existing lower-level crate may be selected only with recorded dependency-cycle
  proof), containing validated support identity, staged clock state, segments,
  slab candidates, constraints, attempts, owner-set joins, controller state,
  digests/receipts, and typed failures.
- A deterministic versioned restart representation for parent support,
  accepted cursor, next ordinal, last step, required adaptive history, active
  boundary/event context, forcing identity, and accepted owner identity.
  Rejected iterates are never persisted.
- An orchestrator-level reference consumer with at least three independent mock
  owners and buffered parent output. It executes multiple segments/slabs,
  rejects and retries an attempt, proves byte-identical rollback, restores
  mid-parent, accepts owners atomically, and publishes only after parent commit.
- Compatibility enums/tuple validation for `LegacyFixedSchedule` and
  `CoupledAdaptiveSupportV1`, including the forward guard that
  `RichardsCoupledV1 + LegacyFixedSchedule` is unsupported.
- Contract-derived unit, property, negative, restart, and integration tests.

## Excluded Scope

- V10 mutation, partial-support override, configuration rehashing, or output
  scaling. V10 remains immutable.
- V11 vegetation integration/migration (Child 2B), snow-covered carrier science
  (2C), and terminal snow receiver completion (resumed Child 1).
- Richards equations, Lane D physics, soil--plant physics, or legacy hydrology
  replacement (`SC-RICHARDS-001` campaign).
- Production selectors/defaults, CoE retirement, calibration, empirical
  validation, publication, deployment, or release.

## Deliverables

1. Approved and indexed `SC-COUPLEDTIME-001` with exact schemas, invariants,
   attempt/commit protocol, restart/publication semantics, operator classes,
   error precedence, and vectors.
2. Executable Rust subsystem with no hidden owner advancement and no floating
   support identity.
3. Contract tests and independently generated reference vectors.
4. Orchestrator reference-consumer proof, including negative proof that owners
   cannot consume different slabs or publish before parent commit.
5. Multi-segment adaptive restart and rejection-rollback evidence.
6. Complete evidence, dual review/finding disposition, dual terminal
   verification, and a concrete Child 2B handoff.

## Dependencies

- Intake at `51dc26cb568ce3b5b5fb7cca861f252404a8a9a8` or an exact later
  descendant, mechanically refreshed before edits.
- Stage 3 campaign and terminal-handoff HOLD evidence.
- On-demand boundary authority: `SC-SNOWENERGY-001`,
  `SC-LANDSURFACEENERGY-001`, `SC-VEGETATION-001`,
  `SC-VEGETATIONTRANSACTION-001`, persisted-restart contracts, and applicable
  forcing/simulation identity contracts.
- No legacy comparator is acceptance authority for this architectural system;
  a small independent state-machine/reference calculator is required.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-coupled-time/**` (provisional; finalize at intake)
- `crates/openwepp-hillslope-orchestrator/**` only for the bounded reference
  consumer/integration seam
- `crates/openwepp-persisted-restart-v1/**` only if inventory proves ownership
- `tests/integration/coupled_time_*`
- root `Cargo.toml`, `Cargo.lock`, this package tree, and truthful campaign
  roadmap/catalog lifecycle updates

Any broader production-owner edit requires a prospective write-set/risk/gate
amendment. Vegetation, snow, Lane D, Richards, soil-thermal, and BGC kernels are
protected boundaries.

## Phase Plan

### Phase 0 — Intake and intent

Record exact HEAD/status, instruction chain, changed files, current duration,
receipt, restart and output owners, dependency-cycle analysis, and chosen crate
boundary. Freeze validation intent. Refresh the relevant source inventory from
current main rather than relying on the Richards assessment at `093e172c5`.

### Phase 1 — Canonical authority

Author the contract and independent vectors/reference calculator. Specify wire
limits and checked arithmetic; half-open support laws; segment/slab/event
distinctions; earliest-constraint tie rules; attempt lifecycle; owner/digest
joins; atomicity; restart; parent finalization/publication; temporal operator
semantics; compatibility tuples; and error precedence.

### Phase 2 — Contract tests and pre-implementation gate

Write tests against the contract/vectors before production Rust. Run and record
contract, profile, schema, and reference checks. Production edits are forbidden
until this gate passes.

### Phase 3 — Reusable subsystem

Implement checked integer arithmetic and typed errors. Keep attempt state
isolated from accepted state. Atomic acceptance validates beginning identities,
common support, ending identities, ledgers, ordinals, and exact coverage. Do not
expose an escape hatch for independent accepted-time advancement.

### Phase 4 — Restart and reference consumer

Implement/version restart and the orchestrator consumer. Exercise multiple
segments and adaptive slabs, a failed/reduced retry, event boundary, mid-parent
restart, scheduled-once behavior, diagnostic maximum, exact rollback, one
parent finalization, and delayed publication.

### Phase 5 — Validate, review, disposition

Reconcile exact diff/write set; format/lint; run focused crate, contract,
property, restart, integration and doctests; run applicable workspace/domain
profiles; run cargo-deny for manifest changes; audit bypasses/placeholders,
line counts, security and assurance impact. Obtain two independent reviews,
disposition every finding, rerun after fixes, and obtain two independent
terminal verifications on the final identity.

## Required Invariants And Scenarios

- `start_ns < end_ns`; checked arithmetic rejects overflow and unsupported wire
  ranges.
- Ordered segments exactly cover the parent; slabs exactly cover each segment,
  with no gap/overlap.
- Event instants can split support but cannot be integrated as slabs.
- All coupled owners consume one support and begin from one accepted owner set.
- Rejection leaves owners, clock, controller, ordinal, ledgers, diagnostics,
  and publication buffer byte-identical.
- Acceptance advances exactly to the slab end, increments the ordinal once,
  and replaces every participating owner atomically.
- The earliest constraint wins deterministically; no slab crosses a parent,
  segment, event, output, or restart boundary.
- Restart reproduces uninterrupted accepted chronology/receipts; rejected
  iterates are absent.
- `ScheduledOnce` runs once at its named boundary, not per slab; reductions use
  accepted slabs only.
- Output stays invisible until complete parent commit; peak is the maximum of
  accepted-slab values, not a volume/nominal-duration reconstruction.
- A full-support single segment is representable, but V10 equivalence belongs
  to Child 2B and is not claimed here.

## Validation And Exit Criteria

This critical increment requires current evidence for every phase on one exact
final source identity: contract/profile/index checks; independent vector
reproduction; warnings-denied lint; focused crate, contract, property,
integration, restart and doctest passes; applicable quick/frost/domain profiles;
manifest policy; exact-diff and bypass audits; line counts; dual reviews with
all findings dispositioned; post-fix reruns; and dual terminal verification.
Record exact commands, counts, hashes, failures, retries, and rationale in
`artifacts/gate-results.md`.

No required 2A gate may be deferred to 2B, 2C, Richards, or campaign closure.
If the reusable subsystem cannot run through the declared orchestrator
consumer, the package is HOLD, not complete.

## Calibration And Evidence Posture

This is architectural/numerical implementation, not parameter calibration or
empirical validation. The calibration-readiness matrix records non-applicability
with rationale and cannot imply snow, vegetation, routing, or Richards efficacy.

## Security And Data Impact

No network, credentials, external messages, protected-data mutation,
deployment, publication, default change, or release is authorized. New
serialization/digest surfaces must be deterministic, bounded, reject malformed
input, and avoid rejected-iterate leakage. Run `cargo deny check` when manifests
or the lockfile change.

## Review And Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to independent time/numerics authority, Rust/API,
serialization/restart reviewers, a bounded heavy-gate runner, and two terminal
verifiers. Outputs are compact findings, command/count/hash summaries, and
artifact paths. Reviewers/verifiers are read-only; the runner writes only
ignored logs and bounded gate artifacts.

Reviews verify semantic authority, integer/wire safety, bypass resistance,
rollback, atomicity, restart determinism, publication timing, exact-diff/gate
legitimacy, and `.rs` line counts. Files at 2,000+ lines are WARN with a split
rationale; 3,000+ nonexempt files block closure.

## Progress

- [x] (2026-08-20) Scaffolded 2A and reordered the campaign into 2A--2C.
- [ ] Complete intake and freeze intent.
- [ ] Complete authority, vectors, tests, and pre-implementation gate.
- [ ] Implement subsystem, restart, and reference consumer.
- [ ] Complete validation, review, verification, and disposition.

## Surprises & Discoveries

- None beyond the Child 1 authority finding at scaffold time.

## Decision Log

- Decision: make time a standalone cross-domain authority before V11.
  Rationale: all physical owners need one chronology; a vegetation exception
  creates competing owners. Date/Author: 2026-08-20 / Codex.
- Decision: require an orchestrator reference consumer in 2A while leaving
  physical adopters to 2B/2C. Rationale: prove executable, bypass-resistant
  infrastructure without deciding vegetation or snow equations. Date/Author:
  2026-08-20 / Codex.

## Outcomes & Retrospective

Queued. No canonical contract, production Rust, selector, default, output, or
runtime authority changed during scaffolding.
