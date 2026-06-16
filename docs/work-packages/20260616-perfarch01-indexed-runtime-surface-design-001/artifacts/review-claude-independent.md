# PERFARCH01 — Independent Review (Claude Code)

Status: APPROVE (design sound; feasibility measured + independently reproduced)
Evidence mode: **Static** (design/ADR/plan/hazards) + **Ran** (independent
microbench re-run)

## What I checked

**Feasibility is measured, not asserted — and reproduces.** I rebuilt and re-ran
the prototype microbench myself (`rustc -O`): clone **103×**, lookup **219×**,
update **104×** (Codex: 110× / 219× / 116×) — same ballpark, run-to-run variance
only. The microbench is a **fair** model: it builds the real symbol families
(~6,396: static, climate `timem/intsty`, WB18/19 layers, frost-fine, PL slots,
MOFE transfer), and the `BTreeMap` baseline includes the `format!` symbol
construction the production code actually does per lookup.

**The sorted-id trick holds for the real patterns.** `sorted_id_order_matches_string_sort=true`
— and meaningfully so: the zero-padded (`{:04}`) dynamic symbols sort
lexicographically in numeric order, so id-in-sorted-order ≡ string-sort order,
which is what cheaply preserves `apply_kernel_writeback` ordering and deterministic
exports.

**The Amdahl reasoning is honest.** It uses a conservative **50–100×** primitive
range (not the peak 219×), and explicitly does **not** claim the 96.24%
scheduler-children share is all removable map/string overhead: ≤10× needs ~89–90%
of elapsed time genuinely migrated out of string mechanics; ≤5× needs ~95–96% and
is called **aspirational, not a storage-only promise.** Correct framing.

**ADR-0022 is well-scoped and correctly *Proposed*** (not auto-ratified): preserves
the `BoundarySymbol` logical interface, mandates per-stage bit-identity +
determinism, forbids FP/phase/OFE reorder and guard weakening, registers dynamic
families before freeze, **fails closed on unknown post-freeze symbols**, and
migrates prefix/membership guards to id-ranges with equivalence tests.

**The staged plan is safe.** Shadow (Stage 2) → round-trip equality → make
authoritative (Stage 3) → resolve-once hot families (Stage 4) → migrate the
order-dependent ops (Stage 5) → empirical target assessment (Stage 6). Each stage
is its own package gated on `anchor_mismatches = 0` + determinism + Rust gates.
This is the right way to land a pervasive change without big-bang risk.

## Caveats (non-blocking)

- The 219× lookup figure conflates `format!`-elimination with indexed lookup (the
  dense side uses pre-resolved ids — the optimistic resolve-once case). Lookups
  that can't be fully resolve-once will see less. The conservative 50–100× Amdahl
  range absorbs this; **Stage 6's measured per-stage share is the real test of ≤10×**,
  not the microbench.
- ≤10× rests on ~90% of elapsed time being removable string-keyed overhead.
  PERFHO02's 96% under the scheduler lifecycle is consistent with but does not
  *prove* that; the migrated share must be measured after each stage (the plan
  says so).

## Verdict

Approve. The architecture is sound, the feasibility is evidence-backed and
independently reproduced, and the migration is staged safely. **Recommend
ratifying ADR-0022**; Stage 1 (`PERFIDX01`) is gated on that ratification (the
plan's Stage 0). The no-independent-dual-review caveat that applied to PERFOPT01
is addressed here by this review.
