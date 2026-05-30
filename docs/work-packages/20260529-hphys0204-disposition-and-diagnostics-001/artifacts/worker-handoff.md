# HPHYS0204 Worker Handoff

Status: completed  
Evidence mode: Static + Ran

## Immediate next actions (scoped follow-on packages)
1. `HPHYS0208` (proposed): coupled threshold-lineage closure for open FC and
   subsurface/soil-water families:
   `ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`.
   - Closure condition:
     contract-authoritative kernel threshold-lineage closure
     (`thetfc_####`/`thetdr_####` authority and downstream consumers), targeted
     guard vectors, and residual-direction diagnostics.
2. `HPHYS0209` (proposed): near-closed WP adjudication lane for
   `ProfileWPStore` (`1/39`).
   - Closure condition:
     determine whether the remaining WP residual is an expected
     process-correct delta or unresolved migration defect.
3. `HPHYS0210` (proposed): integrated hold-lift adjudication after HPHYS0208
   and HPHYS0209.
   - Closure condition:
     explicit classification of remaining deltas vs unresolved migration
     defects, then final hold-lift recommendation.

## Guardrails for next packages
- Maintain contract-first sequencing for any kernel-affecting changes.
- Do not use parity-only metrics as authority override.
- Preserve typed fail-closed posture; no silent defaults/clamps for invalid
  process states.

## Handoff evidence bundle
- Workspace gates (HPHYS0204 run): `fmt`, `clippy`, `test`, `deny` all pass.
- Latest comparator lane:
  `/tmp/hphys0207_20260530T042607Z/parity/reports/`
  - `hillslope_batch_status.tsv`
  - `semantic_status.tsv`
  - `hillslope_semantic_summary.json`
  - `semantic/H*.semantic.json`
