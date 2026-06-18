# PERFIDX06 — Independent Review (Claude Code)

Verdict: **Sound assessment, correct disposition.** The measurement is honest and
apples-to-apples, the **73.12×** verdict is trustworthy, and the redesign recommendation is
right. PERFIDX06 closes the perf *measurement* arc; the next move is a redesign-scoping
package, not more id-table work.

Evidence mode: **Static** (read all six artifacts + the endpoint pin).

## Measurement integrity — the trap was handled

- **Apples-to-apples ratio.** The legacy baseline was **re-measured on the same machine and
  the same H2637 fixture** with the real pinned legacy binary (`wepp_260430_hill`, SHA
  `3b2fdd2b…`), 3 reps each (tight spread: no-UI 9.02–9.12 s). This is exactly the
  like-for-like discipline I flagged — not a stale cross-hardware figure. ✓
- **Endpoint pinned.** Measured binary SHA `82c6cac7` = the PERFIDX04 endpoint (sidecar
  source commit `e9c52a57`); working tree clean; no production Rust edits. The repo HEAD
  being later is just the PERFIDX05-discard + docs. ✓
- **No code/contract change** (assessment package). ✓

## The verdict is trustworthy

- **73.12× no-UI** (666.82 / 9.12), 57.84× with-UI. Consistent with the prior arithmetic
  (~56–75×). PERFIDX04 was real (978→667, −31.9%) but the gap is still an order of
  magnitude above ≤10×.
- **The profile is spread, not peaked** — scheduler/runtime-surface lifecycle dominates as a
  *class* (BTreeMap insert/remove, `__memcmp_sse2`, malloc/free, symbol-table access,
  writeback 17%, decomposition 12%+8%, residual formatting 9.5%), with **no single subtree
  large enough** to close even ≤10× by excision. That is the signature of an architectural
  cost, and it correctly rules out continued narrow id-table work.
- The disposition's framing of the constraint pair is exactly right: any redesign must
  solve **both** the PERFIDX03 export seam *and* the PERFIDX05 dual-write ceiling — the two
  failure modes the program already paid to discover.

## One thing the redesign-scoping package must add: measure the floor first

The disposition says ≤10× needs ~7.3× and 5× needs ~14.6×, and recommends an
array-authoritative hot-path redesign. I agree — but the open question it does **not**
answer (and shouldn't, it's assessment-only) is **openWEPP's intrinsic floor**: legacy does
this physics in ~9 s, but openWEPP legitimately does *more* (typed guards, conservation
gates, producer/consumer trajectory ownership, parquet + HBP output) that legacy does not.
The reachable target is set by how much of the 667 s is *incidental* (string keys, BTreeMap,
allocation, formatting — removable) vs *intrinsic* (the typed-contract work — the floor).
The scoping package's first job is a **minimal array-authoritative prototype that measures
that floor** before committing to a full migration — so we learn whether 5× is even
physically on the table, cheaply, before paying for the big refactor.

## Disposition

Land the PERFIDX06 record. Next perf step is the **redesign-scoping package** (array-authoritative
hot-path state), framed by the floor-measurement question above — not PERFIDX05B / more
write-side id work.
