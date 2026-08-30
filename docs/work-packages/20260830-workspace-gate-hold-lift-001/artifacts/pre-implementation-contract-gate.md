# Pre-implementation contract gate

Status: `V31 EXPECTED RED — COMPLETE`

Evidence mode: `Static + Ran`

Static: `SC-SNOWENERGY-001` was amended from active v29 to candidate v31 before
any v31 production edit. V31 records rejected v30 as non-authoritative and
binds the terminal-one-volume phase-aware midpoint through
`INV-SNOWENERGY-055` and `OBL-SNOWENERGY-C-023`. Full captured, oracle,
boundary, refusal, closure, history, and no-publication vectors were then
authored. `fixed_point.rs`, `open_snow.rs`, and the impact map remain untouched
by v31.

Ran: because concurrent authorized work in the shared tree was temporarily
uncompilable, the unambiguous sentinel ran in an isolated shared clone of
unchanged production commit `6fa804082273c1c4340614ffc208a74a8b48e408`:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator exact_floor_terminal_phase_v31_contract_sentinel
```

Result: expected `FAIL`, exit 101, with exactly two `E0425` diagnostics:

- missing type `CoveredExactFloorTerminalPhaseSupportImageV1`;
- missing value/function `covered_exact_floor_terminal_phase_iterate_v1`.

Retained log: `/tmp/wghl_001d_v31_isolated_expected_red.log`, SHA-256
`9faae2c2810f8252c902e2543c90cd0dbb8e0f8058beaef51838dbb5efd32551`.
Canonical v31 contract SHA-256:
`02ee1fb365626db5c77e601a46f56a4f8e88dea99d7d5834ad7add029f4c740c`.
This is the required pre-implementation red; no v31 production symbol or
consumer seam exists yet.

## WGHL-FULL-001F covered no-update witness

Status: `EXPECTED RED — COMPLETE`

Evidence mode: `Ran`

- canonical contract: `SC-LANDSURFACEENERGY-001` version `13`, pre-review
  SHA-256
  `922917e963788ae10faae699ab8c6eb95180748d53a94b15aa484a34eeadfede`;
- positive contract vectors: unchanged real-consumer tests
  `interior_terminal_event_runs_covered_event_and_snow_free_remainder` and
  `interior_terminal_event_capture_reproduces_below_carrier_domain`;
- unchanged production source SHA-256:
  `ea4c8d8c1d81e6efc8c26ff012025cb89c064167880ba45344c5a4c13709040a`;
- pre-edit confirmation: `git diff --quiet --
  crates/openwepp-land-surface-energy/src/solver_covered_solve.rs` exited `0`;
- isolated unchanged-production identity:
  `021c23bc1661c1aabaca57d8d7cd14cc9e310289`;
- command:
  `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run -p
  openwepp-hillslope-orchestrator -E 'test(=v9_real_consumer_shadow::tests::interior_terminal_event_capture_reproduces_below_carrier_domain) |
  test(=v9_real_consumer_shadow::tests::interior_terminal_event_runs_covered_event_and_snow_free_remainder)'
  --no-fail-fast`;
- revised run ID after the prospective full-step-excess amendment:
  `dd87636c-6728-4b2a-b601-5e36f42eddb0`;
- result: expected `FAIL`, `0 passed / 2 failed`; resource-contended durations
  `175.484 s` and `304.523 s`;
- common failure: `LSEB-E-034`, `FinalFixedCap`, iteration `4`,
  backtracking count `20`. The complete current residual vector passes and the
  reported prospective governed steps are below their unchanged thresholds;
  owner rollback hashes are exact before/after matches.

The run used an isolated shared clone because an independent contract-first
worker had intentionally uncompilable expected-red tests in the shared
orchestrator source during this gate. The clone contains the exact unchanged
production commit; the canonical v13 amendment was already present in the
shared authoritative worktree before execution. No production LSE source was
edited until this evidence was recorded.

The initial narrower domain-invalid-only predicate was not sufficient for the
authentic failure. A temporary, removed diagnostic run established that its
full `b=0` trial was domain-valid but hydraulic step
`1.2616934700542904e-7 mm` exceeded the unchanged `1e-7 mm` threshold, while
the first `b=1` halving would be domain-valid and below every governed
threshold. The parent prospectively amended the package before this expanded
behavior was implemented. No diagnostic print remains in production.
