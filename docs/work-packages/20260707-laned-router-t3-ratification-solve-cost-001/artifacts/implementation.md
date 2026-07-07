# Implementation Notes

Status: EXECUTED

## Implemented Change

Phase B amended `SC-OFEROUTE-001` to rev 31 before code edits. Phase C then
implemented the deterministic implicit solve-cost lever and profile counters.

Files changed:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`

Implemented behavior:
- Added `implicit_equilibrium_map_evaluations` and
  `implicit_branch_evaluations` to the opt-in routing profile snapshot.
- Counted every fixed-point map application inside
  `CellParameters::equilibrium_discharge_converged` when profiling is enabled.
- Counted every implicit branch residual evaluation.
- Threaded a deterministic optional warm seed into implicit cell solves from the
  same downstream march's already-solved upstream discharge (`q_in` for cells
  after the first).
- Accepted a warm seed only when finite, positive, and on the evaluated branch
  side of `Q_c`; otherwise the existing cold rev-29 branch seed is used.
- Extended the active profile stderr JSON to print the new counters.
- Added a D-val hybrid Case-4 harness that runs the active source phase
  explicitly and the post-cutoff drain phase implicitly.

Focused tests:
- Ran `cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::implicit_recession::tests::branch_warm_seed' --profile quick`
  - Result: PASS, 2/2.
- Ran `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick`
  - Result after quarantining the failed ratification gate as ignored: PASS,
    85/85, 249 skipped, 147.582 s.

No production fallback, compatibility wrapper, or physics substitution was
added.
