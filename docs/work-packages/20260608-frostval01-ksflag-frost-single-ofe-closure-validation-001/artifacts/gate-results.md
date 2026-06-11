# Gate Results

Status: complete-after-follow-ons
Evidence mode: Ran

2026-06-11 rerun:
- Built current release binary:
  - `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- Fresh run root:
  - `/tmp/frostval01_rerun_20260611T020951Z`
- Frost-on population:
  - `43/43` single-OFE prefixes exited clean after serially rerunning transient
    metadata-sidecar failure on `p4`.
  - `43/43` emitted WAT outputs.
  - `43/43` have `frsoil.active=true`.
  - `43/43` have nonzero `frozwt`; max `frozwt` range
    `27.499999999999993..31.000000000000007 mm`.
- Frost-off paired population:
  - `43/43` exited clean with explicit `[inputs.frost] wintRed = 0`.
  - `43/43` show nonzero on/off `sum_frozwt` delta.
  - `43/43` show nonzero on/off `Q` and `latqcc` deltas.
- Corrected full-WAT closure ledger:
  - `258` annual rows (`43` prefixes x years `2..7`).
  - Max abs residual: `3.2173375075217336e-11 mm`.
  - Worst row: `p39`, year `6`, residual `-3.2173375075217336e-11 mm`.

Historical original run:

Ran:
- Full single-OFE execution census (43 targets) with TOML wrappers via:
  - `openwepp-cli-hill --run-dir /wc1/runs/al/algebraic-radium/wepp/runs --run-file <wrapper.toml> --output-dir /tmp/frostval01/full/manifests/<prefix> --policy compat`
  - Result: 6 succeeded, 37 blocked by `HS-RUNTIME-E-062`.
- ksflag-off paired reruns for runnable prefixes:
  - Generated off soils by replacing standalone `1 1` with `1 0` exactly once.
  - Verification: `/tmp/frostval01/full/off_ksflag_checks.txt`.
  - Reran 6 off wrappers in `/tmp/frostval01/full/off_runs`.
  - Result: 6/6 succeeded.
- Activation and closure report extraction:
  - Produced `/tmp/frostval01/full/reports/activation_summary.csv`.
  - Produced `/tmp/frostval01/full/reports/closure_yearly.csv`.
  - Produced `/tmp/frostval01/full/reports/closure_prefix_summary.csv`.
- Legacy totalwatsed3 closure audit:
  - Command: `/workdir/openWEPP/.venv/bin/python /workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py /wc1/runs/al/algebraic-radium/wepp/output/interchange/totalwatsed3.parquet --output-dir /tmp/frostval01/full/reports/totalwatsed3_legacy_flag_audit`
  - Result: passed, summary JSON and top-days CSV emitted.
- Runnable-subset totalwatsed3 generation and closure audit (6 prefixes):
  - Commands executed in `/workdir/wepppy/.venv` with `PYTHONPATH=/workdir/wepppy`.
  - Subset interchange source generated at `/tmp/frostval01/full/subset_output/interchange/H.wat.parquet` and `/tmp/frostval01/full/subset_output/interchange/H.pass.parquet`.
  - Result: passed, `/tmp/frostval01/full/subset_output/interchange/totalwatsed3.parquet` emitted and audited to `/tmp/frostval01/full/reports/totalwatsed3_subset_audit`.
- Environment dependency patching for analytics:
  - Installed `duckdb`, `pyarrow`, and `utm` into `/workdir/openWEPP/.venv`.

Primary gate outcomes:
- Milestone 1 (prove frost active over full target scope): met by 2026-06-11 rerun.
- Closure-under-frost over full target scope: met by 2026-06-11 rerun.
- Truthful ledger completeness with explicit blocked accounting: met.
