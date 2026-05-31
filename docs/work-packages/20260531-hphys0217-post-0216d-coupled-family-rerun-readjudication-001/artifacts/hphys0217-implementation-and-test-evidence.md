# HPHYS0217 Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

## Implementation scope executed
- Created and executed HPHYS0217 rerun/readjudication package.
- No production Rust code edits.
- No canonical contract/test edits.

## Ran execution evidence
- Rerun root:
  `/tmp/hphys0217_20260531T071120Z/`
- Hillslope run status:
  `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_batch_status.tsv`
  (`39/39` rc=0).
- Semantic status:
  `/tmp/hphys0217_20260531T071120Z/parity/reports/semantic_status.tsv`
  (`39/39` rc=0).
- Semantic summary:
  `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.json`
  and `.tsv`.

## Residual-family readjudication (Ran)
- `ProfileFCStore`: `27/39`, `2.052691160104116`
- `Dp`: `39/39`, `0.2643680891653757`
- `latqcc`: `39/39`, `0.8131880775568228`
- `Total-Soil`: `39/39`, `140.87503038397858`
- `SoilWaterTotal`: `39/39`, `140.87503038397858`

## Comparison to HPHYS0216 reference (Ran + Static)
- HPHYS0216 reference summary:
  `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_semantic_summary.json`
- `ProfileFCStore` improved from `39/39` (`7.22117381046073`) to
  `27/39` (`2.052691160104116`) after HPHYS0216D fix.
- `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal` remained effectively
  unchanged and saturated.

## Discovery and recovery note (Ran)
- First semantic attempt failed due missing `pyarrow` on system `python3`.
- Re-ran semantic stage with `.venv/bin/python` (contains `pyarrow`) and
  completed `39/39`.
