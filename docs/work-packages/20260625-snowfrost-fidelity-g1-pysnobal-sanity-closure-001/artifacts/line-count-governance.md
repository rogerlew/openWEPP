# Line-Count Governance

Evidence mode: Ran.

Ran:

```text
 1413 crates/openwepp-runner/src/hillslope/snowbench.rs
   79 crates/openwepp-runner/src/bin/openwepp-snowbench.rs
   40 crates/openwepp-runner/src/lib.rs
   16 crates/openwepp-runner/src/hillslope/mod.rs
  294 tests/integration/snowfrost_fidelity_g0_pysnobal_bridge_contract.rs
 1842 total
```

Disposition:

- No touched `.rs` file is at or above the 2000-line warning threshold.
- No touched `.rs` file is at or above the 3000-line required-refactor
  threshold.
- `tools/snowfreeze_observed/pysnobal_compare.py` is outside the Rust
  line-count rule, but the package observed that it is becoming a larger
  harness surface. Future PySnobal packages should consider splitting runner,
  summarization, and routing helpers if the file grows further.
