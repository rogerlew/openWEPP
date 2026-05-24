# WB18 Contract Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Scope
Implemented canonical WB18 percolation authority amendments so percolation
physics is defined by layer-aware contract equations/guards, not WB11 scalar
surrogate reduction.

## Contract Files Amended
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`

## WB18 Contract Changes
- Replaced WB11 surrogate authority in `SC-PERC-001` with WB18 layer-state and
  per-layer flux authority:
  - `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`,
    `wb18_perc_ssc_####`, `wb18_perc_pei_####`
  - bottom-up per-layer routing semantics and conductivity-domain factors.
- Added WB18 constants/branch authority in `SC-PERC-001`:
  - `Bi = 1.0`
  - `fx_min = 0.002`
  - `Delta_t = 86400 s`
- Updated WB11-percolation guard IDs and invariant obligations to WB18
  per-layer symbol authority while preserving typed guard posture.
- Updated `SC-WATBAL-001` hydrology lane authority to WB17 ET + WB18
  percolation + WB11 lateral/drainage.
- Updated registry notes in `index.md` for WB18 percolation authority.

## Version Bumps
- `SC-PERC-001`: `5 -> 6`
- `SC-WATBAL-001`: `19 -> 20`
