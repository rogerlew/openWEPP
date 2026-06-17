# PERFIDX04 Kickoff — Resolve-Once Hot Families (Stage 4)

Execution mode: package-end-to-end (behavior-preserving optimization; bit-identity is
load-bearing).

Autonomy: execute end-to-end (hot-family inventory → per-family incremental migration →
bit-identity → determinism → profiler evidence → realized speedup → gates → dual
review/verification → disposition) without asking for direction on intermediate steps.
Stop only on a declared per-family blocker.

## The lever

PERFIDX03B eliminated the per-lane/day **clone**. PERFIDX04 eliminates the other half:
the per-access **`format!`-built `BoundarySymbol` + `BTreeMap` lookup** on the hot read
paths. Resolve each hot family's `SymbolId` **once** and read via **id-table lookup**
(`IndexedSurface::get(id)` / `entries()` over the indexed rep), dropping the dynamic
String build + map walk from the named hot paths.

This is where the lookup win is supposed to materialize. **Do not assert the ≤10×
verdict** — that is Stage 6 (`PERFIDX06`). Report the realized speedup honestly.

## The central design question (decide this first, it's yours)

PERFIDX03B's mirror is on the **persistent lane state**, refreshed **after** writeback.
Hot reads happen **during execution**, on the `HillslopeWritebackSurface` that
`take_execution_input()` **moves** into the scheduler — *not* the mirror. So the hot
paths do not trivially "read the mirror." You must decide how the hot read paths get an
id-indexed view at execution time (resolve-once + index the execution surface; carry an
indexed execution surface; or another option). The only constraints: **no reintroduced
per-lane/day full-`BTreeMap` export** (the PERFIDX03 trap — see the PERFIDX03B handoff),
and **no writeback payload shape change**.

## Hard stops (non-negotiable)

1. **Irrigation is OUT.** The PERFARCH01 Stage-4 list named irrigation, but it is
   deferred/unwired/inert (`docs/backlog/20260617-irrigation-management-gated-activation.md`).
   No irrigation symbols exist in any production surface. Do **not** pre-resolve, wire,
   or activate irrigation. (This is the PERFIDX03 scope-creep that was already reverted
   once — do not let coverage logic pull it back in.)
2. **Bit-identity is load-bearing.** value-by-id must equal value-by-`BoundarySymbol`.
   `anchor_mismatches = 0` on H2637 both `wepp_ui` variants + the OFE1-OFE5 ladder vs a
   pre-PERFIDX04 baseline (HBP/loss/wat/plot byte-identical; `pass.parquet` row-equal —
   its container bytes are known to churn, that is not a failure). Any mismatch → STOP +
   diagnose the offending family.
3. **Logical names preserved in errors;** fail-closed unknown-symbol behavior intact.

## Steps

1. **Inventory** — from PERFHO02 profiler evidence, enumerate hot symbols per family
   (climate, frost incl. fine-layer, WB18/WB19, PL, MOFE hourly) and their `format!` +
   map-lookup call sites. Record in `perfidx04-hot-family-inventory.md`. Irrigation
   excluded and noted.
2. **Id-table design** — the execution-time id-indexed read approach (the central
   design question above). Record in `perfidx04-id-table-design.md`.
3. **Per-family increments** — migrate one family at a time; bit-identity check per
   increment so a divergence localizes to one family. Resolve ids once; replace hot
   `format!`+lookup with id-table lookup; keep `BoundarySymbol` for errors/cold paths.
4. **Bit-identity (HARD)** — full anchor vs pre-PERFIDX04 baseline. Any mismatch → STOP.
5. **Determinism** — `docs/numerics/`: no FP-reduction reorder, no per-OFE sequencing
   change, pinned-seed reproducible.
6. **Profiler evidence** — `perf`/profiler showing dynamic symbol formatting removed
   from the named hot paths (the Stage-4-specific gate).
7. **Realized speedup** — H2637 + OFE1-OFE5 before/after wall-clock + RSS; net + the
   per-family contributions.
8. **Gates** — fmt; clippy `-D warnings`; `test --workspace`; deny; line-count.

## Set expectations (do not over-claim)

This stage should move the needle more than PERFIDX03B (the clone was already removed;
this removes the lookup/format!). But the total ≤10× verdict awaits Stage 6's
re-measure against legacy. Report net honestly; if a family resists bit-identical
migration, declare it and migrate the rest.

## Hard constraints

- No `SC-*` change; no `BoundarySymbol` public API removal; writeback payload shape held.
- No irrigation pre-resolution/wiring/activation.
- Bit-identical outputs (load-bearing) + determinism (`docs/numerics/`).
- Truthfulness: bit-identity, determinism, profiler, and timing are empirical — label
  `Ran:`; do not assert the speedup or the ≤10× verdict.

## Required reading

- `docs/work-packages/20260617-perfidx04-hot-symbol-id-tables-001/package.md`
- `docs/decisions/0022-indexed-runtime-surface-representation.md` (+ Amendment 1)
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/artifacts/staged-implementation-plan.md` (Stage 4)
- `docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/perfho02-profiler-evidence.md` (which paths are hot) + PERFOPT01 disposition (anchor method)
- `docs/work-packages/20260617-perfidx03b-indexed-kernel-seam-or-export-cache-001/artifacts/{perfidx03b-seam-design,perfidx03b-worker-handoff,review-claude-independent}.md`
- `docs/backlog/20260617-irrigation-management-gated-activation.md` (irrigation carve-out)
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`,
  `docs/standards/mechanical-refactor-authoring-guide.md`, `docs/numerics/README.md`
- The `IndexedSurface` / `IndexedWritebackSurface` read API in
  `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`.
