# Full H1..H39 Suite Metrics

Status: executed
Evidence mode: Ran

Ran:

- Command: `.venv/bin/python docs/work-packages/20260605-hphys0292-spring-snowmelt-infiltration-capacity-lineage-closure-001/artifacts/hphys0292_diagnostics.py --run-root /tmp/hphys0292_full_release_segment_final_20260605T032734Z --trace-max-days 1300`.
- Full run root: `/tmp/hphys0292_full_release_segment_final_20260605T032734Z`.
- Runtime status: `/tmp/hphys0292_full_release_segment_final_20260605T032734Z/reports/hillslope_batch_status.tsv`.
- Semantic status: `/tmp/hphys0292_full_release_segment_final_20260605T032734Z/reports/semantic_status.tsv`.
- Semantic summary: `/tmp/hphys0292_full_release_segment_final_20260605T032734Z/reports/hillslope_semantic_summary.md`.

Result:

- Runtime: `39/39` hillslopes completed with `rc=0`.
- Semantic parity: `0/39`.
- `Q`: `39/39` pass, `total_fail_count=0`, `max_abs_diff=2.0816681711721685e-14`.
- `P`: `39/39` pass, `total_fail_count=0`.
- `Er`: `39/39` pass, `total_fail_count=0`.

Residual metrics:

| Symbol | Hillslope Fail Count | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| Ep | 39 | 42688 | 0.633657 | 7.100844 |
| Total-Soil | 39 | 52185 | 56.010071 | 317.130129 |
| SoilWaterTotal | 39 | 52185 | 56.010071 | 317.130129 |
| Dp | 38 | 10961 | 0.050444 | 0.244800 |
| latqcc | 39 | 38462 | 0.285882 | 3.023092 |
| RM | 39 | 7097 | 0.256086 | 27.960000 |
| Snow-Water | 39 | 10391 | 2.899431 | 65.506840 |

Interpretation:

- The prior positive spring `Q` residual was a WB14 routed-melt capacity allocation defect and is closed.
- Full semantic parity remains blocked by snowpack depletion timing/magnitude plus post-ingress storage/percolation/lateral coupling.
