# PL14S Replay Lane Configuration And Guard Map

Status: `completed`
Evidence mode: `Static`

## Static

| Lane surface | Authority | Guard / failure posture |
|---|---|---|
| Tier-A replay provenance completeness | `SC-SYSTEM-001` `INV-SYSTEM-012`, `INV-SYSTEM-017`, `OBL-SYSTEM-P-005`, `OBL-SYSTEM-P-006` | Missing strict/semantic comparator artifacts, missing provenance hashes, or missing strict skip/execution status is hard-fail / `HOLD`. |
| WB13 candidate schema/order integrity | `SC-WATBAL-001` `INV-WATBAL-012` | Missing required symbols, schema/order violations, or malformed replay rows hard-fail prior to comparator publication. |
| WB13 semantic diagnostics completeness | `SC-WATBAL-001` `INV-WATBAL-017`, `OBL-WATBAL-P-005` | Semantic report must include row-presence deltas, per-column tolerance verdicts, investigation diagnostics, and baseline-only column disclosure; omissions are hard-fail evidence defects. |
| Semantic tolerance profile authority | `SC-SYSTEM-001` `TOL-SYSTEM-007` + `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json` | Comparator interpretation uses explicit default/column override tolerances; tolerance profile is evidence and does not replace strict structural checks. |
| Strict compare branch selection | `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py` | `.dat` candidate: strict comparator is required and executed. `.parquet` candidate: strict comparator is explicitly marked skipped in provenance with reason. |
| Semantic compare branch selection | `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py` + `semantic_hillslope_wat_compare.py` | Semantic comparator is always required; non-zero exit or missing required report fields fails lane execution. |
| Row-key uniqueness | `semantic_hillslope_wat_compare.py` | Duplicate `(OFE,J,Y)` keys in baseline or candidate inputs hard-fail (no silent overwrite). |
| Baseline artifact discovery | `run_pl14s_legacy_suite.py` | `H*.wat.dat` discovery under baseline lane must resolve to exactly one file; zero or multiple matches hard-fail. |
| Erosion exclusion scope | PL14S package scope + `pl14s-erosion-exclusion-note.md` | Erosion/sediment surfaces are out of scope; replay lane assertions are hillslope water-balance only. |

## Canonical artifacts produced by configured lane
- Semantic comparator report:
  - `investigation/h5_wat_semantic_comparator.json`
- Strict comparator report (when `.dat` candidate):
  - `investigation/h5_wat_strict_comparator.json`
- Replay provenance manifest:
  - `investigation/pl14s_provenance_manifest.json`

## Ran
- Phase A lane-map artifact is static-by-scope.
- Phase C replay execution outcomes against this lane map are recorded in:
  - `pl14s-comparator-run-provenance-manifest.md`
  - `pl14s-tier-a-semantic-parity-delta-report.md`
