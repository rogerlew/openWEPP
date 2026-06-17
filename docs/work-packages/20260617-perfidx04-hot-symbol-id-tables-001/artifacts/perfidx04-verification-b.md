# PERFIDX04 Verification B

Ran:
- `perf record` final sample confirmed profiling availability with `perf_event_paranoid` no longer blocking.
- Final speedup evidence recorded:
  - H2637 no-UI: 888.92s -> 673.29s.
  - H2637 with UI: 894.98s -> 669.75s.
  - OFE5: 26.65s -> 22.85s.

Static:
- No SC contract changes were made.
- No writeback payload shape change was made.
- No irrigation activation or pre-resolution was added.

Conclusion:
- Performance verification passed with scoped residual follow-on noted for Stage 5.
