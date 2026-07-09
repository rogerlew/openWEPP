# Source Map

Status: `EXECUTED-COMPLETE`
Evidence: `Static`

## Producer Surfaces

- Direct groundwater recurrence:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs`
- Direct publication groundwater operands:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- HBP writer:
  `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
  and `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`

## Consumer Surfaces

- HBP parser:
  `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
  and `crates/openwepp-input-contract/src/parsers/hbp/types.rs`
- Watershed pass inventory:
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- Watershed runtime contribution:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- Watershed channel kernel:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
