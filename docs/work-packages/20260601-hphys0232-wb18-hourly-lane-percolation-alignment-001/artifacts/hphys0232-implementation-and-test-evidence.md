# HPHYS0232 Implementation and Test Evidence

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Production implementation

Static:
1. Runner lane seed now publishes `wb18_perc_lane_substeps`:
   - daily: `1`,
   - hourly: `24`.
   File: `crates/openwepp-runner/src/hillslope/mod.rs`
2. WB18 percolation kernel now:
   - reads optional `wb18_perc_lane_substeps` (default `1`),
   - enforces `>=1` integral domain,
   - attenuates routed seepage as `pei = pei_unscaled / lane_substeps`.
   File:
   `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`

## Test execution

Ran:
1. `cargo test -p openwepp --test wb18_percolation_physics_kernel_contract`
2. `cargo test -p openwepp-runner hphys0232_wb11_seed`
3. `cargo test -p openwepp --test auth03_level4_constitutive_gate_contract --test auth05_level4_constitutive_authority_hardening_contract`

Observed:
- all listed tests pass.
- new WB18 vectors pass:
  - hourly lane attenuation (`daily/24`),
  - non-positive lane-substeps hard-fail.
- new runner vectors pass:
  - daily lane seeds `wb18_perc_lane_substeps=1`,
  - hourly lane seeds `wb18_perc_lane_substeps=24`.

## Cohort rerun

Ran:
1. `H1..H39` candidate rerun against `unpalatable-rind` runfiles.
2. semantic comparator batch over baseline partitions and candidate parquet.

Evidence root:
- `/tmp/hphys0232_20260601T201921Z/parity/`

Observed:
- execution coverage: `39/39` hillslopes (`rc=0`).
- semantic coverage: `39/39` reports (`rc=0`).
- summary metrics for hold columns remain unchanged from HPHYS0231 at printed
  precision (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`).
