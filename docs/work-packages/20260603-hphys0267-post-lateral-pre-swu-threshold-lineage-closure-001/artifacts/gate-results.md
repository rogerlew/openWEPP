# Gate Results

Status: completed
Evidence mode: Static + Ran

Ran:

- `cargo fmt --check`
  - Result: passed.
- `git diff --check`
  - Result: passed.
- `cargo test -p openwepp-runner hphys0260_trace_row`
  - Result: passed, `1` test run.
- `cargo test -p openwepp-runner hphys0245_trace_writer`
  - Result: passed, `1` test run.
- `/workdir/wepppy/.venv/bin/python -m py_compile docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/hphys0267_diagnostics.py`
  - Result: passed.
- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/hphys0267_diagnostics.py --run-root /tmp/hphys0267_20260603T162040Z --trace-max-days 130`
  - Result: passed; produced targeted traces and full H1..H39 semantic
    metrics.
- Reprocessed threshold classification from existing trace artifacts after
  constraining delta-closure classification to withdrawal layers.
  - Result: passed; final classifications are recorded in
    `targeted-h1-h7-h39-threshold-lineage-classification.md`.

Static:

- Full workspace `cargo clippy --workspace --all-targets -- -D warnings`,
  full `cargo test --workspace`, and `cargo deny check` were not run because
  this package made trace-only runner changes, did not patch production
  physics, and remains in HOLD rather than declaring kernel implementation
  closure.
