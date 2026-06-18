# Base Conductivity Handoff

Evidence class: Static + Ran

Handoff verdict: create a defect-closure follow-on for vertical `ssc`
normalization. Do not change hourly `wb19_lateral_ssh` to harmonic.

## Proposed Follow-On

Proposed package id:

`20260618-basecond01-ssc-harmonic-normalization-defect-closure-001`

Objective:

Fix openWEPP's vertical percolation conductivity projection so
`wb18_perc_ssc_####` follows baseline source intent:

- accumulate vertical conductivity by inverse-conductivity weighting across
  source-layer splits;
- finalize `ssc_m_s` as normalized layer thickness divided by the inverse
  conductivity accumulator;
- keep hourly `lateral_ssh_m_s` as thickness-weighted arithmetic
  `ksat * ui_anisrt`;
- keep HPHYS0257 runtime behavior: modern hourly WB19 lanes consume
  `wb19_lateral_ssh_####`, daily/vertical consumers consume
  `wb18_perc_ssc_####`.

## First Implementation Target

`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`

The likely target is `legacy_normalize_conductivity_layers_to_200mm`.

Current behavior:

- arithmetic `weighted_ksat_mm_h` feeds `ssc_m_s`;
- arithmetic `weighted_lateral_ksat_mm_h` feeds `lateral_ssh_m_s`;
- split-layer H2637 layer 3 publishes both as `270.8259 mm/h`.

Required behavior:

- vertical `ssc_m_s` for H2637 layer 3:
  `117.955408163210 mm/h` or `0.0000327653911564473 m/s`;
- hourly `lateral_ssh_m_s` for H2637 layer 3:
  `270.8259 mm/h` or `0.0000752294166666667 m/s`.

## Required Tests

Add non-aliased tests where arithmetic and harmonic differ:

- source layers: `160 mm @ 330.2755 mm/h` plus `40 mm @ 33.0275 mm/h`;
- expected vertical `ssc`: `117.955408163210 mm/h`;
- expected hourly `ui_ssh` with anisotropy `1.0`: `270.8259 mm/h`;
- assert `wb18_perc_ssc_0003 != wb19_lateral_ssh_0003`;
- add an anisotropy case where `ui_anisrt != 1.0` to prove horizontal
  `ui_ssh` tracks `ksat * ui_anisrt` and does not double-apply profile
  anisotropy.

Also add a regression around unchanged non-split layers:

- 0-200 mm `60 mm/h` remains `60 mm/h`;
- 200-400 mm `330.2755 mm/h` remains `330.2755 mm/h`;
- deep homogeneous `33.0275 mm/h` layers remain unchanged.

## Rerun Evidence

The defect-closure package should record:

- focused runtime projection tests;
- `cargo fmt --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo deny check`;
- H2637 no-UI rerun with WAT/PASS checksums and aggregate
  `runvol`, `sbrunv`, `latqcc`, and peak `latqcc`;
- if practical, H2637 with-UI rerun to confirm the conductivity split does not
  regress modern hourly publication.

## Guardrail

Do not implement the closure by replacing both conductivity surfaces with a
single harmonic value. That would close the vertical `ssc` defect by creating a
new hourly `ui_ssh` defect under `SC-SUBHYD-001` HPHYS0257.
