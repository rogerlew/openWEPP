# WSHEDPLAN01 Current Surface Inventory

Status: complete

Evidence mode: static+ran

Date: 2026-05-26

## Static
- Watershed dispatch scheduling, dependency ordering, and kernel writeback
  orchestration are implemented in `openwepp-watershed-orchestrator`:
  - deterministic dependency graph planning:
    `crates/openwepp-watershed-orchestrator/src/lib.rs:1378-1552`
  - kernel execution and typed writeback protocol:
    `crates/openwepp-watershed-orchestrator/src/lib.rs:1234-1344`
- WS10 channel/impoundment kernel branches and guard semantics are implemented
  (including `ipeak` 1/2/3/4 branch selection), but this remains
  contract-scaffolded behavior rather than full baseline-routine migration:
  `crates/openwepp-watershed-orchestrator/src/lib.rs:625-1062`.
- Watershed runtime input seams are implemented for:
  - `chan.inp` to runtime surface:
    `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:499-573`
  - watershed channel symbol seeding:
    `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:586-625`
  - watershed impoundment symbol seeding:
    `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:640-682`.
- Watershed CLI orchestration intake is scaffolded and operational for
  runfile + parser + topology + HBP ingestion + kernel execution:
  - dispatch execution and CLI wrapper failure codes:
    `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:419-433`
  - runtime ingestion of HBP contributor payload:
    `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:296-417`.
- Watershed output contract path validation is implemented for all required
  parquet outputs (`14` required paths):
  `crates/openwepp-watershed-output/src/contracts.rs:5-97`.
- Watershed parquet writer is intentionally blocked; schema builders exist but
  writer returns hard failure `OWSOUT-E-004`:
  `crates/openwepp-watershed-output/src/writers.rs:12-35`.
- Runner enforces the writer guard as a typed CLI error path (`CLIWAT-E-034`):
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs:892-893,1373-1375`.
- Python orchestration boundary for watershed remains unimplemented:
  `open_wepp_runner/open_wepp_runner.py:558-587`.
- Test posture:
  - WS10/WS11/WS12 watershed kernel contract tests exist and pass.
  - CLI watershed behavior tests explicitly assert failure until data-backed
    watershed writers exist (`OWSOUT-E-004`):
    `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs:78-113`.

## Ran
- `nl -ba crates/openwepp-watershed-orchestrator/src/lib.rs | sed -n '600,840p'`
- `nl -ba crates/openwepp-watershed-orchestrator/src/lib.rs | sed -n '980,1365p'`
- `nl -ba crates/openwepp-watershed-orchestrator/src/lib.rs | sed -n '1360,1625p'`
- `nl -ba crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs | sed -n '460,760p'`
- `nl -ba crates/openwepp-watershed-output/src/contracts.rs | sed -n '1,280p'`
- `nl -ba crates/openwepp-watershed-output/src/writers.rs | sed -n '1,240p'`
- `nl -ba crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs | sed -n '320,760p'`
- `nl -ba crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs | sed -n '860,930p'`
- `nl -ba crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs | sed -n '1350,1408p'`
- `nl -ba open_wepp_runner/open_wepp_runner.py | sed -n '520,620p'`
- `nl -ba tests/integration/ws10_watershed_kernel_contract.rs | sed -n '1,320p'`
- `nl -ba tests/integration/ws11_channel_routing_physics_equivalence_contract.rs | sed -n '1,340p'`
- `nl -ba tests/integration/ws12_impoundment_physics_equivalence_contract.rs | sed -n '1,340p'`
- `nl -ba tests/integration/parser_runtime_seam_integration.rs | sed -n '420,520p'`
- `nl -ba tests/integration/parser_runtime_seam_integration.rs | sed -n '673,756p'`
- `nl -ba crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs | sed -n '1,320p'`
- `cargo test --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract --test cli04_runner_wat_parquet_contract_derived_tests`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
