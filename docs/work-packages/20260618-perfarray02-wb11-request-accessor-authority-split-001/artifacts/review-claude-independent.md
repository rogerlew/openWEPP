# PERFARRAY02 — Independent Review (Claude Code)

Verdict: **Sound, honest NO-GO — and it is the decisive answer to the whole perf program.**
The seam landed, bit-identity passed, the structural proofs are genuinely perf-demonstrated,
and the floor was measured on a *clean* array-native path. The number settles it:
array-native WB11 runoff is **817.8 µs/OFE-day = 21.16× legacy**, over **2.1×** the ≤10×
budget (386 µs) and **4.2×** the 5× budget (193 µs). **≤10× is not reachable by the
array-authoritative migration, and 5× is off the table.**

Evidence mode: **Static** (all artifacts + the cited seam code + the timing breakdown).

## The measurement is trustworthy (I checked the proofs)

- **Structural proofs are perf-demonstrated, not asserted.** Proof 1: the pilot seeds
  `ArrayHotState`, passes *empty* logical maps + `Some(&hot_state)`, and the export is *after*
  apply (`scheduler.rs:1623/1665/1878`), outside the kernel run — `from_btreemap_surfaces`
  (8.02%) / `export_btreemap_surfaces` (1.17%) are confirmed boundary, not seam. Proof 2: the
  array path is the mutually-exclusive branch from logical apply + mirror sync
  (`scheduler.rs:1724/1856/1923/1946`). So the array-native segment has **no per-day export
  and no dual-write** — the 817.8 µs is uncontaminated. ✓
- **Bit-identity passed** (OFE5 + H2637); default wall-clock 671.88 s ≈ PERFIDX06's 666.82 s
  (flag-off path unchanged). The seam is correct; the measurement is on a real array path. ✓

## Why it's still 21× — the diagnosis matters

This is *not* "the symbol machinery wasn't removed." The array-native segment breaks down as:
kernel run ~481 µs + logical→array payload conversion ~325 µs (`from_logical_payload` via
`resolve_logical_fields`) + evaluate/apply ~12 µs. Two findings:

1. **The kernel still produces a *logical* writeback payload** and converts it — the pilot
   migrated the kernel *input* (array reads) but not the kernel *output*. That conversion is
   ~325 µs/OFE-day of pure transitional cost.
2. **Even the kernel run alone (~481 µs) exceeds the entire ≤10× budget (386 µs)** — and that
   is *one phase*. The cost that remains after removing the symbol-keyed machinery is the
   kernel's own computation + guards + its logical output construction, distributed work that
   the array representation does not touch.

So the disposition's read is correct: the request/read seam alone can't reach the budget, and
even Stage-C removal of the 1685 µs boundary conversion would leave the array-native segment at
~21× legacy. The only remaining lever is **array-native kernel *output*** — rewriting the
physics phases' writeback production to avoid logical payload construction inside the kernel —
which is far more invasive (touches the kernel internals, not just the seam) and is *unproven*
to reach the budget. The recommendation to **stop the broad migration** is honest.

## The bottom line for the program

The perf arc ran its course honestly: 73× → read-side wins (PERFIDX04, −24%) → write-side
ceiling (PERFIDX05) → endpoint measured (PERFIDX06, 73.12×) → array-authoritative scoped and
prototyped (PERFARCH02, ~50× on the surface) → **integrated floor measured (PERFARRAY02, 21×
array-native for runoff alone).** The conclusion is evidence-backed, not a guess: **the
symbol-keyed-representation cost was real and removable, but removing it does not reach ≤10×
because the residual is the kernel's own per-OFE-day computation + logical output, which the
array migration doesn't cheapen.** ≤10× / 5× is not reachable by the refactors available
without a deeper, unproven kernel-output rewrite the program has chosen not to chase.

## Disposition + code recommendation

NO-GO is correct; do **not** ratify ADR-0023. Land this record. The PERFARRAY02 pilot code is
**invasive flag-gated plumbing for an abandoned approach** (~408 lines in `scheduler.rs`, a
new timing module, state_access/runoff edits) — like PERFIDX05, it should be **discarded** (the
measurement + learning live in these artifacts). I also recommend **reverting the now-purposeless
Stage A `ArrayHotState` shell** (committed `3eea90b1`, inert, no consumer) so the closed arc
leaves **zero array-authoritative dead code** in production — unless a future "array-native
kernel output" exploration is anticipated, in which case the inert shell may be kept as a
starting point. Operator's call.

## Decision taken (2026-06-18, operator: A)

Option A. PERFARRAY02 pilot code discarded **and** the Stage A `ArrayHotState` shell reverted;
production carries zero array-authoritative code (`cargo check --workspace` clean,
kernel-contract 23 tests pass). The perf arc is closed at the 73.12× endpoint; ADR-0023 not
ratified; conclusion recorded to agent memory. The committed record is docs-only.
