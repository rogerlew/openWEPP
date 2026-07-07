# Worker Handoff

Status: COMPLETE. Evidence mode: Static.

## First Package To Run

Suggested package:

`20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001`

Objective:

- Make hybrid no-harm before any default-promotion attempt.

Required work:

- Declare the staging objective up front:
  - either non-bare implicit solve-cost reduction is primary and the no-harm
    selector is the safety net,
  - or selector-first staging is intentional and WA/forest-class hybrid wins
    remain an explicit later hold.
- Design a preflight/adaptive selector that predicts exact-bare-skin coverage
  and generic non-bare solve-cost risk before selecting hybrid.
- Restrict the predictor to deterministic run-input classes: lane static
  cover/friction composition, source structure, mesh/cadence, and counters
  predicted from those inputs. Do not use wall time, host timing, observed
  mid-run iteration counts, or any measured-cost signal that could make outputs
  machine- or load-dependent.
- Route plain when predicted generic map-iteration cost exceeds expected
  explicit-step savings, but do not let selector-only no-harm be reported as
  closing forest/fleet hybrid viability unless non-bare solve-cost value is
  also recovered or explicitly held.
- Predeclare `SC-OFEROUTE-002` promotion tolerances before tuning evidence:
  routed outlet, hydrograph shape/peak, HBP semantic deltas, pass-sediment
  impacts, closure surfaces, and cohort weighting.
- Before tolerance ratification, run a first-divergent-day/OFE attribution on
  H2637 plain-vs-hybrid to classify the `-0.43957 %` outlet and `-6.474 %`
  pass-sediment signal.
- Add counters/tests proving the selected policy is deterministic and does not
  weaken `INV-OFEHYB-003`, `INV-OFEHYB-007`, or `INV-OFEHYB-008`.
- Rerun H2637 plus the selected cohort.

Success target:

- No selected member has a material timing regression.
- Aggregate selected timing beats active plain.
- If the pass condition is achieved by routing plain for WA/forest-like cases,
  close only a selector-first/no-harm stage and leave non-bare solve-cost
  viability open.
- A full hybrid-viability close must preserve or recover value on forest-like
  generic non-bare cases, not only H2637.
- H2637 win is preserved or the selector documents why fidelity/no-harm policy
  selects plain.
- Fidelity/tolerance surfaces are predeclared before renewed tuning or
  promotion evidence.
