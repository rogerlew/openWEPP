# REFACTOR020 Line-Count Governance Checklist

Status: complete
Evidence mode: Static/Ran

Static:
- Pre-scope baseline: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs` at 2559 lines.
- `.rs` line-count governance threshold requires module decomposition above 2000 lines.
- Post-scope façade is 9 lines.
- Post-scope sharding totals: `common.rs` 242, `soil.rs` 660, `slope.rs` 79, `management.rs` 624, `climate.rs` 951.
- Total split-source lines: 2556 (excluding the deleted closing brace moved into façade), satisfying decomposition intent and keeping module ownership explicit.

Ran:
- 2026-06-08T23:13:29Z: `wc -l` on target files captured for closure evidence.

## Governance Targets
- Reduce `08_tests.rs` below 2000 lines while preserving behavior and test names.
