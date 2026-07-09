# Gate Results

Evidence label: Static/Ran.

Status: `HOLD`

Focused/local gates run on the provisional implementation:

- `cargo fmt -- crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` - exit `0`.
- `cargo nextest run -p openwepp-watershed-orchestrator` - exit `0`,
  `37 tests run: 37 passed, 0 skipped`.
- Focused LCOV/CRAP command in `artifacts/crap-after.md` - exit `0`; max
  target CRAP `30.0`, rows above `30`: `0`.
- `cargo clippy -p openwepp-watershed-orchestrator -- -D warnings` - exit `0`
  after replacing a new `needless_range_loop` in an extracted helper.

Full closure gate disposition:

- `git diff --check -- crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs docs/work-packages/20260709-cqr-nightly-04-watershed-ws20-segment-routing-001` - exit `0`.
- `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-04-watershed-ws20-segment-routing-001 --format json` - exit `0`, `0` errors, `0` warnings.
- `cargo fmt --check` - exit `0`.
- Full `cargo nextest run --workspace --profile full` rerun in
  `command-07-nextest-rerun.log` was interrupted after review changed the
  disposition to local hold; exit `130`, `1506` passed, `1` interrupted/failed,
  `20` not run. This is not passing closure evidence.
- Full clippy/deny reruns were not started after the local hold decision.

Review/hold gates:

- Dual review completed with blocking findings accepted.
- Target Rust implementation/test edits were rolled back to the scaffold state.
- Package closes in local hold; completion gates are intentionally not claimed.
