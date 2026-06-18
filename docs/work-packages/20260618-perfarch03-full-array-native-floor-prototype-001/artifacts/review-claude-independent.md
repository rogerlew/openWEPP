# PERFARCH03 — Independent Review (Claude Code)

Verdict: **GO confirmed — sound, well-evidenced, honestly scoped.** This is the decisive
experiment the perf program quit one step short of, and it answers the viability question: the
fully array-native WB11 runoff branch is **0.96 µs/OFE-day** — **146× faster than the *real*
production kernel on the *same* branch** — proving the H2637 73× is overwhelmingly **symbol
machinery, not physics**. ≤10× (and very plausibly ≤5×) is reachable by the full
array-authoritative migration. The 21× was a half-measure floor; this is the real one.

Evidence mode: **Static** (read the harness `main.rs` + all artifacts) + **Ran** (extrapolation
arithmetic).

## Why I trust the number

- **Like-for-like against the real kernel.** The harness runs the *same* warm-rain runoff branch
  through both `Wb11HydrologyKernel::run` (the real production kernel — `main.rs:424`) at
  **140.8 µs/OFE-day** and an array-native copy at **0.96 µs** — a **146.8×** speedup with the
  physics held identical (`array_physics_only` was 0.075 µs). The comparison is to the production
  kernel, not to the PERFARRAY02 input-only 817 µs. ✓
- **Real physics, proven not a fake.** The array path is validated to **exact `to_bits()` identity
  on 543 state + 8 flux outputs** vs the production payload. A stripped prototype cannot match 543
  outputs bit-for-bit — so it is doing the *real* branch computation, just array-native. ✓
- **The hot loop is genuinely logical-free** (perf-verified): no `BTreeMap`, `memcmp`, `format!`,
  `BoundarySymbol` lookup, or `from_logical_payload` in `time_array_hot_loop` /
  `array_runoff_physics` / `OutputPlan::write_outputs`. ✓
- **Cache lever realized:** dense working set 18 KB, release RSS **~3 MB** — comparable to legacy's
  ~4.6 MB, vs openWEPP's 229 MB. The 50× cache-thrash collapses. ✓
- **The boundary cost is correctly isolated:** one-shot materialize 108 µs/OFE-day, flagged as
  transitional and "must not be repeated inside a production migrated hot path" — exactly the
  PERFARRAY02 trap, now a named migration constraint. ✓

## Honestly scoped — no over-claim

The disposition states plainly: "**This is a GO for the full array-authoritative migration program,
not a claim that production H2637 is already solved**," it does not treat the 21× as the floor, and
it requires real endpoint timing after each migrated rung. That is the discipline I failed earlier
(calling perf "closed") — applied correctly here.

## What it means (my extrapolation, flagged as such)

If phases scaled like this branch (146×), the full OFE-day would drop 2826 µs → ~19 µs ≈ **0.5×
legacy** — too optimistic to take literally (not every phase is pure machinery). But even at a
*fraction* of that — say 10–20× realized across all phases — the OFE-day lands ~140–280 µs =
**~3.6–7× legacy**, i.e. **≤10× with margin and ≤5× plausible**. The physics floor is decisively
below budget; the migration's job is to *realize* it, not discover whether it exists.

## Disposition

GO. Land the record (prototype source + artifacts; `target/` git-ignored — 301 MB). The next package
is the **first production array-authoritative migration rung — WB11 runoff** — reviving the ADR-0023
direction, carrying the PERFARCH03 constraints (no logical payload / no per-phase
`from_logical_payload` in hot loops; boundary materialize only at validation boundaries; exact
identity per branch; **H2637 endpoint timing + RSS after each rung**). This is the path to
viability, and it is now evidence-backed rather than hoped-for.
