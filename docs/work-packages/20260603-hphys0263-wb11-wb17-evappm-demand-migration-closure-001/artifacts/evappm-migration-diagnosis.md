# EVAPPM Migration Diagnosis

Status: completed

Evidence mode: static+ran

Static: Pinned legacy inspection identified the immediate HPHYS0262 Ep-lineage
defect as WB11 seeding from the Priestley-Taylor `evap` branch despite PMET
sidecar mode selecting `evappm`.

Ran: HPHYS0263 diagnostics classified H1/H7/H39 after migration using
`/tmp/hphys0263_20260603T070311Z`.

## Legacy Authority

- `watbal_hourly.for` selects `evap` only when `iflget.eq.1`; any other
  `iflget` selects `evappm`.
- `evappm.for:181-388` computes reference ET, PMET crop coefficient,
  soil evaporation reduction, plant transpiration stress, and final plant
  transpiration demand.
- `sunmap.for:181-234` provides the horizontal potential solar radiation
  derivation used when runtime `radpot` is not present.
- `evappm.for:391-454` remains a separate post-demand redistribution segment
  that mutates soil evaporation/storage after demand computation.

## Migrated Demand Surface

- Branch selector: `pmetpara.mode.iflget != 1`.
- Branch trace: `evappm_pmet`.
- Runtime demand output: `wb11_et_demand = pmet.ep_m`.
- PMET diagnostics include `etorc`, `rn`, `fwv`, `rhd`, `kcbadj`, `kcbcon`,
  `etke`, `etkr`, `etks`, `TEW`, `REW`, `wfevp`, `TAW`, `RAW`, `wftrp`,
  `pmet.es_m`, and `pmet.ep_m`.

## H1/H7/H39 Result

- Classification: `EVAPPM_MIGRATED_BRANCH_OBSERVED`.
- `iflget`: `2`.
- `kcb`: `0.95`.
- `rawp`: `0.80`.
- Candidate seed branch: `evappm_pmet`.
- Day-1 baseline `Ep`: `0.150000 mm`.
- Day-1 candidate `Ep`: `0.151823 mm`.
- Day-1 residual: `+0.001823 mm`.

## Residual Interpretation

- HPHYS0263 closed the large day-1 PMET demand branch defect observed after
  HPHYS0262.
- Remaining full-suite `Ep` and storage residuals are no longer explained by
  using the wrong WB11 demand branch for PMET-mode H1/H7/H39.
- The next EVAPPM-specific owner is the unported post-ET redistribution in
  `evappm.for:391-454`, especially where `Ep`, `Total-Soil`, and
  `SoilWaterTotal` residuals co-occur.
