# Gate Results

Status: completed/HOLD

Evidence mode: Ran

## Commands

- `py_compile`: `rc=0`
  - `/workdir/wepppy/.venv/bin/python -m py_compile docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py`
- Diagnostic full run: `rc=0`
  - `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py --run-root /tmp/hphys0265_20260603T151958Z --trace-max-days 130`
- Corrected targeted refresh after fixing diagnostic `Total-Soil` merge naming:
  `rc=0`
  - `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py --run-root /tmp/hphys0265_20260603T151958Z --trace-max-days 130 --skip-full-suite`

## Run-Root Gates

- Build: `/tmp/hphys0265_20260603T151958Z/reports/build_status.tsv`, `rc=0`.
- Targeted traces: `/tmp/hphys0265_20260603T151958Z/reports/targeted_trace_status.tsv`,
  `rc=0` for H1/H7/H39.
- Full hillslope runtime:
  `/tmp/hphys0265_20260603T151958Z/reports/hillslope_batch_status.tsv`,
  `rc=0` for H1..H39.
- Semantic comparators:
  `/tmp/hphys0265_20260603T151958Z/reports/semantic_status.tsv`, `rc=0`
  for H1..H39; semantic pass remains `0/39`.

## Deferred Gates

- `cargo fmt --check`, `cargo clippy`, and `cargo test --workspace` were not
  run because no Rust production or test code was edited.
- `cargo deny check` was not run because dependency/source changes were not
  made.
