# Line Count Governance Checklist

Status: W-A executed

Evidence mode: Ran

No Rust files were edited in W-A, so no line-count remediation is required for
this increment.

Observed line counts for files mapped by W-A:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2031 | WARN if W-B/W-C edits it; keep changes small or plan split. |
| `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs` | 1389 | Below WARN. |
| `crates/openwepp-watershed-output/src/writers.rs` | 1712 | Below WARN, close to threshold. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs` | 1934 | Below WARN, close to threshold. |

Command:

```bash
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs \
  crates/openwepp-watershed-output/src/writers.rs \
  crates/openwepp-watershed-output/src/contracts.rs \
  crates/openwepp-watershed-orchestrator/src/lib.rs \
  crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs \
  crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs \
  crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs
```
