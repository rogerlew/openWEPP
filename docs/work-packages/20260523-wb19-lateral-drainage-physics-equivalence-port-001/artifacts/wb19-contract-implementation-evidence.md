# WB19 Contract Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Scope
Implemented canonical WB19 lateral/drainage authority amendments so
lateral/drainage behavior is governed by layer-aware equation families and
explicit geometry/guard domains rather than WB11 fraction-split surrogates.

## Contract Files Amended
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`

## WB19 Contract Changes
- Promoted `SC-SUBHYD-001` algorithm authority from WB11 surrogate symbols
  (`wb11_lateral_fraction`, `wb11_drainage_fraction`) to WB19 layer-aware
  inputs:
  - `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ssc_####`,
    `dg_####`, `avgslp`, `slplen`, `wb19_lateral_anisotropy_ratio`,
    `wb19_drain_enabled`, `wb19_drain_depth`, `wb19_drain_spacing`,
    `wb19_drain_diameter`, `wb11_drainage_coefficient`.
- Added explicit WB19 algorithm obligations in `SC-SUBHYD-001` for:
  - Eq. [6.2.4]-style lateral flux over layer-weighted conductivity,
  - Eq. [6.2.10]-[6.2.11] drainage-flux/equivalent-depth branch,
  - top-to-bottom and tile-layer-to-surface withdrawal sequencing,
  - deterministic `Qd = q + Qdd` export with typed guard continuity.
- Updated `SC-WATBAL-001` hydrology-lane authority from WB17+WB18+WB11 to
  WB17+WB18+WB19 with branch-table, invariant, guard-map, alias, constants,
  and test-vector wording updates.
- Updated science-contract index entries for `SC-SUBHYD-001` and
  `SC-WATBAL-001` to advertise WB19 authority as the active hydrology posture.

## Version Bumps
- `SC-SUBHYD-001`: `6 -> 7`
- `SC-WATBAL-001`: `22 -> 23`
