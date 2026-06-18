# PERFIDX05 Disposition

Status: executed with residual blocker.

Ran:
- Full identity anchor passed against PERFIDX04 outputs.
- Failure-path coverage passed in focused and workspace gates.
- Determinism evidence passed.
- Rust gates passed.

Not closed as speed-positive:
- Final timing regressed by roughly 4.4-8.6% on OFE2-OFE5 and 5.3-5.8% on H2637.
- PERFIDX05 should not be used to claim a performance win or the Stage-6 `<=10x` verdict.

Residual blocker:
- Decomposition overflow prefix scan in `07_decomposition_equations.rs` remains unmigrated.
  It needs a separate, proof-backed id-set/range design because an unsafe prefix-to-range
  conversion can silently weaken a validation guard.

Final posture:
- Behavior-preserving partial Stage-5 migration is ready for review.
- Performance and residual-prefix work should continue in a follow-on package.

## Post-review closure (2026-06-18, operator-approved — option A)

HELD, not landed. Independent review confirmed the regression is **structural**, not just
incompleteness: the write/guard-side by-id migration must **dual-write** the
authoritative logical `BTreeMap` *and* the indexed mirror on the hot path, and that cost
exceeds the id-lookup saving (Review-B concurs). The one scan whose removal could have
paid — the decomposition overflow scan — is blocked by the prefix→range interloper-proof
the package deliberately did not force (correct call; the hard stop worked).

Decision: **discard all PERFIDX05 working-tree code** (the failure-path tests test the
regressing wiring, so this is discard-or-keep-whole, like PERFIDX03); `crates/` returns to
the PERFIDX04 state; the committed record is docs-only. The program **pivots to
`PERFIDX06` re-measure** to obtain the actual ≤10× verdict on the PERFIDX04 endpoint
before any further write-side investment. If PERFIDX06 shows we still need more, the next
lever is a deliberate redesign choice (decomposition-scan-with-proof *iff* it beats
dual-write, or indexed-authoritative without the PERFIDX03 export seam) — **not** "finish
Stage 5 as specified," whose premise this evidence undercuts.
