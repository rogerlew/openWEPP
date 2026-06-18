# Contract Implementation Evidence

Evidence class: Static

Status: complete.

Updated contract:

- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`

Changes:

- Bumped `contract_version` to `0.1.11`.
- Added pinned baseline evidence anchor `E-WF-SOL-03` for
  `/workdir/wepp-forest_260430_baseline/src/input.for:752-761,836-844,926-928`
  at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Added `D-SOL-006` / `C-SOL-006` for the parser-to-runtime conductivity
  projection:
  - normalized layers with bottom depth `<= 0.2 m` apply the baseline top
    source-layer `ksat` rule;
  - below that top interval, vertical `ssc` / `wb18_perc_ssc_####` is
    normalized by `thickness / Σ(thickness_source / ksat_source)`;
  - hourly horizontal `ui_ssh` / `wb19_lateral_ssh_####` is the arithmetic
    thickness-weighted mean of `ksat_source * anisotropy_source`;
  - the two surfaces must not be treated as aliases.
- Added boundary export mapping for `ksat,anisotropy`.
- Added guard `G-SOL-015`.
- Added revision-history row for BASECOND01.

Protected boundary:

- `SC-SUBHYD-001` HPHYS0257 already owns modern hourly `ui_ssh` /
  `wb19_lateral_ssh_####`; this package did not change WB19 lateral equation
  authority.
