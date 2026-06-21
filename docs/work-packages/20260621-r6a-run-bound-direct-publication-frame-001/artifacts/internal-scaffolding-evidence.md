# Internal Scaffolding Evidence

Status: implemented.
Evidence mode: Static + Ran.

## Current Failure Evidence

Static inspection before implementation found:

- `DirectPublicationFrame` currently contains only `runoff_m`,
  `infiltration_m`, `evapotranspiration_m`, `drainage_m`, and
  `lateral_flow_m`.
- `select_direct_runtime_skeleton_once` constructs `DirectRunFrame::skeleton`
  for explicit direct modes; skeleton execution cannot satisfy R6A acceptance.
- HBP currently reads `HillslopeWritebackSurface` through `build_hbp_output`.
- WAT rows currently build from `execution.wb13_rows`.
- PASS rows currently build from `execution.pass_rows` derived from WB13/outlet
  rows.
- loss JSON and manifest outputs are not direct publication frame consumers.

Commands used during R6 resumed hold analysis:

```text
rg -n "PublicationFrame|publication\.manifest|publication\.runoff|publication\.storage|publication\.loss|DirectPublication|direct publication|PublicationOperands" crates docs/architecture/array-native-runtime-specification.md --glob '!target/**'
rg -n "fn build_hbp_output|runtime_surface: &HillslopeWritebackSurface|build_hillslope_wat_rows\(&execution\.wb13_rows|write_hillslope_pass_parquet\(|build_loss_output_json\(|write_hillslope_run_manifest\(" crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs
```

## Scaffolding Required Before Closure

R6A cannot close until the following internal scaffolding exists:

- frame schema that covers promoted R6 ledger fields;
- constructor inputs typed from direct run/lane/day state;
- runner handoff for real parsed run dimensions;
- direct projection consumers for all five output families;
- source scan that proves direct projection consumers avoid compatibility
  publication paths;
- focused anti-alias and independent reconstruction tests;
- default-disabled test proving no direct publication frame construction.

## Implemented Scaffolding

- `DirectRunPublicationFrame` and `DirectPublicationDayRow` provide the
  run-bound frame schema.
- `DirectFrameExecutor::run_publication_capture` captures typed direct
  run/lane/day state during direct span execution.
- `HillslopeRuntimeSelection::DirectPublicationFrameShadow` provides an
  explicit runner handoff over real parsed slope OFE counts and climate day
  spans.
- Direct HBP/WAT/PASS/loss/manifest projection consumers take
  `&DirectRunPublicationFrame`.
- Source scans over the direct builder/projection ranges found no forbidden
  compatibility source reads.
- Focused tests prove default-disabled compatibility constructs no direct
  publication frame and the opt-in path does not use the old skeleton counter as
  closure evidence.
