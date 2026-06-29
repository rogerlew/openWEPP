# Frost Rubric Results

Status: `FAIL-NON-PROMOTION`

The frost observation-corpus primary gate compares the opt-in layered insulation
handoff against the bulk snow depth/density handoff on forcing-robust frost
signatures.

Ran:

- Command: `.venv/bin/python tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py --mode frost --hill-binary target/debug/openwepp-cli-hill`
- Artifact: `artifacts/paradigm2-stage2-frost-rubric.json`
- Runtime: real direct-production executor, five normalized frost-observation
  sites, two model arms.

Primary forcing-robust frost profile:

| Model | Robust fails | Robust score | Primary counts |
|---|---:|---:|---|
| `stage1_layered_density_bulk_snow_frost_handoff` | `3` | `49` | `{'fail': 3, 'marginal': 3, 'pass': 2, 'strong': 14}` |
| `stage2_layered_resistance_v1` | `3` | `49` | `{'fail': 3, 'marginal': 3, 'pass': 2, 'strong': 14}` |

Cell deltas:

- Primary forcing-robust cells improved: `0`
- Primary forcing-robust cells worsened: `0`
- Limited report-only cells improved: `1` (`site1_sleepers_south_field_vt`
  `frost_depth_timeseries`)
- Limited report-only cells worsened: `1` (`site4_ggd498_morris_mn`
  `frost_max_depth_bias`)

Disposition: primary gate failed because the opt-in candidate did not improve
forcing-robust frost fidelity versus the bulk handoff. No activation is
authorized.
