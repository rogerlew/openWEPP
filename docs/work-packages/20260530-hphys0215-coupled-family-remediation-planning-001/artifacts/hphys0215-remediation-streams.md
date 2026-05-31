# HPHYS0215 Remediation Streams

Status: completed
Evidence mode: Static + Ran

## Inputs consumed
- HPHYS0214 disposition:
  `docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214_disposition.md`
- HPHYS0214 residual matrix:
  `docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214-residual-gap-matrix.md`
- HPHYS0211 root-cause ledger:
  `docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/hphys0211-residual-gap-matrix.md`
- Integrated diagnostics:
  `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.json`

## Blockers requiring HPHYS0216+ remediation
- `ProfileFCStore`: `27/39` fail hillslopes.
- `Dp`: `39/39` fail hillslopes.
- `latqcc`: `39/39` fail hillslopes.
- `Total-Soil`: `39/39` fail hillslopes.
- `SoilWaterTotal`: `39/39` fail hillslopes.

## Stream decomposition (approved queue blueprint)
| Stream ID | Follow-on package | Families | Contract authority | Primary write set | Objective closure criteria |
| --- | --- | --- | --- | --- | --- |
| `HP215-S1` | `HPHYS0216` | `ProfileFCStore` | `SC-WATBAL-001`, `SC-PERC-001`, `SC-SOIL-001` | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`; `docs/specifications/science-contracts/contracts/SC-PERC-001.md`; `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`; `crates/openwepp-runner/src/hillslope/mod.rs` | remove structural lane split (`all-pass/all-fail`) and reduce `ProfileFCStore` fail hillslopes from `27/39`; preserve `thetfc_####`/`thetdr_####` layer-authority publication invariants and typed guards. |
| `HP215-S2` | `HPHYS0217` | `Dp` | `SC-WATBAL-001`, `SC-PERC-001`, `SC-SYSTEM-001` | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`; `docs/specifications/science-contracts/contracts/SC-PERC-001.md`; `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`; `crates/openwepp-runner/src/hillslope/mod.rs` | close persistent `Dp` publication drift with process-authoritative percolation lineage validation (state carry + unit/aggregation authority), targeting `0/39` fails or contract-authorized residual disposition with explicit proof. |
| `HP215-S3` | `HPHYS0218` | `latqcc` (and coupled `Qd`, `Tile`, `SubRIn`) | `SC-WATBAL-001`, `SC-SUBHYD-001`, `SC-SYSTEM-001` | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`; `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`; `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`; `crates/openwepp-runner/src/hillslope/mod.rs` | close lateral-flow publication residuals with strict coupling invariants (`Qd = latqcc + Tile`) and runtime-symbol lineage proof, targeting `0/39` fails or authority-backed residual adjudication. |
| `HP215-S4` | `HPHYS0219` | `Total-Soil`, `SoilWaterTotal` | `SC-WATBAL-001`, `SC-SOIL-001`, `SC-SYSTEM-001` | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`; `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`; `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`; `crates/openwepp-runner/src/hillslope/mod.rs` | enforce aggregate recompute continuity from realized mutable state to WB13 publication (`wb11_soil_water -> Total-Soil -> SoilWaterTotal`) and reduce both families from `39/39` fail saturation. |
| `HP215-S5` | `HPHYS0220` | Integrated hold-lift rerun | `SC-SYSTEM-001` + all stream-touched SC docs | `docs/work-packages/README.md`; package-local artifacts; comparator scripts only if needed | rerun required gates and 39-hillslope semantic diagnostics after S1-S4; publish final process-authority-first `HOLD`/`GO` with residual ownership closure. |

## Sequencing and dependency constraints
1. `HPHYS0216` before `HPHYS0217` to settle FC threshold authority surface used
   by downstream percolation/lateral pathways.
2. `HPHYS0217` and `HPHYS0218` may run in either order if contract deltas do
   not overlap, but must both complete before `HPHYS0219`.
3. `HPHYS0219` depends on realized withdrawal/state propagation from S2/S3.
4. `HPHYS0220` starts only after S1-S4 dispositions are complete.

## Evidence obligations for each follow-on package
- Must run contract-first sequence:
  1. contract amendments (if obligations changed),
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- Must include:
  - kernel-profile compliance checklist artifact,
  - dual review + dual verification artifacts,
  - workspace gate evidence and targeted semantic diagnostics.
