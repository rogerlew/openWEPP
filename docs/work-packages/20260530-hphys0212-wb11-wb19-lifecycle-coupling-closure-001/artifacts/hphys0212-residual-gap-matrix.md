# HPHYS0212 Residual Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Sources
- HPHYS0211 baseline residual summary:
  `docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/hphys0211-residual-gap-matrix.md`
- HPHYS0212 rerun outputs:
  - `/tmp/hphys0212_20260530T221447Z/parity/reports/hillslope_batch_status.tsv`
  - `/tmp/hphys0212_20260530T221447Z/parity/reports/hillslope_semantic_summary.json`
  - `/tmp/hphys0212_20260530T221447Z/parity/reports/semantic/H*.semantic.json`
  - `/tmp/hphys0212_20260530T221447Z/parity/logs/h5.stderr.log`

## Execution health
- Hillslope execution success: `38/39`
- Hillslope execution failure: `H5` (`rc=1`)
  - Failure signature:
    `HKERNEL-WB12-STORAGE-E-003` during `storage_reconciliation`
    (`DOMAIN_VIOLATION`), surfaced through `CLIHILL-E-011`.
- Semantic reports generated: `38/39` (missing `H5`).

## Monitored family comparison
| Family | HPHYS0211 fail hillslopes | HPHYS0212 fail hillslopes | Mean abs diff (HP211 -> HP212) | Assessment |
| --- | --- | --- | --- | --- |
| `ProfileFCStore` | `27/39` | `26/38` | `2.0527 -> 2.0380` | slight improvement, unresolved |
| `Dp` | `39/39` | `38/38` | `40.1559 -> 0.2644` | large magnitude improvement, fail saturation persists in executed subset |
| `latqcc` | `39/39` | `38/38` | `173.2285 -> 0.9583` | large magnitude improvement, fail saturation persists in executed subset |
| `Total-Soil` | `39/39` | `38/38` | `116.0649 -> 141.2557` | unresolved, magnitude worsened |
| `SoilWaterTotal` | `39/39` | `38/38` | `116.0649 -> 141.2557` | unresolved, magnitude worsened |

## HPHYS0212 closure-measure adjudication
1. `MEASURE-HP212-001` contract-first evidence complete: **pass**.
2. `MEASURE-HP212-002` no daily WB11/WB18 mutable reseed regression:
   **pass** (targeted test coverage + code-path change).
3. `MEASURE-HP212-003` WB19 controls runtime-input sourced:
   **pass** (runner guard + soil/management projections).
4. `MEASURE-HP212-004` WB13 deterministic `latqcc`/`Tile`/`Qd` coupling guard:
   **pass**.
5. `MEASURE-HP212-005` gates pass + residual deltas published with truthful
   decisioning: **pass**.

## Residual blockers and follow-on ownership
| Gap ID | Description | Evidence | Owner package |
| --- | --- | --- | --- |
| `HP212-GAP-001` | `H5` runtime execution fails in WB12 storage reconciliation (`HKERNEL-WB12-STORAGE-E-003`), preventing full 39/39 adjudication. | `h5.stderr.log` | `HPHYS0213` |
| `HP212-GAP-002` | `Dp` and `latqcc` fail saturation remains across all executed semantic reports despite major magnitude reduction. | semantic summary + `H*.semantic.json` | `HPHYS0213` |
| `HP212-GAP-003` | `Total-Soil` and `SoilWaterTotal` remain saturated and worsened by mean absolute difference. | semantic summary | `HPHYS0213` |
| `HP212-GAP-004` | `ProfileFCStore` remains partially failing and needs authority adjudication after coupled WB11/WB19/WB12 closure. | semantic summary | `HPHYS0214` |
