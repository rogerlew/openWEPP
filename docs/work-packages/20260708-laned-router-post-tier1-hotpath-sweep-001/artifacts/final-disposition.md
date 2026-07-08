# Final Disposition

Status: `EXECUTED-COMPLETE-POST-TIER1-HOTPATH-SWEEP`
Evidence mode: Static/Ran.

## Outcome

The bounded post-Tier1 explicit-router hotpath sweep is complete.

Landed:

- retained max celerity and first max-cell index from `prepare_step_alpha()`;
- removed the duplicate wet-cell Courant evidence scan;
- delayed additive-path slope square-root work until after pure-skin branch
  selection;
- added focused retained-max unit coverage.

Not changed:

- no hybrid implicit work;
- no mesh, fidelity, or tolerance readjudication;
- no `SC-OFEROUTE-001` amendment;
- no `Re^0.45` approximation;
- no watershed/channel/baseflow/sediment work.

## Evidence Summary

- Focused kinematic-wave tests: `27/27` passed.
- Focused cascade tests: `6/6` passed.
- Full workspace nextest: `1437/1437` passed, `3` skipped.
- `cargo fmt --check`, clippy, `cargo deny check`, `git diff --check`, and
  scoped markdown lint passed.
- H2637 timing median: `11.72 s` user, versus Tier1 `11.90 s`.
- H2637 profile: `solver_cfl_ns=2277134095` versus Tier1 `2488591327`.

## Remaining Work

WSHED-W7R remains the next queued production-path package. Future hotpath work
should not be split into another package unless it has a concrete low-risk
implementation target; the unratified `Re^0.45` envelope remains the only
named Tier1 hold.
