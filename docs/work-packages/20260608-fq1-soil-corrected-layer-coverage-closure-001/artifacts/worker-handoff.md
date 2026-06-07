# Worker Handoff

Evidence mode: `Ran:`.

First actionable item: close defect
`FQ1-P11-HKERNEL-WB11-PERC-E-003-J162` for
`/wc1/runs/al/algebraic-radium/wepp/runs/p11.*`.

Observed failure after FQ1 soil correction:

- Command shape: `openwepp-cli-hill --run-dir /wc1/runs/al/algebraic-radium/wepp/runs --run-file /tmp/fq1_after/runfiles/p11.toml --output-dir /tmp/fq1_after/outputs/p11 --policy compat --legacy-sidecar-discovery`.
- Deterministic rerun: `/tmp/fq1_after/outputs/p11_rerun/stderr.txt`.
- Message: `HKERNEL-WB11-PERC-E-003`.
- Phase: `percolation_deep_seepage`.
- Date: `calendar_year=1990`, `julian_day=162`, `sim_day_index=162`.
- Guard terms include `infiltration=0`, `slflag=1`, `kslast=0.0000000028`,
  `ui_bdrkth=10`, and `invalid_layers=none`.

Recommended authority envelope:

- Contracts: `SC-PERC-001`, `SC-WATBAL-001`; include snow/runoff authority only if
  localization proves the guard is caused by upstream snow/runoff state.
- Protected boundary from this package: do not reopen soil corrected-layer
  coverage unless new evidence shows `HS-RUNTIME-E-062` has returned.

FQ1 soil evidence to reuse:

- `/tmp/fq1_after/run_status.tsv`
- `artifacts/fq1-validation-ledger.md`
- `artifacts/corrected-layer-coverage-localization.md`
