# H1/H7/H39 Trace Evidence

Status: executed
Evidence mode: Ran

Ran:

- Target trace command: `.venv/bin/python docs/work-packages/20260605-hphys0292-spring-snowmelt-infiltration-capacity-lineage-closure-001/artifacts/hphys0292_diagnostics.py --run-root /tmp/hphys0292_full_release_segment_final_20260605T032734Z --trace-max-days 1300 --skip-full-suite`.
- Trace status: `/tmp/hphys0292_full_release_segment_final_20260605T032734Z/reports/hphys0292_target_trace_status.tsv`.
- Target capacity rows: `/tmp/hphys0292_full_release_segment_final_20260605T032734Z/reports/hphys0292_target_capacity_rows.md`.
- Extraction result: `rc=0`.

Evidence summary:

- H1/H7/H39 target traces use schema `hphys0245.v15`.
- Final target rows publish `wb14_effective_conductivity_m_s` equivalent to 40 mm/h for the selected spring rows.
- `wb12_infiltration_m` equals `snow_routed_melt_m` on active snowmelt-only target rows; residual `Q` is zero.
- Rows with positive direct post-winter rain retain the explicit post-rain diagnostic and do not classify that rain as snowmelt.
