# Coverage After

Evidence label: Static/Ran.

Status: `PROVISIONAL-ROLLED-BACK-NOT-CLOSURE`

Provisional focused command:

- `cargo llvm-cov clean --workspace && cargo llvm-cov -p openwepp-watershed-orchestrator --lcov --output-path /tmp/openwepp-cqr-nightly-04-ws20-focused.lcov` - exit `0`.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs`

LCOV line coverage:

- `LF:1364`
- `LH:817`
- Line coverage: `59.89736070381232%`

Region coverage:

- `NOT AVAILABLE` from LCOV.

Coverage changed because provisional module-local characterization tests were
added in the target file and production helper lines were extracted.

Hold disposition:

- This measurement is retained only as evidence for the local hold decision.
- It is not current-tree closure evidence because the implementation and tests
  were rolled back after review.
- Current target source is back to the scaffold line count and baseline
  implementation state.
