# PERFIDX04 Realized Speedup

Ran:

| Case | Baseline s | Final current s | Speedup | Ratio |
| --- | ---: | ---: | ---: | ---: |
| OFE1 | 5.27 | 5.50 | -4.36% | 0.958x |
| OFE2 | 11.01 | 9.72 | 11.72% | 1.133x |
| OFE3 | 16.35 | 13.92 | 14.86% | 1.175x |
| OFE4 | 26.24 | 22.07 | 15.89% | 1.189x |
| OFE5 | 26.65 | 22.85 | 14.26% | 1.166x |
| H2637 no-UI | 888.92 | 673.29 | 24.26% | 1.320x |
| H2637 with UI | 894.98 | 669.75 | 25.17% | 1.336x |

Ran:
- Baseline timing source: pre-PERFIDX04 release binary and anchor runs captured under `/tmp/perfidx04/baseline`.
- Final current timing source: `/tmp/perfidx04/artifacts/final-current-rerun-times.tsv`.

Static:
- PERFIDX04 does not decide the overall <=10x / <=5x verdict. That remains Stage 6.
