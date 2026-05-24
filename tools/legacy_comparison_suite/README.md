# Legacy Comparison Suite

Purpose: reusable tooling for openWEPP-vs-legacy WEPP parity investigations,
with a default focus on hillslope water-balance semantic parity (PL14S scope).

## Scope
- Compares legacy `H*.wat.dat` outputs against openWEPP candidate outputs.
- Supports candidate input as legacy-style `.dat` or openWEPP `.parquet`.
- Produces investigation-oriented reports (per-column statistics, missing rows,
  top divergent rows), not only strict byte-level diff status.

## Why this exists
- Parity policy is semantic, not bit-for-bit (`ADR-0003`).
- Strict raw compare remains useful for structure checks, but by itself is not
  sufficient to diagnose why trajectories differ.
- PL14S and successor lanes need a repeatable, provenance-aware suite for
  recurring legacy comparison work.

## Tools
- `semantic_hillslope_wat_compare.py`
  - semantic comparator for hillslope `wat` surfaces.
- `run_pl14s_legacy_suite.py`
  - baseline replay + strict comparator (when candidate is `.dat`) + semantic
    comparator orchestration with provenance output.

## Quick start

Semantic compare only:

```bash
python3 tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py \
  --baseline-wat /tmp/baseline/output/H5.wat.dat \
  --candidate-wat /tmp/candidate/output/H5.wat.dat \
  --report-json /tmp/pl14s_semantic_report.json \
  --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json
```

Replay + compare orchestration:

```bash
python3 tools/legacy_comparison_suite/run_pl14s_legacy_suite.py \
  --baseline-run-dir /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0 \
  --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill \
  --baseline-run-file p5.run \
  --candidate-wat /tmp/openwepp_candidate/H5.wat.dat \
  --output-root /tmp/pl14s_suite_run
```

## Optional dependencies
- Reading candidate parquet requires `pyarrow`.
  - If unavailable, the semantic tool exits with a clear import error message.

## Current limitations
- Erosion/sediment surfaces are intentionally excluded in PL14S lane scope.
- This suite does not claim watershed or hourly parity closure gates.
