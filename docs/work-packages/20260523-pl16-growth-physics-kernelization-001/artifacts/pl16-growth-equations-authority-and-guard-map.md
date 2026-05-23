# PL16 Growth Equations Authority and Guard Map

Status: `complete`
Evidence mode: `Static`

## Equation Component Map

| component | key symbols | implementation surface | guard posture | contract authority |
|---|---|---|---|---|
| Daily heat units and cumulative phenology | `tmax`, `tmin`, `btemp`, `gddmax`, `sumgdd` | `compute_equation_growth_state_surface` | hard-fail on missing/non-finite/domain-invalid inputs | `SC-PLANT-001` `INV-PLANT-019`, PL16 algorithm steps 4-5 |
| Temperature and water regulation | `otemp`, `Ws`, `gdd`, `temstr` | `compute_equation_growth_state_surface` | `Ws in [0,1]`, `otemp > btemp`, finite trigonometric outputs | `SC-PLANT-001` `INV-PLANT-021` |
| Radiation-driven biomass increment | `rad`, `extnct`, `beinp`, `lai` | `compute_equation_growth_state_surface` | PAR expression must be finite and non-negative | `SC-PLANT-001` PL16 steps 7-8 |
| Senescence decline branch | `dlai`, `dropfc`, `decfct`, `spriod` | `compute_equation_growth_state_surface` | decline rates must remain bounded; non-negative state preservation | `SC-PLANT-001` `INV-PLANT-020` |
| Harvest index update | `hi`, `fphu`, `Ws` | `compute_equation_growth_state_surface` | bounded to `[0, hi]` | `SC-PLANT-001` PL16 step 9 |
| Canopy and LAI update | `bb`, `xmxlai`, `vdmt`, `hia` | `compute_equation_growth_state_surface` | finite denominator/output checks; `cancov <= 0.999` | `SC-PLANT-001` `INV-PLANT-018` |
| Root mass and root depth update | `rsr`, `rtmmax`, `rdmax`, `solthk`, `dlai` | `compute_equation_growth_state_surface` | non-negative finite root state, capped by `min(rdmax,solthk)` | `SC-PLANT-001` PL16 step 12 |

## Required-Symbol Guard Surface

`require_growth_equation_inputs` enforces typed hard-fail guards for:
- climate forcing: `tmax`, `tmin`, `rad`
- water stress: `Ws`
- soil envelope: `solthk`
- projected slot/crop parameters: `btemp`, `otemp`, `gddmax`, `dlai`, `dropfc`, `decfct`, `spriod`, `bb`, `beinp`, `extnct`, `hi`, `xmxlai`, `rsr`, `rtmmax`, `rdmax`

No silent defaults are used when symbols are missing/non-finite/out-of-domain.
