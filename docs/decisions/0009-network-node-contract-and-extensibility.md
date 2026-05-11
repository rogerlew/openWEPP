# ADR-0009: Network node contract and extensibility policy

**Status:** Proposed  
**Date:** 2026-05-11  
**Deciders:** Roger Lew, Codex

## Context

openWEPP must support watershed network amendments over time (new routing
behaviors, optional features, and node-class additions such as reservoirs)
without destabilizing baseline hillslope/channel workflows.

Current documentation describes subprocess topology but does not yet pin a
typed, extensible node contract for watershed execution graphs.

## Decision

1. Watershed execution uses a typed directed graph contract with explicit
   `node_id`, `node_kind`, and edge connectivity.
2. Initial node kinds are:
   - `hillslope`
   - `channel`
   - `outlet`
   - `reservoir` (supported as a first-class extensibility target)
3. Node behavior is implemented through node adapters that bind `node_kind`
   to routine implementations selected through the routine lifecycle catalog
   (ADR-0008).
4. Network amendments follow explicit compatibility policy:
   - additive node metadata or optional capabilities: backward-compatible;
   - required fields or semantic contract changes: schema-major change.
5. Unknown required `node_kind` or missing adapter is a hard configuration
   error; no silent downgrade to alternate routing logic.
6. Execution order remains deterministic (phase + topological constraints) so
   node-class additions do not introduce ambiguous scheduling.

## Consequences

- Reservoir modeling can be added as a bounded node-adapter extension rather
  than a full engine redesign.
- Future node classes can be introduced under a contract-first schema policy.
- Network evolution becomes explicit and reviewable through contract/version
  changes instead of implicit parser behavior.
- Failure posture remains strict and predictable for operators.
