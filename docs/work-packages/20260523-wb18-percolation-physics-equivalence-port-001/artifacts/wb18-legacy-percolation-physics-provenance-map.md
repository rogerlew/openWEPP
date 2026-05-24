# WB18 Legacy Percolation Physics Provenance Map

Status: `completed`
Evidence mode: `Static`

## Baseline Authority
- Legacy baseline worktree:
  - `/workdir/wepp-forest_260430_baseline`
- Pinned commit:
  - `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Legacy Source Touchpoints Inspected
- `src/perc.for`
  - percolation phase entry/dispatch posture.
- `src/purk.for`
  - layer-routing mechanics, conductivity scaling behavior, and bottom-up
    percolation sequencing authority.
- `src/watbal.for`
  - hydrology lane integration posture for percolation outputs.

## WB18 Runtime Mapping
- Legacy layer water content terms -> `wb18_perc_theta_####`
- Legacy field-capacity terms -> `wb18_perc_fc_####`
- Legacy upper-limit terms -> `wb18_perc_ul_####`
- Legacy saturated-conductivity terms -> `wb18_perc_ssc_####`
- Legacy per-layer transfer terms -> `wb18_perc_pei_####`
- Legacy deep seepage aggregate -> `D` and `Pe`

## Provenance Notes
- WB18 implementation follows bottom-up routing and conductivity-domain
  scaling posture from legacy percolation routines.
- Canonical authority for openWEPP behavior is in `SC-PERC-001` and companion
  `SC-*` contracts; this map records legacy traceability, not authority
  replacement.
