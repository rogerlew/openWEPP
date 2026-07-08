# Fidelity Envelope

Status: designed for future bridge; no projection accepted here.
Evidence class: Static.

## Scope

This envelope is a predeclared checklist for any future package that proposes a
legacy-cropland coefficient bridge. It does not ratify coefficients, tolerances,
or implementation behavior by itself.

## Authority Gates

Before fidelity metrics are considered, a bridge must prove:

- all five static Lane D operands are produced;
- every operand has source provenance, units, and finite domain bounds;
- no operand is filled by fixture-tuned constants or silent defaults;
- missing, partial, mixed, or out-of-domain authority fails closed;
- output metadata records whether coefficients were explicit, table-produced,
  or bridge-produced.

## Fidelity Surfaces

A bridge implementation package must predeclare concrete numeric tolerances and
then evaluate at least these surfaces before tuning:

- single-OFE runoff magnitude and event timing for coefficient-projected
  cropland cases, with the reference hierarchy stated explicitly:
  source-authorized Lane D candidate vs. the package's chosen independent
  reference where available, legacy/off comparison as a compatibility control,
  and conservation/consumer closure as non-negotiable gates;
- single-OFE sediment magnitude and annual vector behavior where sediment is
  active;
- MOFE routed runoff/sediment magnitude and routed-hourly shape on selected
  coefficient-projected cases;
- active closure, positivity-clamp guard, DC01 no-double-feed guard, and routed
  hydrograph erosion-consumer closure;
- H2637-class subsurface-dominated full hillslope water balance after
  groundwater/baseflow implementation;
- explicit-disable and no-coefficient legacy validation path identity.
- stratified cohort coverage across slope, cover/residue state, random
  roughness range, storm intensity/timing class, single-OFE vs. MOFE/OFE count,
  and low-mass sediment years.

## Anti-Fitting Rules

- Do not tune to one fixture or one year.
- Freeze the coefficient-generation rule before acceptance runs. If exploratory
  data are used to design the rule, keep a separate untouched acceptance/holdout
  cohort and report it independently.
- Report all cohort members, including low-mass sediment years.
- Use candidate-vs-reference and disable/off controls appropriate to the
  surface; do not compare a tuned candidate only to the previous active result.
- Treat daily mass, hourly timing, and sediment response as separate acceptance
  surfaces.
- Any coefficient bridge that passes by altering unrelated solver, mesh,
  closure, or sediment-process tolerances is out of scope.
