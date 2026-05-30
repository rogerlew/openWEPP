# HPHYS0204 Worker Handoff

Status: completed  
Evidence mode: Static + Ran

## Immediate next actions (scoped follow-on packages)
1. `HPHYS0208` (proposed): residual-family closure for
   `Total-Soil`/`SoilWaterTotal` process-lineage migration.
   - Closure condition:
     contract-authoritative lineage proof and targeted guard vectors,
     with comparator residual direction tracked as diagnostic.
2. `HPHYS0209` (proposed): residual-family closure for `Dp`/`latqcc`
   percolation-lateral lineage and WB13 publication coupling.
   - Closure condition:
     baseline-authoritative process migration evidence and typed domain guard
     continuity.
3. `HPHYS0210` (proposed): FC/WP residual hold-lift adjudication for the
   remaining `27/39` and `1/39` set after HPHYS0207 stabilization.
   - Closure condition:
     explicit classification of expected process-correct deltas vs unresolved
     migration defects, then final hold-lift recommendation.

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
