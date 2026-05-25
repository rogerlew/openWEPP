# REFACTOR001 Public API Surface Parity Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Legacy export inventory (`HEAD` pre-refactor `crates/openwepp-runner/src/lib.rs`) was compared to post-refactor facade exports.

Legacy public surface families identified:
- constants:
  - `BINARY_RELEASE_SCHEMA_ID`
  - `HILLSLOPE_RUN_MANIFEST_SCHEMA_ID`
  - `HILLSLOPE_RUNFILE_SCHEMA_ID`
  - `REQUIRED_RUN_OUTPUT_PASS`
  - `REQUIRED_RUN_OUTPUT_LOSS`
  - `SIMPIPE_GUARD_ID`
  - `SIMOUT_GUARD_ID`
  - `WUI_MODE_GUARD_ID`
  - `SIMMODE_TIMESTEP_GUARD_ID`
  - `SIMCONS_INTAKE_GUARD_ID`
  - `SIMCOUP_GUARD_ID`
  - `DAILY_EXECUTION_LANE`
  - `HOURLY_EXECUTION_LANE`
  - `SUBHOURLY_EXECUTION_LANE`
  - `SCHEDULER_KERNEL_PUBLICATION_SOURCE`
  - `WB13_PUBLICATION_SOURCE_SIMULATION_OWNED`
  - `WB13_REPLAY_CANDIDATE_SURFACE_WAT`
  - `WB13_REPLAY_CANDIDATE_SURFACE_PASS`
  - `SIMIMPL09_ADOPT_PROFILE`
- public enums:
  - `SidecarPolicy`
  - `BinaryRole`
  - `ReleaseMetadataError`
  - `ReleaseLintError`
  - `RunnerError`
  - `HillslopeCliError`
- public structs:
  - `RunnerLaunchRequest`
  - `HillslopeRunRequest`
  - `HillslopeRunReport`
  - `ReleaseLintReport`
- public functions:
  - `build_hillslope_argv`
  - `launch_hillslope`
  - `lint_release_directory`
  - `write_release_sidecar_for_binary`
  - `validate_release_sidecar`
  - `execute_hillslope_run`

Post-refactor `lib.rs` re-exports exactly these families from split modules:
- `pub use constants::*;`
- `pub use policy::SidecarPolicy;`
- `pub use role::BinaryRole;`
- `pub use errors::{...};`
- `pub use api::{...};`
- `pub use launch::{...};`
- `pub use release::{...};`
- `pub use hillslope::execute_hillslope_run;`

Conclusion:
- Public API surface parity preserved for the previously exported runner API.

## Ran
Validation proving downstream compatibility:
1. `cargo test -p openwepp-runner --tests`
   - result: pass
2. `cargo test --workspace`
   - result: pass
