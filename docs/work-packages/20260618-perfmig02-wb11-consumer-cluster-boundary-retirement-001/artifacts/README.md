# Artifacts

Status: executed 2026-06-18.

Required deliverables:

- `perfmig02-reader-map.md` - the WB11-output reader set; which of the 543+8 symbols each phase reads;
  the materialization boundary to retire.
- `perfmig02-migration.md` - what was flipped to dense `SymbolId` reads; which materialization was
  dropped/moved.
- `perfmig02-bit-identity.md` - PERFMIG01 543+8 fixture + H2637 `.hbp`/`wat`/`pass` identity still
  exact.
- `perfmig02-endpoint-timing.md` - H2637 seconds + ratio + RSS vs PERFMIG01 (669.97 s) / PERFIDX06;
  **retired-boundary attribution** (the `apply_indexed` cost for migrated symbols measured to drop).
- `perfmig02-logical-free-proof.md` - no dual-read for migrated symbols; the WB11 materialization dropped/moved.
- `perfmig02_disposition.md` - CONTINUE + next cluster / REDIRECT + the deep single-phase array-native pivot.

Additional closure evidence:

- `perfmig02-gate-results.md` - gate table and focused command evidence.
- `perfmig02-line-count-governance.md` - `.rs` line-count disposition.
- `perfmig02-review-verification.md` - local review/verification passes and finding disposition.
- `perfmig02-transition-boundary-bench.tsv` plus `perfmig02-transition-boundary-bench/` - artifact-local
  retired-materialization bench source and result.
