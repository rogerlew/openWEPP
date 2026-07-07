# Hold Legitimacy Audit

Status: EXECUTED-HOLD-HYBRID-VIABILITY. Evidence mode: Static + Ran.

## Hold Condition

Hybrid default promotion is held because current evidence fails two promotion
requirements:

- no ratified production fidelity tolerance exists for the observed active
  plain-vs-hybrid publication deltas;
- the selected cohort does not meet timing no-harm.

## Evidence

Fidelity evidence:

- H2637 outlet delta: `-0.43957 %`.
- H2637 `tdet` pass-sum delta: `-1.8883 %`.
- H2637 `sedcon_1..5` pass-sum deltas: `-6.4742 %`.
- H2637 `HBP` and pass parquet hashes differ.

Timing evidence:

- Selected aggregate user time: `57.34 s` plain vs `59.95 s` hybrid
  (`+4.55 %` hybrid).
- WA Cascades forest user time: `15.65 s` plain vs `24.50 s` hybrid
  (`+56.55 %`).
- N Idaho forest user time: `0.96 s` plain vs `1.23 s` hybrid (`+28.13 %`).

Solver-profile evidence:

- H2637 hybrid: `980804` implicit steps, `0` map evaluations, `-16.31 %`
  user time.
- WA Cascades hybrid: `500560` implicit steps, `98192634` map evaluations,
  `+56.55 %` user time.

## Why This Is Outside This Package

This package is an adjudication package. Closing the hold would require one or
more of:

- implementing an adaptive no-harm selector;
- implementing non-bare implicit solve-cost reductions;
- predeclaring and ratifying production-facing fidelity/timing tolerances.

Those are substantive contract/code packages. Flipping the selector here would
violate `SC-OFEROUTE-002#INV-OFEHYB-008`.

## First Actionable Follow-On

Create and execute a hold-lift package that implements a profile-aware no-harm
selector and/or non-bare implicit solve-cost reduction, then reruns the selected
cohort and H2637 fidelity/tolerance gates.

The first implementation step should be a preflight classifier over eligible
implicit bins/cells that estimates exact-bare-skin coverage versus generic
non-bare map-iteration risk before choosing hybrid for a lane/day or run.
