# Full 39 Suite Metrics

Status: executed
Evidence mode: Ran

Ran:
- Full H1..H39 semantic suite under
  `/tmp/hphys0296_full_20260605T070000Z`.
- Source summary:
  `/tmp/hphys0296_full_20260605T070000Z/reports/hillslope_semantic_summary.md`.

Overall:
- Semantic pass: `0/39`.
- `Q` pass: `39/39`.

| Metric | Pass count | Total fail count | Mean abs diff mean | Max abs diff |
|---|---:|---:|---:|---:|
| `Ep` | `0/39` | `42688` | `0.633657` | `7.100844` |
| `Total-Soil` | `0/39` | `52185` | `56.010071` | `317.130129` |
| `SoilWaterTotal` | `0/39` | `52185` | `56.010071` | `317.130129` |
| `Dp` | `1/39` | `10961` | `0.050444` | `0.244800` |
| `latqcc` | `0/39` | `38462` | `0.285882` | `3.023092` |
| `Q` | `39/39` | `0` | `0.000000` | `0.000000` |
| `RM` | `0/39` | `7097` | `0.256086` | `27.960000` |
| `Snow-Water` | `0/39` | `10391` | `2.899431` | `65.506840` |

Interpretation:
- HPHYS0296 changed contracts/tests/diagnostics only; production metrics are
  unchanged from HPHYS0295.
- `Q` remains closed.
- Semantic parity remains open due snow/`RM`, storage, ET, percolation, and
  lateral residuals.
