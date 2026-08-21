# Gate Results

Status: `implementation review PASS / terminal gates pending`.

- `cargo fmt --all -- --check`: PASS.
- `cargo check -p openwepp-vegetation -p openwepp-persisted-restart-v1 -p openwepp-hillslope-orchestrator`: PASS.
- vegetation + persisted-restart nextest: 297/297 PASS.
- V11 orchestrator focused population: 6 PASS, 1 ignored evidence sweep.
- minimum-support case: PASS at 600000000 ns; one-tick-below case: typed
  `SupportBelowMinimum` rejection before Newton.
- 600+1200, 1200+600, three unequal, forcing-order, full-support physical
  compatibility, and rollback cases: PASS.
- receipt KAT/order, soil/LSE owner joins, accepted/checkpoint custody, and
  Restart V3 retention/replay checks: PASS.

Exact historical numerical diagnostics and reverted prototype evidence remain
in `tiny-support-lse-authority-blocker.md`; the released decision is recorded
in `lse-support-domain-evidence.md`. Implementation Reviews A/B/C are PASS.
Heavy closure gates and dual terminal verification remain pending.
