# Line-Count Governance

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r5d.rs
```

Result:

```text
2156 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
1095 crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs
 686 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r5d.rs
3937 total
```

Disposition:

- `direct_runtime.rs` remains over 2000 lines. This is an existing direct
  runtime aggregation file and is accepted with WARN disposition.
- New production implementation `direct_runtime/growth.rs` is below 2000
  lines.
- No touched non-exempt file is at or above 3000 lines.

