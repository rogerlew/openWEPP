# HPHYS0247 Worker Handoff

Status: hold

Evidence mode: static + ran

Static:
- Continue from HPHYS0247 only if the scope is to inspect this package’s
  residual evidence. Open a follow-on work package for additional production
  migration beyond the touched surfaces.

Ran:
- Corrected:
  - Winter activation now follows runtime snow/frost/temperature triggers
    rather than `snow.options.snow_file_present`.
  - WB19 lateral transfer now follows baseline `meblfc`, `tdvv`, and `fffx`.
- Remaining priority order:
  1. WB18 H39 early-season percolation/deep-seepage lineage (`Dp/Pe`).
  2. WB17 plant transpiration versus soil evaporation partition (`Ep`/`Es`).
  3. Snowmelt/runoff timing parity after activation (`RM`, `Q`,
     `Snow-Water`).
  4. Residual WB19 lateral magnitude after WB18 storage is corrected.
- Primary run evidence:
  `/tmp/hphys0247_20260602T070132Z_final`.
- Primary comparator evidence:
  `/tmp/hphys0247_20260602T070132Z_final/reports/H39.semantic.json`.
