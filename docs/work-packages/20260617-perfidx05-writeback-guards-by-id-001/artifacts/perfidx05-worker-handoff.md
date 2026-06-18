# PERFIDX05 Worker Handoff

Status:
- Code changes are implemented and gates pass.
- Behavior-preserving identity evidence is clean.
- Realized speed is negative on the final anchor except OFE1.
- One decomposition prefix-scan residual remains.

Important files:
- `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/phase.rs`

Follow-on recommendation:
- Profile the final binary before further edits.
- Start with `ensure_no_overflow_indexed_symbols_for_decomposition` and the dual logical
  plus indexed writeback mutation cost.
- Keep legacy sidecar discovery flags aligned with PERFIDX04 when rerunning anchors.
