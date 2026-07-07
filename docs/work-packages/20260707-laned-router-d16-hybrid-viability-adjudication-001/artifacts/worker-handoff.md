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

Kill / scope-narrowing criteria:

- Predeclare these criteria in the follow-on package before execution; do not
  negotiate them after another failed tuning attempt.
- Any one of these is sufficient to abandon broad default-promotion pursuit:
  - the fidelity-savings death curve shows that shrinking implicit eligibility
    into ratified pass-sediment/hydrograph-shape tolerance removes essentially
    all surviving speedup;
  - a genuine non-bare solve-cost attempt cannot make generic non-bare
    implicit steps cheaper than the explicit steps they replace while
    preserving `INV-OFEHYB-003` determinism and Z-rating branch discipline;
  - Tier-2 5 cells/OFE mesh ratifies and re-run arithmetic shows the hybrid
    value pool is superseded;
  - this follow-on plus at most one successor fails to flip the cohort no-harm
    gate.
- Scope-narrowing comes before full abandonment: if non-bare solve-cost remains
  out of reach but bare/low-cover hybrid value remains real, retreat to
  bare-skin-only opt-in hybrid for disturbed/burned post-fire classes rather
  than default promotion.
- Do not cite these as abandonment triggers: the current cohort loss, fidelity
  deltas before first-divergent-day/OFE attribution, or any single timing
  endpoint in isolation.
- If broad hybrid or a narrowed hybrid surface is abandoned, retire the
  corresponding tests in the same package as the contract lifecycle and
  code-removal/quarantine change. The durable record of a negative result is
  the contract/work-package evidence chain, not permanent live tests for a
  dead subsystem.
- While hybrid remains experimental, tier the test surface deliberately:
  cheap invariant guards stay always-run; heavy acceptance vectors may be
  ignored with documented reproduction commands only when the package records
  the tradeoff explicitly.
