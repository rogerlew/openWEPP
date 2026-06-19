# PERFMIG01 — Independent Review (Claude Code)

Verdict: **CONTINUE confirmed — the rung is correct, safe, identity-clean, and honestly
net-negative. Commit it.** But the endpoint win is still entirely ahead of us: this rung proved the
*pattern* is safe, not that the migration *pays off*. PERFMIG02 is the first real test of the thesis, and
its gate must be a **measured** endpoint improvement, not another projection.

Evidence mode: **Static** (read the disposition + all five artifacts + the production diff: writeback
guards, scheduler apply, the runoff builder) + **Ran** (none; relied on Codex's reported gate runs and
the recorded fixtures).

## Why CONTINUE is right and the rung is safe to land

- **Identity is airtight.** `.hbp` + `wat.parquet` byte-identical; `pass.parquet` Arrow-equal (pyarrow
  `table_equal True`); focused fixture exact on 543 state + 8 flux by `f64::to_bits()`. A
  representation flip that changes nothing in the outputs. ✓
- **The safety guards are real, not asserted.** `apply_indexed_kernel_writeback` resolves *all* ids into
  temp vectors first (each via `.ok_or(UnknownSymbolId)`) and mutates only after every id resolves — a
  genuine resolve-all-then-apply pattern, so an unknown id fails the whole apply **before** any mutation
  (the no-partial-mutation guard). `evaluate_indexed` preserves finite + domain failure classes;
  non-accept decisions fail closed. ✓
- **Logical-free on the migrated success path:** the warm-rain branch builds a single id-backed payload
  (`0` logical state/flux updates), no `from_logical_payload`, no full `BTreeMap` seam export — only the
  named, updated-ids materialization. The unmigrated snow/frost/irrigation/MOFE branches are explicit
  named boundaries with the existing missing-required-symbol guard, not a silent fallback. ✓
- **The +0.47% regression was predicted, in writing, by this package** and is inside its declared
  single-rung boundary-offset class. Gates green (fmt, check, clippy `-D warnings`, test, deny, markdown).

## The honest read of the number — what is actually happening

The migration **did not capture any physics win yet.** It swapped the *output representation* (logical
payload → dense payload) but the scheduler immediately **materializes dense back to logical**
(`apply_indexed` = 107.5 µs/payload) because every downstream phase still reads logical. That dense→logical
round-trip is **pure added overhead** — it costs *more* than the original all-logical writeback it
replaced (hence +3.15 s). The apply boundary (107.5 µs) ≈ PERFARCH03's 108 µs materialize ≈ the original
logical-payload work: the migration **relocated** the symbol resolution, it did not eliminate it.

This is not a flaw — it is the expected shape of a single writeback-only rung. But it has two sharp
consequences for the program that the disposition understates:

1. **The boundary retires only when downstream *readers* migrate to read dense directly.** A
   writeback-only rung *adds* an apply-boundary without removing any logical read, so it is **net-negative
   by construction.** If PERFMIG02 migrates more *writers* (more phases' writebacks) without migrating the
   *readers* of those outputs, boundaries **accumulate** and the endpoint keeps drifting up. The
   disposition's "contiguous WB11-**consumer** cluster" is the right instinct **only if "consumer" means
   the phases that *read* these 543 symbols** — migrate the readers so the materialization for those
   symbols is **dropped, not moved.**

2. **Even with boundaries retired, writeback-flip alone cannot reach the PERFARCH03 floor.** PERFARCH03's
   0.96 µs was a *fully* array-native branch (dense **read + compute + write**). PERFMIG01 migrated only
   the **write**; the kernel still reads logical inputs and computes over logical state — the bulk of the
   ~140 µs/branch machinery is untouched. Realizing the floor needs the **input-read + internal-compute**
   migration too (the PERFARRAY01/02 input-side lever), not just the output-side state-authority flip this
   rung began. The path to ≤10× is **both** levers: retire boundaries (reader migration) **and** migrate
   internal compute to dense.

## The gate PERFMIG02 must carry (sharper than "trajectory projection")

We have now spent one production rung and the endpoint moved **the wrong way** (73.12× → 73.46×) on faith
that the boundary retires later. That faith is well-founded (PERFARCH03 proved the floor exists), but it
cannot be spent indefinitely. PERFMIG02's acceptance gate must be a **measured net endpoint improvement**
— the apply-boundary cost for the migrated symbols must *actually drop* in the H2637 endpoint, proving
boundaries **retire** rather than accumulate. **Two consecutive net-negative rungs is the REDIRECT
signal**, not a third "trust the projection." A modest first rung was fine; a flat second rung would mean
the boundary-collapse mechanism isn't converting, and the attack needs rethinking (e.g. migrate a phase
fully array-native — read+compute+write — to capture the internal-compute win directly, accepting two
edge boundaries, rather than widening writeback-only).

## Disposition

CONTINUE — commit the rung (production diff + ADR-0023 + artifacts; bench `target/` git-ignored). It is a
correct, safe, identity-clean foundation. Scaffold **PERFMIG02** with the hardened gate above: migrate the
**readers** of the WB11 outputs to retire (not relocate) the apply-boundary, and make a **measured**
endpoint improvement the acceptance criterion. The destination is proven reachable (PERFARCH03); this rung
proved we can move toward it safely; the next must prove we are actually moving.
