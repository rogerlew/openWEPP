# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: ran

Ran:

```bash
python3 docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/hphys0291_diagnostics.py \
  --run-root /tmp/hphys0291_full_release_current_20260605T020128Z
```

Outputs:

- Run root: `/tmp/hphys0291_full_release_current_20260605T020128Z`
- Summary: `/tmp/hphys0291_full_release_current_20260605T020128Z/reports/hillslope_semantic_summary.md`
- Selected metrics: `/tmp/hphys0291_full_release_current_20260605T020128Z/reports/hphys0291_selected_metrics.json`
- Runtime status: `/tmp/hphys0291_full_release_current_20260605T020128Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0291_full_release_current_20260605T020128Z/reports/semantic_status.tsv`

Result:

- Runtime: `39/39` hillslopes completed with `rc=0`.
- Semantic parity: `0/39`.

| Symbol | Passing Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | ---: | ---: | ---: | ---: |
| `P` | `39/39` | `0` | `0.000000` | `0.000000` |
| `RM` | `0/39` | `7097` | `0.256086` | `27.960000` |
| `Q` | `0/39` | `2108` | `0.552220` | `38.472185` |
| `Ep` | `0/39` | `45401` | `0.727061` | `7.242659` |
| `Es` | `38/39` | `500` | `0.010422` | `1.825681` |
| `Er` | `39/39` | `0` | `0.000000` | `0.000000` |
| `Dp` | `1/39` | `9220` | `0.042845` | `0.244800` |
| `latqcc` | `0/39` | `36003` | `0.373461` | `11.865076` |
| `Total-Soil` | `0/39` | `52521` | `57.069194` | `348.886998` |
| `SoilWaterTotal` | `0/39` | `52521` | `57.069194` | `348.886998` |
| `Snow-Water` | `0/39` | `10391` | `2.899431` | `65.506840` |

Interpretation:

- Static/Ran: HPHYS0291 is lifecycle hardening and does not claim semantic
  closure for the H1..H39 suite.
- Static/Ran: Remaining parity failures continue to implicate upstream
  snowpack/liquid partitioning and downstream storage/ET coupling, not missing
  WB13 same-day producer publication.
