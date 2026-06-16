# ADR-0022: Indexed runtime-surface representation

**Status:** Proposed
**Date:** 2026-06-16 UTC
**Deciders:** Roger Lew (pending ratification), Codex (draft)
**Builds on:** [ADR-0003](0003-parity-semantic-not-bit.md), [ADR-0011](0011-architecture-first-top-down-science-contracts.md)
**Work package:** `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/`

## Context

The runtime surface currently uses
`BTreeMap<BoundarySymbol, BoundaryValue>`, where `BoundarySymbol` owns a heap
`String`. PERFHO01, PERFOPT01, and PERFHO02 show the high-OFE wall-clock gap is
CPU-bound in per-OFE-day symbol-surface mechanics, not output writing. PERFOPT01
removed some clone/detail overhead and improved H2637 from 978.55 s to 849.86 s,
but PERFHO02 still found hydrology symbol lookup, dynamic symbol formatting,
guard scans, and `apply_kernel_writeback` sort/insert overhead.

The current logical seam is valuable: diagnostics, writeback payloads, tests, and
contracts name `BoundarySymbol`s. The physical storage is the problem.

## Decision

Adopt a run-scoped indexed runtime-surface architecture:

- `BoundarySymbol` remains the logical public name.
- A frozen `SymbolRegistry` assigns dense `SymbolId`s for each run.
- `SymbolId`s are assigned in sorted `BoundarySymbol::as_str()` order.
- State and flux surfaces are stored as dense indexed arrays.
- Hot paths use pre-resolved `SymbolId`s or family id tables.
- Compatibility seams can still resolve and export by `BoundarySymbol`.
- No symbol is lazily interned after registry freeze; unknown post-freeze symbols
  fail closed.

The sorted-id rule is binding because it preserves current sorted string-order
effects cheaply, including writeback applied-symbol order and deterministic
exports.

## Consequences

- Staged implementation can remove hot `String` allocation, `memcmp`,
  `BTreeMap::clone_subtree`, and success-path writeback string sorting without
  changing logical symbols.
- Every code-bearing stage must preserve bit identity and within-config
  determinism; the storage change does not authorize FP reduction reorder, phase
  reorder, OFE reorder, guard weakening, or diagnostic masking.
- Dynamic families must be registered before freeze from parsed dimensions
  (climate points, soil layers, frost shape, PL slots/crops, irrigation events,
  MOFE hours, watershed nodes).
- Prefix and membership guards migrate to registry ranges or required-id sets,
  with tests proving identical accept/reject behavior.
- The <=10x performance target is plausible but not guaranteed by this ADR; it
  depends on migrating roughly 89-90% of current elapsed time out of
  string-keyed surface mechanics. <=5x remains aspirational until measured.
