# Gate Results

Status: `implementation review PASS / terminal gates pending`.

- `cargo fmt --all -- --check`: PASS.
- `cargo check -p openwepp-vegetation -p openwepp-persisted-restart-v1 -p openwepp-hillslope-orchestrator`: PASS.
- land-surface-energy + vegetation + persisted-restart nextest: 361/361 PASS.
- V11 orchestrator focused population: 6 PASS, 1 ignored evidence sweep.
- minimum-support case: PASS at 600000000 ns; one-tick-below case: typed
  `SupportBelowMinimum` rejection before Newton.
- 600+1200, 1200+600, three unequal, forcing-order, full-support physical
  compatibility, and rollback cases: PASS.
- receipt KAT/order (2/2), soil/LSE owner joins, accepted/checkpoint custody,
  and Restart V3 retention/replay checks: PASS.

Exact historical numerical diagnostics and reverted prototype evidence remain
in `tiny-support-lse-authority-blocker.md`; the released decision is recorded
in `lse-support-domain-evidence.md`. Implementation Reviews A/B/C are PASS.
The full workspace quick profile remains red on pre-existing SC-SNOWENERGY /
SC-SNOWFREEZE identity drift and broad all-target Clippy remains red only on
the pre-existing `float_cmp` test; package-scoped gates pass. Dual terminal
verification and lifecycle transition remain pending.
