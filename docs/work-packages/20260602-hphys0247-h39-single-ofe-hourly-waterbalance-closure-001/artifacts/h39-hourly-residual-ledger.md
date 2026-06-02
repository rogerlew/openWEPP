# H39 Hourly Residual Ledger

Status: hold

Evidence mode: ran

Static:
- Trace schema: `openwepp-hphys0245-wb11-wb18-trace-v1`.
- Ledger symbols:
  `wb11_soil_water`, `Σwb18_perc_theta_####`, `D/Pe`,
  WB19 `q`/`latqcc`, WB13 `Total-Soil`, WB13 `SoilWaterTotal`.

Ran:
- Trace path:
  `/tmp/hphys0247_20260602T070132Z_final/hillslope_output/H39.hphys0247.trace.jsonl`.
- Day 1 candidate ledger:
  - `post_seed`: `wb11_soil_water=363.554235 mm`,
    `Σtheta=323.143334 mm`, gap `40.410901 mm`.
  - `after percolation`: `D=Pe=22.980342 mm`,
    `wb11_soil_water=340.573894 mm`, `Σtheta=300.162992 mm`.
  - `after lateral`: `wb11_soil_water=334.062649 mm`,
    `Σtheta=294.050103 mm`, lateral delta `6.112889 mm`.
  - `post_wb13`: `Dp=22.980342 mm`,
    `Total-Soil=334.062649 mm`, `SoilWaterTotal=334.062649 mm`.
- Day 1 baseline comparison:
  `Dp=0.24 mm`, `latqcc=0.77 mm`, `Total-Soil=386.59 mm`.
- WB19 result:
  HPHYS0247 reduced the prior day-1 lateral overshoot from
  `79.515092 mm` to `6.112889 mm`, and semantic comparator max `latqcc`
  error fell from about `78.745092 mm` to `8.130000 mm`.
- Remaining dominant ledger issue:
  WB18 percolation still drains `22-24 mm/day` on days 1-4 where baseline
  reports `0.24 mm/day`, which immediately depresses soil storage and drives
  downstream `Total-Soil`/`SoilWaterTotal` residuals.
- Secondary ledger issue:
  WB17 ET partition remains non-authoritative for H39: candidate `Ep=0` on
  the first 10 days while baseline has positive transpiration, and candidate
  `Es` absorbs the ET residual.
- Snow ledger issue:
  Winter now runs with `snow_file_present=false`, but melt/runoff timing is
  not semantically aligned; `RM`, `Q`, and `Snow-Water` remain failing.
