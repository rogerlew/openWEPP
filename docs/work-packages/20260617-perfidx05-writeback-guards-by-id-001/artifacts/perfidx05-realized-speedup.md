# PERFIDX05 Realized Speedup

Ran:
- Fresh PERFIDX04 baseline rerun: `/tmp/perfidx05/artifacts/baseline-rerun-times.tsv`
- Final PERFIDX05 rerun: `/tmp/perfidx05/artifacts/final-current-rerun-times.tsv`
- Speedup table: `/tmp/perfidx05/artifacts/final-speedups-vs-fresh-baseline.tsv`

Result versus fresh PERFIDX04 baseline:

| Case | Baseline s | PERFIDX05 s | Speedup |
| --- | ---: | ---: | ---: |
| ofe1_same | 5.28 | 5.27 | 0.19% |
| ofe2_same | 9.58 | 10.09 | -5.32% |
| ofe3_same | 14.11 | 14.73 | -4.39% |
| ofe4_same | 22.45 | 23.64 | -5.30% |
| ofe5_same | 21.83 | 23.70 | -8.57% |
| h2637_same | 674.78 | 713.68 | -5.76% |
| h2637_with_ui_same | 673.20 | 709.09 | -5.33% |

Conclusion:
- PERFIDX05 is behavior-preserving but not speed-positive on the final timed anchor.
- Do not use PERFIDX05 to assert the Stage-6 `<=10x` verdict.
- The next performance package should start with profiling the residual logical surfaces,
  especially decomposition overflow scanning and writeback/transfer dual logical+indexed
  mutation cost.
