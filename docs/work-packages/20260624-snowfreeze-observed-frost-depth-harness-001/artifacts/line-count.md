# Line Count Governance

Evidence class: Ran.

Status: complete.

Measure touched Rust files before closure.

Policy:

- `>=2000` lines: WARN and decomposition rationale required.
- `>=3000` lines: closure blocker unless generated/fixture exception is
  approved with owner and sunset plan.

| File | Lines | Disposition |
| --- | ---: | --- |
| `tests/integration/snowfreeze_observed_frost_depth_contract.rs` | 224 | Below warning threshold. |
| `tools/snowfreeze_observed/observed_harness.py` | 1261 | Python harness; below package concern threshold, but should be split if expanded with additional source adapters. |
