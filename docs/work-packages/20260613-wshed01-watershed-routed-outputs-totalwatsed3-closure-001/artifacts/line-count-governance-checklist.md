# Line Count Governance Checklist

Status: W-C executed

Evidence mode: Ran

W-B edited the impoundment parser and runner test only. No production file
crossed the 2000-line warning threshold.

Observed line counts after W-B:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2031 | WARN; W-B did not edit this file. W-C should avoid growing it further or plan split. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | 1431 | Test file below WARN. |
| `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs` | 1390 | Below WARN. |
| `crates/openwepp-watershed-output/src/writers.rs` | 1712 | Below WARN, close to threshold. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs` | 1934 | Below WARN, close to threshold. |

Command:

```bash
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs \
  crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs \
  docs/contracts/openwepp-watershed-runfile-contract.md
```

Observed line counts after W-C:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2066 | WARN; W-C limited growth and moved WAT aggregation to `watershed_wat.rs`. |
| `crates/openwepp-runner/src/watershed_wat.rs` | 574 | New module below WARN. |
| `crates/openwepp-watershed-output/src/writers.rs` | 1904 | Below WARN, close to threshold. |
| `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs` | 2029 | WARN test file; no production refactor required. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | 1404 | Below WARN. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs` | 1102 | Below WARN. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs` | 931 | Below WARN. |

Command:

```bash
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/src/watershed_wat.rs \
  crates/openwepp-watershed-output/src/writers.rs \
  tests/integration/ws11_channel_routing_physics_equivalence_contract.rs
```
