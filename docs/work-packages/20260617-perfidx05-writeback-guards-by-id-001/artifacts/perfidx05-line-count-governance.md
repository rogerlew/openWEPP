# PERFIDX05 Line Count Governance

Ran:
- `wc -l` on touched production and test files.

Counts:
- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`: 2688
- `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs`: 334
- `crates/openwepp-kernel-contract/src/lib.rs`: 735
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`: 2665
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`: 907
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`: 74
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs`: 58
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/phase.rs`: 897
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs`: 1131
- `tests/integration/kernel_writeback_contract.rs`: 211

Conclusion:
- Touched production files remain below the 3000-line governance threshold.
