# Gate Results

Status: `HOLD checkpoint`.

- `cargo fmt --all -- --check`: PASS.
- `cargo check -p openwepp-vegetation -p openwepp-persisted-restart-v1 -p openwepp-hillslope-orchestrator`: PASS.
- vegetation + persisted-restart nextest: 297/297 PASS.
- V11 orchestrator focused population: 4 PASS, 1 FAIL.
- sole focused failure: one-nanosecond actual LSE solve,
  `LsebE034 / IterationLimit`, 50 iterations and 736 backtracks.
- 600+1200, 1200+600, three unequal, forcing-order, full-support physical
  compatibility, and rollback cases: PASS.

Exact numerical diagnostics and reverted prototype evidence are in
`tiny-support-lse-authority-blocker.md`. This is not a terminal package gate
record; broad closure gates and final review/verification remain pending.
