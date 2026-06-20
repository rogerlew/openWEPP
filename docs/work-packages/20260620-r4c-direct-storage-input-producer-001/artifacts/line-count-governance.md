# R4C Line-Count Governance

Status: complete.
Evidence mode: Ran.

R4C must record line counts for every touched `.rs` file.

Policy:

- 2000+ lines: WARN, record explicit disposition and split/sunset plan.
- 3000+ lines: blocking for non-exempt files; split before closure.
- R4C includes a narrow storage module split to reduce `direct_runtime.rs`
  pressure before adding the storage-input producer.

## Results

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs \
  crates/openwepp-hillslope-orchestrator/src/lib.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs \
  crates/openwepp-runner/src/hillslope/03_tests.rs
```

Measured counts:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 1859 | Below WARN after storage split. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs` | 409 | New narrow storage submodule. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 103 | No concern. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | 1306 | Test file, below WARN. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 645 | No concern. |

Verdict: PASS. No touched Rust file is in the 2000+ WARN band or 3000+
blocking band.
