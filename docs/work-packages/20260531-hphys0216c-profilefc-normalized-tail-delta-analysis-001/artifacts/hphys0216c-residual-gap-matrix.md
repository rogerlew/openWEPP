# HPHYS0216C Residual Gap Matrix

Status: completed
Evidence mode: Static + Ran

## Comparator sources
- HPHYS0216 semantic summary:
  `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_semantic_summary.tsv`
- Baseline partitions:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`

## Integrated family status (from HPHYS0216 outputs)
| Family/Column | Fail Hillslopes | Mean Abs Diff Avg | Status |
| --- | --- | --- | --- |
| `ProfileFCStore` | `39/39` | `7.22117381046073` | regressed-vs-HPHYS0214 |
| `Dp` | `39/39` | `0.26436808916537585` | unchanged-open |
| `latqcc` | `39/39` | `0.8131880775568225` | unchanged-open |
| `Total-Soil` | `39/39` | `140.87503038397853` | unchanged-open |
| `SoilWaterTotal` | `39/39` | `140.87503038397853` | unchanged-open |

## ProfileFCStore delta characterization
Computed by joining baseline and HPHYS0216 candidate rows on
`(OFE, julian, year+2012)` and aggregating `candidate_fc - baseline_fc`.

Observed properties:
1. All 39 hillslopes have a negative signed FC offset.
2. The offset is constant per hillslope across all days.
3. No FC ordering guard violation is observed in HPHYS0216 candidate outputs
   (`ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore` remains true).

Offset clusters (`candidate_fc - baseline_fc`, mm):
| Offset (mm) | Hillslope Count | Hillslope IDs |
| --- | --- | --- |
| `-11.334426` | `1` | `7` |
| `-9.875433` | `10` | `2, 3, 8, 9, 11, 12, 14, 16, 21, 26` |
| `-7.071693` | `2` | `37, 39` |
| `-6.503285` | `19` | `1, 5, 6, 10, 13, 15, 19, 20, 22, 23, 24, 25, 27, 28, 30, 32, 34, 35, 36` |
| `-4.833031` | `7` | `4, 17, 18, 29, 31, 33, 38` |

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP216C-GAP-001` | `ProfileFCStore` regression is deterministic and profile-static, not stochastic/time-varying. | Ran: cohort join diagnostics (duckdb) + semantic summary | open |
| `HP216C-GAP-002` | FC publication moved to mapped parser-layer aggregation while seed path still carries normalized-depth tail contributions, creating constant per-profile offsets. | Static: `runtime_inputs/02_soil_slope.rs` lineage + runner FC publication path | open |
| `HP216C-GAP-003` | Follow-up remediation package required before `HPHYS0217` closure claims are promotable. | Static + Ran: HPHYS0216 disposition + this package | open |
