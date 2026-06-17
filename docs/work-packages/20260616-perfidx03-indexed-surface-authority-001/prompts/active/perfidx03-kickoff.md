# PERFIDX03 Kickoff — Indexed-Surface Authority Flip (Stage 3)

Execution mode: package-end-to-end (behavior-preserving authority flip — high-risk).

Autonomy: execute end-to-end (pre-flip diverse-management gate → flip → load-bearing
bit-identity → determinism → realized-speedup measurement → gates) without asking
for direction on intermediate steps.

## This is the flip — bit-identity is now load-bearing

Make the sparse indexed store (PERFIDX02's `IndexedWritebackSurface`) **authoritative**
for the per-OFE runtime surface, behind the `BoundarySymbol` compatibility layer,
keeping the kernel writeback payload shape and **bit-identical outputs**. The clone
win materializes here (lane clones become sparse-Vec clones). The shadow proved
equality on a cohort; the flip makes that equality load-bearing, so any live
divergence is real.

## Two hard stops (non-negotiable)

1. **Do not flip without the pre-flip reachable-registry proof.** The registry is now
   production-active and a miss is fail-closed (a crash). Prove the tightened
   reachable enumeration (`ncut`/`ncycle` + `.unwrap_or(0)`) yields **0 post-freeze
   unknowns across a *diverse* config cohort** — grazing, multiple cuts/cycles,
   irrigation, varied soil-layer/crop-rotation — not just H2637 + the ladder. Fix the
   enumeration first if a gap appears.
2. **Stop + diagnose on any bit-identity divergence.** Do not weaken the anchor gate.

## Steps

1. **Pre-flip gate** — diverse-management completeness audit → 0 post-freeze unknowns.
2. **Flip authority** — swap the surface backing to the sparse store behind the
   `BoundarySymbol` compatibility accessors (`get`/`insert`/`iter`/export; insert
   maintains sorted-`SymbolId` order; lookup binary search). Lane clones → sparse
   clones. Writeback payload shape unchanged; `apply_kernel_writeback` applied-symbol
   order preserved via sorted-`SymbolId` order.
3. **Bit-identity (HARD)** — `anchor_mismatches = 0` on H2637 both `wepp_ui` variants
   + the 1–5-OFE ladder vs a pre-flip baseline (the PERFOPT01/PERFIDX02 anchor
   method). Any mismatch → STOP + diagnose.
4. **Determinism** — sorted-`SymbolId` order live; pinned-seed reproducible; no
   FP/phase/OFE reorder.
5. **Realized speedup** — H2637 before/after wall-clock + RSS. Report net + the
   clone-vs-lookup split where possible.
6. **Gates** — fmt; clippy `-D warnings`; `test --workspace`; deny; line-count.

## Set expectations (do not over-claim the speedup)

Stage 3 realizes the **clone** win but **not** the lookup/`format!` win — hot lookups
still resolve `BoundarySymbol`→`SymbolId` per access via the compatibility layer
(extra indirection vs the old direct BTreeMap lookup), until Stage 4 (resolve-once).
**Net wall-clock may be modest or near-neutral** here; that is acceptable if
bit-identity holds and the clone cost is demonstrably gone. The full ≤10× awaits
Stages 4–6 and the Stage-6 re-measure. Report the net honestly.

## Hard constraints

- No `BoundarySymbol` API change; no `SC-*` change; writeback payload shape held.
- Bit-identical outputs (load-bearing) + determinism (`docs/numerics/`).
- Truthfulness: bit-identity, completeness, and timing are empirical — label `Ran:`;
  do not assert the speedup.

## Required reading

- `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/package.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- PERFIDX02 `artifacts/{perfidx02-clone-economics-measurement,perfidx02-shadow-equality-evidence,review-claude-independent}.md`
  + the `IndexedSurface`/`IndexedWritebackSurface` types.
- PERFIDX01 `artifacts/review-claude-independent.md` (the registry-coverage caveat);
  PERFARCH01 `staged-implementation-plan.md`; PERFOPT01 disposition (anchor method).
- `AGENTS.md`, `docs/codex_exec_plans.md`, `docs/numerics/README.md`,
  `docs/standards/rust-scientific-coding-standard.md`.
- The surface/writeback code in `package.md` Dependencies.
