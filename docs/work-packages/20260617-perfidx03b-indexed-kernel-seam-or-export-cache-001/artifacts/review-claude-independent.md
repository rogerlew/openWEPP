# PERFIDX03B — Independent Review (Claude Code)

Verdict: **COMPLETE — ready to land.** The PERFIDX03 export-cost blocker is closed,
outputs are bit-identical, the irrigation scope-creep did **not** return, and the one
yellow flag (`pass.parquet` byte difference) is a **pre-existing false alarm** I
disproved with my own runs.

Evidence mode: **Static** (full diff + seam mechanism) + **Ran** (my own `pass.parquet`
forensics + focused tests). The full-workspace gates (fmt/clippy/`test --workspace`/deny)
are **Codex's runs** (reproduced in verification-a/b), not re-run by me.

## What PERFIDX03B actually did

Eliminated the per-lane/day **clone** via `std::mem::take` move semantics
(`take_execution_input()`): the logical `HillslopeWritebackSurface` is *moved* into
execution instead of cloned, then refilled from the report; the indexed mirror is
refreshed after writeback as PERFIDX04 groundwork. It did **not** migrate the read
seam — the kernel still reads `BoundarySymbol` maps (Stage 4, future). So this
realizes the *clone* win only, which is why the net is modest.

## Verified

1. **Irrigation: clean.** `git diff crates/` contains **zero** `irrig` matches. The
   PERFIDX03 parse→seed scope-creep was not reintroduced. ✓
2. **Bit-identity: confirmed, and the `pass.parquet` flag is a false alarm.**
   - `H1.hbp` (authoritative trajectory), `loss.json`, `wat.parquet`, `plot.parquet`
     are byte-identical baseline-vs-current.
   - `pass.parquet` bytes differ — but **the same baseline binary produces 3 distinct
     `pass.parquet` hashes across 3 identical runs** (current binary likewise), while
     `wat.parquet` is byte-stable. So `pass.parquet` has **inherent parquet-container
     non-determinism**, independent of this change.
   - Decoded to CSV **in file order** (`SET threads=1`), baseline-bitid vs
     current-bitid are **identical** — 2192 rows, same values, **same order**. The
     difference is purely parquet container metadata. PERFIDX03B's `pass.parquet`
     is logically bit-identical *including row order*; Codex's `EXCEPT ALL` row gate
     was correct (and conservative — order matches too). ✓
   - The **full anchor ran and passed** (H2637 both UI variants @ ~874 s + the 1–5
     OFE ladder) — a real improvement over PERFIDX03, where the full anchor was
     skipped after the speed gate failed first. ✓
3. **Perf: regression closed, honestly modest.** OFE5 27.01→**38.34 s** (PERFIDX03
   regression) → **25.45 s** vs 26.82 s baseline (−5.1%). The clone is gone. The net
   is small because (a) the read seam still resolves `BoundarySymbol` and (b) the
   mirror-refresh is carried now as PERFIDX04 groundwork (overhead that eats into the
   clone win). The ≤10× target still awaits Stage 4 (resolve-once reads) + the Stage-6
   re-measure — correctly *not* claimed here. ✓
4. **Fail-closed on error: safe.** The move empties persistent lane state during
   execution, but the caller `?`-propagates day-level errors
   (`00_runner_intake_and_lane_setup.rs:1800`) → run-fatal, so the emptied state is
   never reused. The clone-based `execute_persistent_ofe_sequence_day_with_kernel` is
   retained (test-covered) for recovery contexts. ✓
5. **Kernel-contract change is legitimate.** `IndexedSurface::from_btreemap` reworked
   as a two-pointer **merge-walk** over the sorted BTreeMap and sorted registry
   (the ADR-0022 sorted-`SymbolId` invariant), dropping the per-entry lookup + final
   sort while preserving fail-closed unknown-symbol behavior. ✓
6. **Registry: a real gap was caught.** Review-A flagged (High) that the frozen
   registry missed valid first-day multi-OFE frost fine-layer symbols
   (`frost.runtime_fgfrst_0002_0017`); fixed with a bounded `MAX_FROST_FINE_CONTROL_COUNT
   * layer_count` reserve. Without this the production-active registry would fail-closed
   (crash) on a legitimate run — worth having surfaced. ✓
7. **My focused-test runs:** `-p openwepp-kernel-contract indexed` (3 passed);
   `perfidx03b_persistent_state_refreshes_indexed_writeback_surface` (1 passed). ✓

## Residual / forward notes (not blockers)

- **Mirror overhead now, benefit later.** The indexed mirror is refreshed every
  lane/day but not yet *read* by the kernel. PERFIDX04 must make the read seam consume
  it; until then it is groundwork cost the −5.1% is paid against. The follow-on must
  not reintroduce a full-map export at the seam (the PERFIDX03 trap).
- **Pre-existing `pass.parquet` container non-determinism** (data is reproducible; the
  parquet bytes are not). Out of scope here. Only worth a backlog note if byte-level
  parquet reproducibility ever becomes a requirement — it currently is not, and the
  decoded data is fully deterministic.

## Disposition

Land it. PERFIDX04 (resolve-once read-seam migration) is the next perf step.
