# Worker Handoff

Status: complete-after-follow-ons
Evidence mode: Static + Ran

2026-06-11 rerun summary:
- FROSTVAL01 was rerun after FQ-1/FQ-3/FQ-4 repaired the substrate and folded in
  the corrected full-WAT closure ledger.
- Fresh run root: `/tmp/frostval01_rerun_20260611T020951Z`.
- Built current `openwepp-cli-hill` release binary before execution.
- Frost-on population: 43/43 clean exits, 43/43 WAT outputs,
  43/43 `frsoil.active=true`, 43/43 nonzero `frozwt`.
- Frost-off paired population: 43/43 clean exits with `[inputs.frost] wintRed = 0`.
- Paired on/off deltas prove hydrology changes under frost: 43/43 nonzero `Q`
  delta and 43/43 nonzero `latqcc` delta.
- Corrected annual closure-under-frost: 258 rows, max abs residual
  `3.2173375075217336e-11 mm`.
- No new defect-shaped follow-on is required for standard `ksflag` frost
  activation or single-OFE closure-under-frost.

Historical original-run summary:
- Executed 43 single-OFE targets for FROSTVAL01 using `openwepp-cli-hill`.
- 6 targets ran (`p8,p13,p22,p23,p26,p28`), 37 blocked by `HS-RUNTIME-E-062`.
- Built paired ksflag-off runs for the 6 runnable targets and confirmed `1 0` in off soils.
- All 6 paired runs succeeded and produced identical frost/flux activation indicators versus ksflag-on.
- Closure residuals on runnable set are all classified `frost-break` for years 2-7.
- Legacy totalwatsed3 closure audit ran successfully.
- Runnable-subset totalwatsed3 generation and audit also ran successfully in `/workdir/wepppy/.venv`.

Current run root and evidence:
- `/tmp/frostval01_rerun_20260611T020951Z/run_status.tsv`
- `/tmp/frostval01_rerun_20260611T020951Z/off/run_status.tsv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/activation_summary.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/frost_on_off_deltas.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/annual_closure_residuals.csv`
- `/tmp/frostval01_rerun_20260611T020951Z/reports/summary.json`
- `artifacts/rerun-20260611-frostval01.md`

Historical run roots and evidence:
- `/tmp/frostval01/full/run_status.tsv`
- `/tmp/frostval01/full/off_status.tsv`
- `/tmp/frostval01/full/off_ksflag_checks.txt`
- `/tmp/frostval01/full/reports/activation_summary.csv`
- `/tmp/frostval01/full/reports/closure_yearly.csv`
- `/tmp/frostval01/full/reports/closure_prefix_summary.csv`
- `/tmp/frostval01/full/reports/totalwatsed3_legacy_flag_audit/daily_closure_audit_summary.json`
- `/tmp/frostval01/full/reports/totalwatsed3_subset_audit/daily_closure_audit_summary.json`

Remaining boundaries:
1. Frost depth magnitude/parity is outside FROSTVAL01 and remains with FDMC01/FDHP01.
2. MOFE/routing remains outside this single-OFE package.
3. Forest `ksatadj` remains separate from this standard-`ksflag` validation.
