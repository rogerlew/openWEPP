# HPHYS0213 Residual Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Sources
- HPHYS0212 baseline residual summary:
  `docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/artifacts/hphys0212-residual-gap-matrix.md`
- HPHYS0213 rerun outputs:
  - `/tmp/hphys0213_20260530T233248Z/parity/reports/hillslope_batch_status.tsv`
  - `/tmp/hphys0213_20260530T233248Z/parity/reports/semantic_status.tsv`
  - `/tmp/hphys0213_20260530T233248Z/parity/reports/hillslope_semantic_summary.json`
  - `/tmp/hphys0213_20260530T233248Z/parity/reports/semantic/H*.semantic.json`
  - `/tmp/hphys0213_20260530T233248Z/parity/logs/h5.stderr.log`

## Execution health
- Hillslope execution success: `39/39`.
- Semantic reports generated: `39/39`.
- H5 blocker status: `HKERNEL-WB12-STORAGE-E-003` no longer present.

## Monitored family comparison (HPHYS0212 -> HPHYS0213)
| Family | Fail hillslopes | Mean abs diff average | Assessment |
| --- | --- | --- | --- |
| `ProfileFCStore` | `26/38 -> 27/39` | `2.03798 -> 2.05269` | unresolved, slight regression |
| `Dp` | `38/38 -> 39/39` | `0.26440 -> 0.26437` | unresolved fail saturation, negligible magnitude improvement |
| `latqcc` | `38/38 -> 39/39` | `0.95829 -> 0.81319` | unresolved fail saturation, magnitude improved |
| `Total-Soil` | `38/38 -> 39/39` | `141.25575 -> 140.87503` | unresolved fail saturation, magnitude improved |
| `SoilWaterTotal` | `38/38 -> 39/39` | `141.25575 -> 140.87503` | unresolved fail saturation, magnitude improved |

## HPHYS0213 closure-measure adjudication
1. `MEASURE-HP213-001` contract-first evidence complete: **pass**.
2. `MEASURE-HP213-002` WB12 storage domain failure closed for H5: **pass**.
3. `MEASURE-HP213-003` WB19 `q`/`Qdd`/`Qd` bounded by realized withdrawals:
   **pass**.
4. `MEASURE-HP213-004` WB11 aggregate soil-water lineage updated after WB19
   mutation: **pass**.
5. `MEASURE-HP213-005` required gates pass + rerun evidence published:
   **pass**.

## Residual blockers and follow-on ownership
| Gap ID | Description | Evidence | Owner package |
| --- | --- | --- | --- |
| `HP213-GAP-001` | Monitored comparator families remain fail-saturated (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`). | semantic summary + `H*.semantic.json` | `HPHYS0214` |
| `HP213-GAP-002` | Hold-lift adjudication across process-authority closure and residual diagnostics remains pending. | package disposition + gate bundle | `HPHYS0214` |
