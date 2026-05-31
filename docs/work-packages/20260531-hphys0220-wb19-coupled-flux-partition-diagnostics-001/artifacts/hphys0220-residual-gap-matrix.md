# HPHYS0220 Residual Gap Matrix

Status: completed
Evidence mode: Static + Ran

## Evidence sources
- HPHYS0218 summary:
  `/tmp/hphys0218_20260531T075251Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0219 summary:
  `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.json`
- Baseline source:
  `/workdir/wepp-forest_260430_baseline/src/watbal.for`

## Comparator integrity check (Ran)
- `zero_common_row_reports = 0`
- `nonzero_common_row_reports = 39`

| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP220-GAP-001` | Residual directionality must be classified for HPHYS0219 vs HPHYS0218. | `Dp` improved `39/39`; `latqcc` regressed `39/39`; `Total-Soil` and `SoilWaterTotal` regressed `39/39`. | closed |
| `HP220-GAP-002` | Determine whether coupled tradeoff is random or structural. | `corr(ΔDp,Δlatqcc)=-0.9997641396512593` across 39 hillslopes. | closed (structural) |
| `HP220-GAP-003` | Identify missing baseline WB19 process surfaces in openWEPP lineage. | Baseline contains `avcoca`/`watyld`/`fcdep`/`unsdep` coupling not represented in current openWEPP WB19 kernels. | open |
| `HP220-GAP-004` | Provide executable remediation scope with contract-first sequence. | HPHYS0221 remediation handoff published with explicit scope and gates. | closed |

## Summary
- HPHYS0219 corrected coefficient authority, but coupled residual closure is
  blocked by structural WB19 flux-partition behavior.
- Next progress requires WB19 water-yield and saturated-depth coupling
  remediation under contract-first governance.
