# Source Map

Status: `EXECUTED-COMPLETE`
Evidence: `Static`

## Parser Surfaces

- HBP public parser API:
  `crates/openwepp-input-contract/src/parsers/hbp/mod.rs`
- HBP parser types:
  `crates/openwepp-input-contract/src/parsers/hbp/types.rs`
- HBP payload validator:
  `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`

## Consumer Surfaces

- Watershed pass inventory:
  `crates/openwepp-runner/src/watershed_supervisor.rs`
- Watershed routing-input construction:
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`

## Tests

- Parser contract tests:
  `tests/integration/infile_hbp_parser_contract.rs`
- Watershed CLI behavior tests:
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
