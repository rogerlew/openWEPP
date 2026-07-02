# Line-Count Governance

Status: `passed-with-warn`

Evidence mode: `Ran:`

Command:

```sh
wc -l \
  crates/openwepp-watershed-output/src/writers.rs \
  crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs \
  crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs \
  tests/integration/cli03_runner_contract_derived_tests.rs \
  tests/integration/wshedw5_typed_watershed_runtime_contract.rs \
  tests/integration/infile_watershed_structure_parser_contract.rs
```

Result:

```text
  2716 crates/openwepp-watershed-output/src/writers.rs
  2126 crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs
   751 crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs
  2468 crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs
  1374 tests/integration/cli03_runner_contract_derived_tests.rs
   536 tests/integration/wshedw5_typed_watershed_runtime_contract.rs
   864 tests/integration/infile_watershed_structure_parser_contract.rs
 10835 total
```

Disposition:

- `crates/openwepp-watershed-output/src/writers.rs`: `WARN`, below `3000`.
  W6 added direct typed publication support while preserving existing schemas.
  Follow-on split target is separating schema builders/value projection from
  writer tests after the watershed publication arc closes.
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`: existing `WARN`,
  below `3000`. W6 reduced public publication staging but did not split the CLI.
  Follow-on split target remains extracting runfile parsing/topology setup from
  the binary.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`: below
  `2000`; no closure blocker.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`: test file
  `WARN`, below `3000`; no closure blocker.
- `tests/integration/cli03_runner_contract_derived_tests.rs`: below `2000`; no
  closure blocker.
- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`: below
  `2000`; no closure blocker.
- `tests/integration/infile_watershed_structure_parser_contract.rs`: below
  `2000`; no closure blocker.

No non-exempt `.rs` file is at or above `3000` lines.
