# HPHYS0244 Layer State Lineage Probe

Static: source lineage inspection.
Ran: targeted source searches and line-excerpt capture.

## Direct Artifact Availability
Current emitted artifacts do not provide daily layer `st`/`theta` or WB18 `Pe`
telemetry for `H1`, `H7`, or `H39`.

Observed available evidence is therefore split:
- **Direct output evidence**: WAT `Dp`, `Total-Soil`, and `SoilWaterTotal`.
- **Static lineage evidence**: internal WB18/WB11 symbols and writeback paths
  that produce those output columns.

## Static Lineage
OpenWEPP already has internal symbols for the requested layer/flux family:
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:999`
  constructs `wb18_perc_{field}_{layer_index:04}` state symbols.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:1003`
  constructs `wb18_perc_pei_{layer_index:04}` flux symbols.
- `crates/openwepp-hillslope-orchestrator/src/constants.rs:131` defines
  percolation loss `D`.
- `crates/openwepp-hillslope-orchestrator/src/constants.rs:133` defines
  percolation recharge `Pe`.

WB18 percolation mutates the same state/flux family:
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1129`
  computes pre-scaled `pei`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1153`
  computes lane-scaled `pei`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1164`
  subtracts `pei` from layer `theta`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1177`
  assigns bottom-layer `pei` to `percolation_loss`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1214`
  writes updated `wb18_perc_theta_*`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1223`
  writes per-layer `wb18_perc_pei_*`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1232`
  writes daily `D`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs:1238`
  writes daily `Pe`.

Runner publication then consumes those lineage surfaces:
- `crates/openwepp-runner/src/hillslope/mod.rs:3086` reads `D` as the
  percolation-loss source.
- `crates/openwepp-runner/src/hillslope/mod.rs:4296` states publication
  authority for `Total-Soil` is `wb11_soil_water -> watcon -> Total-Soil`.
- `crates/openwepp-runner/src/hillslope/mod.rs:4304` converts
  `wb11_soil_water` to millimetres.
- `crates/openwepp-runner/src/hillslope/mod.rs:4420` emits `Total-Soil`.
- `crates/openwepp-runner/src/hillslope/mod.rs:4427` emits
  `SoilWaterTotal`.

## Existing Guard Evidence
The runner already has a regression test asserting daily reseed must not erase
mutable layer state:
- `crates/openwepp-runner/src/hillslope/mod.rs:6063` seeds
  `wb18_perc_theta_0001`.
- `crates/openwepp-runner/src/hillslope/mod.rs:6075` executes daily
  reconciliation seeding.
- `crates/openwepp-runner/src/hillslope/mod.rs:6085` asserts daily reseed
  preserves mutable `wb18_perc_theta` state.
- `crates/openwepp-runner/src/hillslope/mod.rs:6089` asserts
  `wb12_storage_initial` follows carried `wb11_soil_water`.

## Probe Outputs
- `/tmp/hphys0244_20260602T045926Z/static_lineage_search.txt`
- `/tmp/hphys0244_20260602T045926Z/source_line_evidence.txt`
- `/tmp/hphys0244_20260602T045926Z/source_wb11_seed_snippet.txt`
- `/tmp/hphys0244_20260602T045926Z/source_wb13_publication_snippet.txt`
- `/tmp/hphys0244_20260602T045926Z/source_daily_reseed_test_snippet.txt`

## Finding
The requested layer state is present internally as `wb18_perc_theta_*`, and
WB18 `Pe`/per-layer `pei` is present internally as flux writeback. The missing
piece is emitted diagnostic observability, not symbol existence. The next
implementation package should first expose a diagnostics-only trace at
post-seed, post-WB18, post-WB19, pre-WB13, and post-WB13 boundaries for:
`wb18_perc_theta_*`, `wb18_perc_pei_*`, `D`, `Pe`, and `wb11_soil_water`.
