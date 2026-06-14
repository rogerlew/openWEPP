# Line Count Governance Checklist

Status: W-B executed

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
