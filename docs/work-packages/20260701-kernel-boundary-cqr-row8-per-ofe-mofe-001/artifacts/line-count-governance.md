# Line-Count Governance

Evidence mode: Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs
```

Result:

```text
  2087 crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs
   736 crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs
  2823 total
```

## Disposition

Status: `PASS`.

All row #8 touched or owned Rust files remain below the 3000-line governance
threshold. No line-count exception is used for this row.
