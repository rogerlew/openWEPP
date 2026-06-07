# Worker Handoff

Status: executed-hold
Evidence mode: Static + Ran

Summary:
- Executed 43 single-OFE targets for FROSTVAL01 using `openwepp-cli-hill`.
- 6 targets ran (`p8,p13,p22,p23,p26,p28`), 37 blocked by `HS-RUNTIME-E-062`.
- Built paired ksflag-off runs for the 6 runnable targets and confirmed `1 0` in off soils.
- All 6 paired runs succeeded and produced identical frost/flux activation indicators versus ksflag-on.
- Closure residuals on runnable set are all classified `frost-break` for years 2-7.
- Legacy totalwatsed3 closure audit ran successfully.
- Runnable-subset totalwatsed3 generation and audit also ran successfully in `/workdir/wepppy/.venv`.

Run roots and evidence:
- `/tmp/frostval01/full/run_status.tsv`
- `/tmp/frostval01/full/off_status.tsv`
- `/tmp/frostval01/full/off_ksflag_checks.txt`
- `/tmp/frostval01/full/reports/activation_summary.csv`
- `/tmp/frostval01/full/reports/closure_yearly.csv`
- `/tmp/frostval01/full/reports/closure_prefix_summary.csv`
- `/tmp/frostval01/full/reports/totalwatsed3_legacy_flag_audit/daily_closure_audit_summary.json`
- `/tmp/frostval01/full/reports/totalwatsed3_subset_audit/daily_closure_audit_summary.json`

Recommended next package order:
1. Unblock `HS-RUNTIME-E-062` lineage coverage for the 37 blocked single-OFE targets.
2. Re-run full 43-target ksflag on/off activation audit.
3. Re-run full closure-under-frost ledger with complete population.
