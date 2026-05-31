# HPHYS0221 Residual Gap Matrix

Status: completed
Evidence mode: Static + Ran

## Evidence sources
- HPHYS0219 summary:
  `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0221 summary:
  `/tmp/hphys0221_20260531T141839Z/parity/reports/hillslope_semantic_summary.json`

## Comparator integrity check (Ran)
- `zero_common_row_reports = 0`
- `nonzero_common_row_reports = 39`

| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP221-GAP-001` | Verify coupled-family movement after WB19 coupling implementation. | `latqcc` mean improved `0.7948120660697657 -> 0.7517675547493208`; `Total-Soil` and `SoilWaterTotal` improved `140.82816405718864 -> 140.7089613428272`; `Dp` regressed `0.2808444379937233 -> 0.32491959785932106`. | open |
| `HP221-GAP-002` | Verify fail-count reduction for always-fail families. | `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal` remain `39/39` fail hillslopes. | open |
| `HP221-GAP-003` | Verify no regression in FC/WP families. | `ProfileFCStore` unchanged (`2.052691160104116` mean, `27` fails); `ProfileWPStore` unchanged (`0.05729745831355476` mean, `1` fail). | closed |
| `HP221-GAP-004` | Confirm implementation-level closure readiness. | Coupling behavior implemented and validated, but residual closure criteria not met for hold-lift. | open |

## Summary
- HPHYS0221 implementation moved several coupled metrics in the expected
  direction but did not reduce fail saturation and regressed `Dp`.
- Disposition remains `HOLD`; follow-on work should target coupled calibration /
  additional baseline-lineage surfaces that govern `Dp` while preserving
  `latqcc` and total-soil gains.
