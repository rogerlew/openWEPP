# simimpl06 wb13 publication provenance map

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Publication authority map
- Execution owner:
  - `execute_daily_scheduler_kernel_lifecycle(...)`
- Publication assembly owner:
  - `build_simulation_owned_wb13_row(...)`
- Emission targets:
  - `outputs.pass` (`H5.hbp` WB13 text payload)
  - `outputs.wat` (`H.wat.parquet` surface)

## Source-of-truth conversion map
- Runtime writeback symbols consumed:
  - calendar/time: `year`, `mon`, `day`
  - climate: `prcp`, `tmax`, `tmin`
  - snow/frost: `snow.options.ssd`, `frost.runtime_ws_frz`
  - soil profile: `nsl`, `solthk`, `dg_####`, `thetfc_####`, `thetdr_####`
- WB13 scalar surface emitted:
  - `P`, `RM`, `Q`, `Ep`, `Es`, `Er`, `Dp`, `UpStrmQ`, `SubRIn`, `latqcc`,
    `Total-Soil`, `frozwt`, `Snow-Water`, `QOFE`, `Tile`, `Irr`, `Area`,
    `SoilWaterTotal`, `ProfileDepth`, `ProfilePorosityCap`,
    `ProfileFCStore`, `ProfileWPStore`

## Manifest provenance subtree
- `execution_provenance.*` (SIMPIPE closure from SIMIMPL05)
- `wb13_publication.*` (SIMOUT closure from SIMIMPL06):
  - `source = "simulation-owned"`
  - `projection_fallback_used = false`
  - `guard_id = "HS-SIMOUT-E-001"`
  - `replay_candidate_surfaces = ["interchange/H.wat.parquet", "interchange/H.pass.parquet"]`

## Guard/failure map
- Missing/non-finite/domain-invalid required runtime symbols at WB13 assembly:
  - `HillslopeCliError::RuntimeSurfaceFailure { surface: "wb13_publication", ... }`
  - detail prefix: `HS-SIMOUT-E-001`
