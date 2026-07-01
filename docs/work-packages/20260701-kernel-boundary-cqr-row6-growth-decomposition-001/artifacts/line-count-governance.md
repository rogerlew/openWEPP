# Line-Count Governance

Evidence mode: Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/decomposition.rs
```

Result:

```text
 1584 crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs
  785 crates/openwepp-hillslope-orchestrator/src/direct_runtime/decomposition.rs
 2369 total
```

## Disposition

Status: `PASS`.

All row #6 touched or owned Rust files remain below the 3000-line governance
threshold. No line-count exception is used for this row.
