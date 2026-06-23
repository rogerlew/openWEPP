# Verification

Status: executed-held.

## Static

- Static: `SC-SED-001` has canonical EROD13 Wave-1, EROD14 Wave-2, and
  EROD15 HBP export authority, including required symbols, guard families, and
  Wave-3 export mappings.
- Static: `artifacts/operand-lineage.md` records direct source authority,
  units/basis, producer targets, publication consumers, and anti-alias
  candidates before production R7D6 code edits.
- Static: `SC-HYDRAULICS-001` WB16 peak-flow coupling readiness addendum
  requires finite/non-negative `peakro` and `watdur`, explicit
  `watdur = Q / peakro` continuity, branch metadata, and no fallback
  reconstruction when WB16 peak surfaces are missing or invalid.
- Static: R7D6 direct runtime code now has typed direct peak-duration and
  erosion inputs, direct compute, state mutation, downstream operands, and
  shadow projections before publication row construction.

## Ran

- Ran: `cargo fmt --check` passed.
- Ran: `git diff --check` passed.
- Ran: R7D trace-marker scan over `crates/` returned no matches.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r7d5_erosion_active_publication_fails_closed_without_direct_sediment_producer --lib`
  passed.
- Ran: `cargo test -p openwepp-hillslope-orchestrator
  r7d6_typed_erosion_producer_populates_publication_operands --lib` passed.
- Ran: `cargo test -p openwepp-runner direct_production --lib` passed after
  adding the direct WB16 peak-duration span and counter updates.
- Ran: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  passed.
- Ran: H2637 direct production
  `/tmp/r7d4-h2637-5day` with `--direct-production-executor` progressed
  through these blockers:
  `erod14.class_count` domain failure, missing producer-authoritative
  `runoff.peak_runoff_m3_s`/`erosion.peak_runoff_m3_s`, missing runtime
  `peakro`, and direct WB16 `peak_runoff.remax` branch mismatch.
- Ran: H2637 direct production label `r7d6-zero-lddend` exited `0`:
  `direct elapsed=1.19 rss_kb=63708`.
- Ran: fresh H2637 compatibility label `r7d6-compat-current` exited `0`:
  `compat elapsed=0.65 rss_kb=50740`.
- Ran: H2637 output comparison:
  `cmp` reports WAT byte identity (`wat_cmp=0`), PASS mismatch
  (`pass_cmp=1`), and HBP mismatch (`hbp_cmp=1`).
- Ran: `pyarrow` PASS comparison found the only remaining PASS field
  mismatch is `peakro` across six rows. Direct values are
  `3.591689245524811e-06`, `4.837293745180717e-07`,
  `9.939800459642262e-07`, `4.726157673358129e-07`, `3.63e-08`, and
  `3.63e-08`; compatibility values are all `0.0`.

## Not Run

- Full workspace `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` were not run because R7D6
  is held at
  `HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL` and the dirty tree
  already exposes broader non-R7D6 direct-runtime failures under broad filters.
