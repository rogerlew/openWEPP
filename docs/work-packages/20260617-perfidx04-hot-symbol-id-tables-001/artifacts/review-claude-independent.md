# PERFIDX04 — Independent Review (Claude Code)

Verdict: **COMPLETE — ready to land.** The strongest package in the perf program:
H2637 −24–25%, bit-identical, irrigation clean, profiler evidence genuine. The three
things I said I'd watch (anchor held / profiler real / irrigation out) all check out,
and I reproduced the bit-identity myself.

Evidence mode: **Static** (full +2335/−371 diff + design) + **Ran** (my own OFE2
bit-identity re-run + both crates' focused tests). Full anchor (OFE1–5 + H2637 both UI),
full-workspace gates, profiler capture, and wall-clock timings are **Codex's runs**
(reproduced in verification-a/b), not re-run by me.

## The architecture (answers PERFIDX04's central design question correctly)

Codex chose: **carry the indexed mirror beside the logical `BTreeMap` surface** in the
execution input, pass both into the kernel request, and **dual-apply** each accepted
writeback (and same-day OFE transfer mutation) to *both* surfaces via in-place mutation
helpers — rebuilding the mirror only once per day after seed/climate/PL prep. Hot reads
go through the mirror by `SymbolId`; the logical surface stays the commit/compatibility
authority (mirror is a **non-authoritative read shadow**). Crucially this **does not
reintroduce a per-lane/day full-`BTreeMap` export** — the PERFIDX03 trap. Confirmed:
no `from_btreemap`/`export_btreemap` on the scheduler hot path; the only clone is a
cheap sparse-`Vec` mirror clone.

## Verified

1. **Irrigation: clean.** Zero `irrig` in the entire crates diff; Review-A confirms no
   irrigation roots in the hot tables. The carve-out held. ✓
2. **Bit-identity — I reproduced it.** I re-ran **OFE2** (smallest case that exercises
   the same-day OFE *transfer* mirror-sync path, which OFE1 cannot) with both binaries
   (SHAs match Codex's report: baseline `9a66ff3f`, current `82c6cac7`). Manifest-recorded
   output hashes: **`H1.hbp`, `loss.json`, `wat.parquet`, `plot.parquet` byte-equal**;
   `pass.parquet` differs (the pre-existing parquet-container churn I characterized in
   PERFIDX03B — logical rows equal). Matches Codex's full-anchor TSV (all cases
   `byte_equal=1` for HBP/loss/wat/plot, pass rows 0/0). ✓
3. **Profiler / Stage-4 gate is genuine** (not just wall-clock). Direct hot `hourly_symbol`
   `format!` down to **0.01% self**; hot paths now show `hourly_symbol_for_request`,
   `frost_fine_layer_symbol_for_request`, `HotSymbolTables::state_grid_symbol`,
   `require_integral_pl_dispatch_symbol_ref_in_range` (id-table lookups) instead of
   per-access `format!`. Honest residual note: remaining `format_inner` is cold/logical
   export + Stage-5 writeback/guard territory, not the migrated hot families. ✓
4. **Determinism preserved.** Scheduler phase / OFE-lane / writeback accept-reject-apply
   order unchanged; the mirror is a read shadow, the logical surface remains authoritative
   for commit. Bit-identity across 7 cases is the proof the by-id reads equal the
   by-`BoundarySymbol` reads. ✓
5. **Realized win, honestly reported.** OFE1 **−4.36%** (id-table setup not repaid on a
   trivial single-OFE run — amortization), then OFE2–5 **+11.7–15.9%**, H2637 no-UI
   **+24.26%**, H2637 UI **+25.17%**. The win scales with OFE-days, which is exactly the
   high-OFE far-point target. ✓
6. **Scope guards intact.** No `SC-*` change; no `BoundarySymbol` API removal (logical
   names preserved in errors; fail-closed unknown-symbol intact); no writeback payload
   shape change. Line counts under the 3000 hard threshold (core_types 2671, scheduler
   2452, state_access 2219). ✓
7. **My focused-test runs:** `-p openwepp-kernel-contract` (23 passed);
   `-p openwepp-hillslope-orchestrator` (156 passed). ✓

## Minor / forward (non-blocking)

- **OFE1 −4.36%.** Setup overhead unamortized on a single-OFE run. Small (≈0.23 s) and
  partly within run-to-run noise. Single-OFE hillslopes are common in real watersheds,
  so if a later stage wants it back, a lazy/threshold build of the hot tables for tiny
  OFE counts is a candidate — not a blocker, and irrelevant to the high-OFE target.
- **5 `#[allow(dead_code)]`** mark retained *logical-context* PL-dispatch wrappers
  (`*_for_context(PlDispatchContext::logical(...))`) that the indexed hot path
  superseded. Inert (cannot affect behavior); mild dead-weight. Stage-5 should either
  wire them as a real logical fallback / test oracle or remove them.
- Clippy allowances added are narrow and justified (arg-count mirroring existing request
  fields, `too_many_lines` on table-builders/invariant checks, owned-symbol error API).

## Disposition

Land it. Next perf step is **`PERFIDX05`** (writeback + guards by `SymbolId`), then
**`PERFIDX06`** re-measures the actual legacy ratio and decides the ≤10× verdict.
