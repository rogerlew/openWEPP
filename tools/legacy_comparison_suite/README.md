# Legacy Comparison Suite

Purpose: reusable tooling for openWEPP-vs-legacy WEPP parity investigations,
with a default focus on hillslope water-balance semantic parity (PL14S scope).

## Scope
- Compares legacy `H*.wat.dat` outputs against openWEPP candidate outputs.
- Supports candidate input as legacy-style `.dat` or openWEPP `.parquet`.
- Produces investigation-oriented reports (row-presence deltas, per-column
  tolerance verdicts, top divergent rows), not only strict byte-level diff
  status.

## Mandatory input identity
- Parity runs are valid only when baseline and candidate use the same input
  surfaces for the tested lane.
- At minimum, the following files must be identical across baseline/candidate
  runs: soil (`*.sol`), management/landuse (`*.man`), slope (`*.slp`), climate
  (`*.cli`), plus required sidecars (`pmetpara.txt`, `snow.txt`,
  `wepp_ui.txt`, and any other lane-required sidecar).
- Record and retain input hash evidence (for example `sha256sum`) for these
  files in each parity evidence bundle.
- Comparator outcomes produced without identical input-file evidence are
  investigation-only and are not promotable parity closure evidence.

## Why this exists
- Parity policy is semantic, not bit-for-bit (`ADR-0003`).
- Strict raw compare remains useful for structure checks, but by itself is not
  sufficient to diagnose why trajectories differ.
- PL14S and successor lanes need a repeatable, provenance-aware suite for
  recurring legacy comparison work.

## Tools
- `semantic_hillslope_wat_compare.py`
  - semantic comparator for hillslope `wat` surfaces.
  - emits report schema `pl14s-semantic-wat-v2`.
  - hard-fails on duplicate `(OFE,J,Y)` row keys.
- `run_pl14s_legacy_suite.py`
  - baseline replay + strict comparator (when candidate is `.dat`) + semantic
    comparator orchestration with provenance output.
  - emits provenance schema `pl14s-legacy-suite-v2`.
  - enforces deterministic strict-lane policy metadata and candidate source
    classification before writing provenance.
  - enforces conversion-derived dat row-consistency checks against replay-span
    overlap evidence before closeout readiness is emitted.

## Repo-local uv environment (replicable)

Create a repo-local virtualenv in a fresh clone:

```bash
cd /path/to/openWEPP
uv venv .venv --python 3.12
source .venv/bin/activate
uv pip sync tools/legacy_comparison_suite/requirements.lock.txt
```

Notes:
- `.venv/` is gitignored at repo root.
- The locked dependency source is:
  - `tools/legacy_comparison_suite/requirements.in`
  - `tools/legacy_comparison_suite/requirements.lock.txt`
- Refresh lock after dependency edits:

```bash
uv pip compile tools/legacy_comparison_suite/requirements.in \
  --output-file tools/legacy_comparison_suite/requirements.lock.txt \
  --python-version 3.12
```

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
  --candidate-surface-source-class native-runtime-dat \
  --output-root /tmp/pl14s_suite_run
```

Parquet candidate example:

```bash
python3 tools/legacy_comparison_suite/run_pl14s_legacy_suite.py \
  --baseline-run-dir /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0 \
  --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill \
  --baseline-run-file p5.run \
  --candidate-wat /tmp/openwepp_candidate/interchange/H.wat.parquet \
  --candidate-surface-source-class native-runtime-parquet \
  --output-root /tmp/pl14s_suite_run
```

## Guard posture
- Strict comparator is required when candidate input is `.dat`; parquet runs
  are classified as `strict-equivalent-required` and must satisfy semantic-lane
  equivalence checks.
- `--candidate-surface-source-class` is required and must be one of:
  `native-runtime-dat`, `conversion-derived-dat`, or
  `native-runtime-parquet`. `conversion-derived-dat` evidence is tagged as
  non-promotable for final Tier-A closeout claims and must satisfy explicit
  row-consistency checks (`common_row_count > 0`, no unmatched baseline/candidate rows).
- Semantic comparator evidence is required for all runs and must include:
  - row-presence deltas,
  - per-column tolerance verdicts,
  - investigation columns used/missing,
  - canonical alias continuity for `Total-Soil`,
  - observed row-width diagnostics,
  - baseline-only column disclosure,
  - top divergent row diagnostics.
- Missing or malformed semantic report content is treated as an execution
  failure by `run_pl14s_legacy_suite.py`.

## Optional dependencies
- Parquet candidate reads require `pyarrow` (included in
  `requirements.lock.txt`).

## Current limitations
- Erosion/sediment surfaces are intentionally excluded in PL14S lane scope.
- This suite does not claim watershed or hourly parity closure gates.
