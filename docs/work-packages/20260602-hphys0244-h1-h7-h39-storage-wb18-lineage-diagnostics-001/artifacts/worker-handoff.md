# HPHYS0244 Worker Handoff

Static: handoff from completed HPHYS0244 diagnostics.

## Current State
- HPHYS0244 is complete as a diagnostics package.
- No runtime/contract/test code was modified.
- Repository docs now include HPHYS0243 and HPHYS0244 uncommitted package
  additions.

## Key Evidence
- HPHYS0244 diagnostic root:
  `/tmp/hphys0244_20260602T045926Z`
- Candidate root:
  `/tmp/hphys0243_20260602T042747Z/parity`
- Targeted summary:
  `/tmp/hphys0244_20260602T045926Z/storage_dp_summary.tsv`
- First 30 days:
  `/tmp/hphys0244_20260602T045926Z/first_30_storage_dp_timeseries.tsv`
- Source lineage evidence:
  `/tmp/hphys0244_20260602T045926Z/source_line_evidence.txt`

## Immediate Next Step
If authorized, scaffold HPHYS0245 as a telemetry-first package for `H1`, `H7`,
and `H39`.

Required trace symbols:
- `wb18_perc_theta_*`
- `wb18_perc_pei_*`
- `D`
- `Pe`
- `wb11_soil_water`
- WB13 `Total-Soil`
- WB13 `SoilWaterTotal`

Required trace boundaries:
- post-seed
- post-WB18
- post-WB19
- pre-WB13
- post-WB13

## Caution
Do not implement a `Dp` clamp or tune percolation constants from WAT residuals
alone. HPHYS0244 shows `Dp` is a transient overdrain symptom coupled to a much
larger storage depletion residual.
