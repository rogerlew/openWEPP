Run root: docs/work-packages/20260814-snow-free-land-surface-energy-runtime-shadow-001/artifacts/comparator-heavy/20260817T231432Z-child3-v5
Child ID: child3-20260817T231432Z
PASS: 7
FAIL/HOLD: 1

- bench_strict_projection: PASS (exit 0) — 1/1 tests selected+passed
- bench_one_open_tile: PASS (exit 0) — 1/1 tests selected+passed
- bench_single_rank_covered: PASS (exit 0) — 1/1 tests selected+passed
- bench_two_rank_covered: PASS (exit 0) — 1/1 tests selected+passed
- bench_mixed_open_covered: PASS (exit 0) — 1/1 tests selected+passed
- bench_complete_public_envelope: PASS (exit 0) — 9/9 tests selected+passed
- bench_actual_byte_rollback: PASS (exit 0) — 1/1 tests selected+passed
- gate_cargo_clippy: FAIL (exit 101) — hard blocker

Blocker: `crates/openwepp-hillslope-output/src/hillslope_wat.rs:66` (`clippy::unnecessary-sort-by`), suggested change is `metadata_pairs.sort_unstable_by_key(|(left_key, _)| *left_key)`.

Remaining required gates were not run due this hard blocker.
