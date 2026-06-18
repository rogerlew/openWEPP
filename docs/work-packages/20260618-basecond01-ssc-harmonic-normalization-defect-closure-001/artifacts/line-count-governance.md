# Line-Count Governance

Evidence class: Ran

Status: complete.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs \
  crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/soil.rs \
  tests/integration/parser_runtime_seam_integration/common.rs
```

Result:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs` | 1729 | OK, below 2000 WARN threshold |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/soil.rs` | 781 | OK, below 2000 WARN threshold |
| `tests/integration/parser_runtime_seam_integration/common.rs` | 752 | OK, below 2000 WARN threshold |

No 2000+ touched Rust files and no 3000+ non-exempt touched Rust files.
