# PERFDEEP01 Contract Transition Map

Evidence: Static.

## Objective

Define how `HillslopeDayFrame` coexists with current `openwepp-kernel-contract` payload contracts during migration so Stage-1+ can proceed without ad-hoc cross-crate breakage.

## Current Authoritative Contract Surface (Unchanged in Stage-0)

- Kernel request/response contracts remain map/payload oriented (`HillslopeKernel`, `KernelWritebackPayload`, indexed payload support).
- Scheduler authority remains `HillslopeWritebackSurface` logical maps.
- Stage-0 frame is shadow-only and does not replace contract types.

## Coexistence Model by Stage

1. Stage-0 (this package):
- Authoritative: logical `HillslopeWritebackSurface`.
- Shadow: `HillslopeDayFrame` seeded/flushed for identity verification.
- Contract impact: none (no signature or payload shape change).

2. Stage-1/Stage-2 (phase-island migration):
- Authoritative remains logical maps at scheduler boundary.
- Migrated phases operate on frame fields internally.
- Boundary adapter performs deterministic projection:
  - map -> frame before migrated phase
  - frame -> map after migrated phase writeback
- Contract impact: no external `openwepp-kernel-contract` break during island rollout.

3. Cutover Stage (frame-authoritative hot path):
- Authoritative shifts to frame in scheduler.
- Logical maps become edge-projection artifacts only (I/O/publication/legacy adapter boundaries).
- Contract transition option selected here:
  - Preferred: adapter-bridge continuity first, then optional contract-version cleanup once all consumers are off map-native internals.

## Compatibility Matrix

- `openwepp-hillslope-orchestrator`:
  - Stage-0: dual representation (map authoritative + frame shadow)
  - Stage-1+: mixed representation via adapters per migrated phase island
- `openwepp-kernel-contract`:
  - Stage-0/Stage-1+: unchanged public contract
  - Cutover: may introduce versioned typed payload only after adapter parity and downstream readiness
- `openwepp-runner` publication path:
  - Stage-0: reads logical surfaces as today
  - Future: reads frame projections at edge once publication operand map is fully migrated

## Non-Negotiable Transition Invariants

- No loss of diagnostic attribution semantics at guard failures.
- No silent bound canonicalization beyond existing contract policy.
- Bit-identical edge outputs during shadow/mixed stages unless a ratified contract explicitly permits a change.

## Stage-0 Decision

Transition strategy is `adapter-bridge first`:

- Preserve existing cross-crate contract signatures while migrating execution internals.
- Delay contract-shape break to an explicit later package after frame-authoritative behavior is established and validated.
