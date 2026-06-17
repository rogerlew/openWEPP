# ADR-0022: Indexed runtime-surface representation

**Status:** Accepted
**Date:** 2026-06-16 UTC
**Deciders:** Roger Lew (ratified), Codex (draft), Claude Code (independent review — approve)
**Ratified:** 2026-06-16 by Roger Lew (operator authority); applied by Claude Code on operator instruction following independent review.
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
- State and flux surfaces are stored as indexed arrays **sized to the working
  set** — *not* a dense `Vec` over the global `SymbolId` space (refined by
  **Amendment 1**).
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


## Amendment 1 (2026-06-16): storage sized to the working set, not the global id space

**Status:** Accepted (operator-authorized; Claude Code architecture guidance from
the PERFIDX01 finding).

**Trigger.** PERFIDX01's completeness audit found the run-scoped registry is
**~1.7M symbols for H2637** — a deliberate worst-case over-enumeration of all
bounded indexed families; only ~3.6K materialize at runtime, and the per-phase-cloned
surface holds ~hundreds–low-thousands. Read as a dense `Vec<Option<BoundaryValue>>`
indexed by the **global** `SymbolId`, the Decision's "dense indexed arrays" would
size each cloned surface to ~1.7M slots (~40 MB, mostly `None`), cloned ~14×/day/OFE
— **larger and slower than the BTreeMap it replaces**, inverting the dominant clone
win. (Lookups are O(1) regardless of size and are unaffected.)

**Refinement.** The store backing the per-phase-cloned surface is sized to its
**working set**, not the global id space:

- Keyed by `SymbolId` but **sized to the symbols present/reachable in that surface**
  (~hundreds–low-thousands), via a **sparse sorted `Vec<(SymbolId, BoundaryValue)>`**
  (primary) or a **compact local-index dense array** (if a path needs O(1)) — chosen
  by the Stage-2 prototype's measurement at real H2637 scale. A dense `Vec` over the
  global `SymbolId` space is **rejected**.
- The **global sorted `SymbolRegistry` is unchanged** — id assignment (sorted-string
  order), completeness/fail-closed, `BoundarySymbol` export. The sparse store's
  natural `SymbolId` order **preserves the sorted-string order** that
  `apply_kernel_writeback` and deterministic exports rely on.
- The **production registry registers the reachable universe** (bounded by parsed
  dimensions), not the worst-case bound; PERFIDX01's 1.7M was a validation posture.
- Read-mostly climate forcing (`timem_*`/`intsty_*`) stays per-day cleared+reloaded
  (already the case), out of the persistently-cloned state.

**Gate (binding on Stage 2, before any authority flip).** A Stage-2 prototype must
**measure at H2637 scale** that the chosen working-set representation keeps the
**clone a win** vs the current BTreeMap at the real per-OFE-day present count, and
record RSS with the production (reachable) registry. If none keeps the clone a win,
that is the migration's go/no-go and must be surfaced before authority flips.
Analysis: `docs/work-packages/20260616-perfidx01-run-scoped-symbol-registry-001/artifacts/perfidx01-storage-representation-analysis.md`.

The rest of the Decision (sorted-id rule, fail-closed, per-stage bit-identity +
determinism, guard-to-id-range migration) is unchanged.
