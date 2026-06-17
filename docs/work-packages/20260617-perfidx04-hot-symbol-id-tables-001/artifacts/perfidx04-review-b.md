# PERFIDX04 Review B

Static:
- Independent pass over profiler-specific acceptance:
  - Direct hot hourly `format!` construction was reduced to 0.01% self in final no-children report.
  - Frost fine-grid paths use pre-resolved grid symbols where a request is available.
  - PL active-slot dispatch uses parsed PL id tables instead of formatting schedule/growth names on the hot dispatch path.
- Reviewed residual profiler samples: remaining `format_inner` is attributed to cold/logical export and non-Stage-4 writeback/guard work.

Ran:
- Final `perf record` sample produced 9,495 samples and user-space symbol reports.

Conclusion:
- Stage-4 profiler gate is satisfied. Residual symbol machinery is appropriate Stage-5 follow-on territory.
