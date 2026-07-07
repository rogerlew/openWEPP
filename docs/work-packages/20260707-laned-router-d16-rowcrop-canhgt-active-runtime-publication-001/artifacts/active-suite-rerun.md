# Active Suite Rerun

Status: EXECUTED-COMPLETE

Ran:

- Materialized the selected active cohort with
  `/home/workdir/wepppy/.venv/bin/python artifacts/materialize_selected_cohort.py`;
  the manifest contains four members.
- Rebuilt the release runner and ran the selected active suite with
  `/home/workdir/wepppy/.venv/bin/python artifacts/run_active_suite.py`.
- Summarized the suite with
  `/home/workdir/wepppy/.venv/bin/python artifacts/summarize_active_suite.py`.

Outcome:

- The former blocker is closed: `mn_corn_h4` active plain completed instead of
  failing on day 136 with positive LAI and missing/non-positive `canhgt`.
- All four selected members completed in active plain and active explicit
  hybrid modes.
- Detailed run logs live under `artifacts/active-suite-run-logs/`.
- Detailed command state lives in `artifacts/active-suite-command-log.json`.
- Summary outputs live in `artifacts/active-suite-run-summary.md` and
  `artifacts/active-suite-summary.md`.
