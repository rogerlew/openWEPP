# Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

Static:

- Extended the opt-in HPHYS trace row with:
  - `wb18_fc_layers_m`
  - `wb19_coca_layers`
  - `wb19_frzw_layers_m`
  - `wb19_drfc_layers_m`
  - `wb19_fzdrfc_layers_m`
- Added contract-derived assertions that the trace row derives
  `drfc = fc + (1-coca)*dg` and `fzdrfc = max(drfc-frzw,0)`.
- Added HPHYS0267 diagnostic runner
  `docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/hphys0267_diagnostics.py`.

Ran:

- `cargo test -p openwepp-runner hphys0260_trace_row`
  - Result: passed, `1` test run.
- `cargo test -p openwepp-runner hphys0245_trace_writer`
  - Result: passed, `1` test run.
- `/workdir/wepppy/.venv/bin/python -m py_compile docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/artifacts/hphys0267_diagnostics.py`
  - Result: passed.
