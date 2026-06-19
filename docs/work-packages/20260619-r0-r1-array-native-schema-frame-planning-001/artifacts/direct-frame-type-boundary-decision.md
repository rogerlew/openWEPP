# Direct-Frame Type-Boundary Decision

Status: complete for planning-only scope.
Evidence mode: Static/Ran.

## Static Evidence

Ran:

```text
rg -n "pub struct HillslopeDayFrame|pub struct HillslopeLaneDenseState|Vec<Option<BoundaryValue>>|state_slots|flux_slots|SymbolRegistry" crates/openwepp-hillslope-orchestrator/src/day_frame.rs
```

Relevant findings:

- `HillslopeDayFrame` stores `symbol_registry: SymbolRegistry`.
- `HillslopeDayFrame` stores `state_slots: Vec<Option<BoundaryValue>>`.
- `HillslopeDayFrame` stores `flux_slots: Vec<Option<BoundaryValue>>`.
- `HillslopeLaneDenseState` also stores `Vec<Option<BoundaryValue>>` state and
  flux slots.

## Decision

The existing `HillslopeDayFrame` and `HillslopeLaneDenseState` are
compatibility/transition types. They must not be the direct-mode runtime types.

R0 must introduce a distinct direct-frame type family or first rename/isolate
the existing compatibility types. Preferred naming for the next implementation
package:

- `DirectRunFrame`
- `DirectLaneFrame`
- `DirectDayFrame`
- `DirectPublicationFrame`
- `DirectPhaseView`

These should live in a direct-frame module namespace so the compatibility
`day_frame.rs` types cannot be confused with direct-mode state.

## Direct-Type Prohibition

Direct-mode frame storage must not contain:

- `SymbolRegistry`;
- `BoundarySymbol`;
- `BoundaryValue`;
- `Option<BoundaryValue>`;
- `HillslopeWritebackSurface`;
- `KernelWritebackPayload`;
- `IndexedWritebackSurface`;
- `HotSymbolTables`;
- logical/dense refresh or dirty-flush state.

## Gate

PASS for planning-only R0. Future R0 implementation must prove these
prohibitions in code before runtime readiness can be claimed.
