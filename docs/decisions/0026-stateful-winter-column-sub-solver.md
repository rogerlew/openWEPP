# ADR-0026: Stateful winter-column sub-solver for snow/frost

**Status:** Accepted
**Date:** 2026-06-23 UTC
**Deciders:** Roger Lew (operator ratification), Codex (draft)
**Builds on:** [ADR-0011](0011-architecture-first-top-down-science-contracts.md),
[ADR-0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md),
[ADR-0025](0025-array-native-hillslope-day-frame.md)
**Design authority:**
[`docs/architecture/coupled-frost-sub-solver-specification.md`](../architecture/coupled-frost-sub-solver-specification.md)
**Science authority:**
[`SC-SNOWFREEZE-001`](../specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md)

## Context

ADR-0025 adopted a direct-frame runtime where phases are typed functions over
borrowed frame views. That remains the normal array-native shape. R7G proved a
narrow exception is required for snow/frost: frost is not a feed-forward daily
water-balance phase. It is a coupled, stateful, hourly winter process with
persistent freeze/thaw fronts, fine-layer state, layer shadows, liquid/frozen
exchange ledgers, and a snowpack insulation dependency.

The current direct-runtime snow/frost retrofit can reach endpoint execution with
zero compatibility-edge counters, but it does so by carrying and projecting
state around request/symbol-surface frost and snow bridges. R7G showed that this
shape cannot simultaneously preserve fine/shadow frost state, avoid unsafe
coarse-layer projection, meet HBP/WAT/PASS parity, and keep H2637 within the
`<=10x` performance gate.

## Decision

Adopt a stateful **winter-column sub-solver** as the accepted direct-runtime
architecture for snow/frost.

The winter column is a narrow exception to the ordinary direct-frame pure-phase
model. It owns lane-persistent `DirectWinterColumnState`, with distinct typed
snow and frost sub-states, and mutates that state in place. It may run internal
hourly loops and staged pre/post hydrology steps over the same mutable state
when source ordering requires that shape. To the outer direct executor, it is a
typed day-level producer that emits typed downstream operands and publication
operands; it is not a compatibility request surface.

Binding rules:

- `SC-SNOWFREEZE-001` remains the authority for physics, guards, units, and
  snow/frost producer obligations.
- Snow remains a distinct typed sub-state/sub-solver with independent
  `SC-SNOWFREEZE-001` obligations; frost does not own snow physics.
- Frost thermal forcing reads prior snowpack state. Same-day snow partition
  mutates snow state afterward for liquid forcing and publication.
- Persistent fine/shadow frost carry is not coarse layer projection. Coarse
  layer mutation requires an explicit closed liquid/frozen storage exchange.
- Production direct winter execution must not build or consult
  `DirectFrostRunoffSurface`, `HillslopeKernelRequest`,
  `HillslopeWritebackSurface`, `BoundarySymbol`, `BoundaryValue`, WB13 rows, or
  map-backed symbol helpers in the winter hot path. Those adapters are allowed
  only in named test/comparator seams.
- The current direct-runtime snow/frost retrofit is temporary scaffolding. It
  must be removed after consumers are cut over to winter-column operands.

## Consequences

Positive:
- R7G has a viable closure route that matches the process shape instead of
  extending the failed one-day symbol-surface retrofit.
- The snow/frost state authority is localized, making no-material carry,
  fine/shadow persistence, coarse-layer mutation, and publication projection
  independently testable.
- The direct executor can preserve the array-native no-compatibility hot-path
  contract while allowing a controlled stateful internal solver.

Negative / cost:
- This is a larger migration than another R7G patch. It requires a new module
  boundary, typed winter state, consumer cutover, and deletion of current direct
  snow/frost plumbing.
- The implementation must trace whether frost liquid partition needs post-ET
  state before finalizing a one-step versus pre/post API.

Required gates:
- R7G terminal gates rerun from the winter-column architecture:
  `compatibility_edge_invocations=0`, no winter hot-path compatibility/symbol
  references, H2637 direct default `<=10x` legacy, HBP/WAT/PASS/loss/plot/
  manifest byte/Arrow identity, manifest metadata parity, anti-alias fixtures,
  and independent snow/frost operand reconstruction.
- Internal solver diagnostics are validated under `SC-SNOWFREEZE-001` semantic
  parity and named tolerances unless a field is explicitly declared bit-exact.
  Public output gates remain byte/Arrow identity.

## Non-decisions

This ADR does not change snow/frost physics, `SC-SNOWFREEZE-001`, output
schemas, default direct activation, compatibility rollback policy, or R7
performance/parity gates. It authorizes only the runtime architecture exception
and the deletion path for the current direct-runtime snow/frost retrofit.
