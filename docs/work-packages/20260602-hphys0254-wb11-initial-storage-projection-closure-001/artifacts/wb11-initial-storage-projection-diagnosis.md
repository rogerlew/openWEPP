# WB11 Initial Storage Projection Diagnosis

Status: complete

Evidence mode: static

Static:

- HPHYS0253 localized the H1 day-1 storage defect upstream of scheduler phases: pre-patch candidate post-seed WB11 storage was `323.346740 mm` against a baseline WAT-derived t=0 proxy of `343.500000 mm`.
- Static inspection showed the runtime seed used parser-layer depth/field symbols where baseline-authoritative hydrology seeding requires normalized corrected-layer hydrology geometry spanning the full profile.
- Directly redefining generic `nsl`/`dg_####`/theta aliases as hydrology-normalized symbols conflicted with parser and constitutive authority tests.
- The corrected authority split is:
  - Generic `nsl`, `dg_####`, `solthk_####`, `thetfc_####`, and `thetdr_####` remain parser/external-authority symbols.
  - Hydrology seed/runtime aliases use `wb11_nsl`, `wb19_dg_####`, `wb19_solthk_####`, `wb19_thetfc_####`, `wb19_thetdr_####`, `wb19_por_####`, `wb19_coca_####`, `cpm_####`, `ssc_####`, and WB18 percolation aliases.
- WB18 lower-layer percolation exposed a secondary baseline-contract defect: non-bottom lower-layer `stu=(st+frzw)/ul` values at or above `0.95` must be capped to `0.95` before `sqrt(1-stu)` rather than hard-failing finite over-UL ratios.

Result:

- The package corrected WB11 seed projection lineage without heuristic storage compensation.
- H1 post-seed WB11 storage now aligns to baseline inferred t=0 within `+0.015748 mm`.
