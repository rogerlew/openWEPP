# PERFIDX01 — Storage-Representation Analysis (input to the ADR-0022 amendment)

Status: design analysis 2026-06-16 (Claude Code, architecture guidance)
Evidence mode: **Ran** (PERFIDX01 completeness audit) + **Static** (surface/clone
data-flow trace)

## Why this exists

PERFIDX01's completeness audit reported, for H2637: **registry = 1,699,798
symbols, constructed = 3,616, unknown = 0**, with RSS ~doubling (228→427 MB).
ADR-0022 specifies a **dense `Vec<Option<BoundaryValue>>` indexed by the global
`SymbolId`**, justified by "~6K symbols ⇒ clone = cheap memcpy." At 1.7M that
justification needs re-examination before Stage 2 builds the indexed store. This
analysis grounds the fix in the actual surface/clone structure.

## What the structure actually is (Static trace)

1. **The per-phase-cloned surface is small.** `HillslopeWritebackSurface`
   (`scheduler.rs:248`) is cloned per OFE per day (`scheduler_seed_and_runtime.rs:2125`),
   per phase (`scheduler.rs:772`), and post-day (`:2035`). One such surface holds
   **~hundreds to low-thousands** of present entries at a given OFE-day — the
   per-OFE *state* families (WB18/19 layer state ~9×nsl, frost layer/fine, MOFE
   hourly ~96, PL dispatch, ~50–100 static WB scalars), **not** 1.7M.
2. **Climate forcing is per-day reloaded, not accumulated.** `timem_*`/`intsty_*`
   are built per day for that day's breakpoints (`core_types.rs:418` — indices
   `1..=point_count`, `point_count ≤ 1500`, **reused each day**), inserted once
   per day (`03_climate.rs:160`), and the prior day's are **explicitly removed**
   before the next (`scheduler_seed_and_runtime.rs:2126`). So forcing is bounded
   (~tens–hundreds present) and is not the per-phase clone driver.
3. **The 1.7M is registry *capacity*, not a live surface.** The audit enumerates
   every bounded indexed family to its **maximum** (max breakpoint count; PL
   decomp slots×crops×roots×`1..366`; frost layer×fine×roots; erod
   particle×segment; layer-pairs `O(layer²)`; per-OFE multiplication). It is a
   deliberate worst-case over-enumeration to prove "0 unknowns" against the whole
   bounded universe. Only ~3.6K are constructed at runtime for this config.
4. **The clone cost is the deep-copy of Strings, not the symbol count.** PERFHO01
   localized it to `BTreeMap::clone_subtree` deep-copying the present
   `(BoundarySymbol=String, BoundaryValue)` pairs.

## The precise risk

Only one ADR-0022 detail is contradicted: **a dense `Vec<Option>` indexed by the
global `SymbolId` (0..1.7M)** would be a ~40 MB, mostly-`None` `Vec`, cloned
~14×/day/OFE — *slower and larger* than the small BTreeMap it replaces. Lookups
(O(1) array index) are unaffected at any size. So the fix is to **size the store
to the working set, not the global id space.** The architecture (registry,
sorted global ids, O(1) lookup, the ~100× clone win) is intact.

## Options

| Option | What | Pro | Con |
|---|---|---|---|
| **A. Compact local ids for the cloned store** | The dense/array store is indexed by a compact id sized to the per-OFE *state* reachable set; the global `SymbolId` stays the logical/sort/export key via a small map | Clone ∝ working set; O(1) lookup; keeps sorted-global-id order | Need a stable local↔global map; reachable set still includes PL-decomp/frost-fine families |
| **B. Sparse store** (`Vec<(id, value)>` sorted by id) | Store only present symbols | Sized to *present* (~hundreds), memcpy clone, no Strings, naturally ordered | O(log n) lookup (fine at ~hundreds), not O(1) |
| **C. Partition by clone-frequency** | Keep read-mostly forcing out of the per-phase-cloned surface | Shrinks the clone target | Forcing is already per-day-reloaded, so partial benefit only |
| **D. Tighten registry to the reachable set** | Production registry registers reachable (not worst-case) symbols | Smaller global universe; better RSS | Some families (PL-decomp `1..366`) must still register to a bound |

## Recommendation

**Reject the dense-global-`SymbolId` store. Size the cloned-surface store to its
working set** — concretely, **B (sparse, sorted `Vec<(SymbolId, BoundaryValue)>`)
as the primary candidate for the cloned surface**, because the cloned surface's
present set (~hundreds) is a small fraction of its reachable id space (PL-decomp +
frost-fine make the reachable set large), and sparse is sized to *present* with
memcpy clone, no Strings, and naturally sorted-id order (which directly preserves
`apply_kernel_writeback` ordering). Keep the **global sorted `SymbolRegistry`** for
id assignment, completeness/fail-closed, and exports (unchanged). Adopt **D**
(production registry registers the reachable, not worst-case, universe; the audit's
1.7M was a validation posture). **C** is largely already true.

A is a viable alternative if O(1) lookup proves necessary on a path that sparse
can't serve cheaply; the Stage-2 prototype decides A vs B by measurement.

## What Stage 2's prototype must measure (before any authority flip)

At **real H2637 scale**, not the 6K microbench:
1. The **actual per-OFE-day present count** (the cloned-surface size) and the
   reachable per-OFE-state set.
2. **Clone time**: sparse `Vec<(id,val)>` (and/or compact-dense) vs the current
   BTreeMap, at that present count — confirm the clone stays a win.
3. **Lookup time** for the chosen representation on the hot families.
4. **RSS** with the production (reachable) registry, not the over-enumerated one.

If a representation sized to the working set does not keep the clone a win at H2637
scale, that is the real go/no-go — surface it before Stage 2 flips authority.
