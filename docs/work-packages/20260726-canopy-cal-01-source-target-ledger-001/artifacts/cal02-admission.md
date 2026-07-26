# CAL-02 Admission

Verdict: `READY_BOUNDED`

CANOPY-CAL-02 may execute a bounded, deterministic common-forcing analytical
reconstruction. It must copy and hash-bind the complete existing fixture
`tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/`, whose `p10.run`
encodes the 1980–2024 period. The fixture's physical inputs are the only
admitted forcing surface.

The admitted arms are:

- the fixture unchanged, including its exact seasonal-deciduous `p10.man`, as
  the fixture-baseline control;
- the same physical forcing with Bill's delivered hardwood management
  (`dropfc=0.95`);
- the same physical forcing with a branch derived from that delivered file by
  changing only `dropfc` to `0.92`; and
- the same physical forcing with Bill's delivered Santee mixed-forest
  management.

Only management replacement, the single `dropfc` branch edit, and run-file
relative-path rewrites required by an isolated CAL-02 fixture are authorized.
CAL-02 must record source and destination SHA-256 identities before running.
No calibration or other operand edit is admitted.

It may not claim an exact rerun. Bill’s exact climates, converted soils/slopes,
constant-cover files, run controls, executable identity, machine outputs, and
return-period inputs were not delivered. The Santee arm is not a site-specific
Santee reconstruction: using the Hubbard physical fixture isolates management
process behavior under common forcing. The fixture `p10.man` describes a
seasonal deciduous canopy with winter retention and is not constant cover or a
substitute for Bill's missing WEPP Windows comparator. The 45-year fixture is
not a substitute for Bill's 100-year stochastic realization or reported
equilibrium horizon.

The reconstruction must:

- execute both Hubbard `dropfc=0.92` and `dropfc=0.95`;
- classify Yang `7.6 Mg/ha` as standing foliage biomass, not annual leaf fall;
- keep foliage/needle and woody litter distinct wherever native diagnostics
  permit;
- keep forest-floor stock separate from total fuel;
- keep hillslope surface runoff and sediment separate from watershed discharge
  and channel sediment;
- exclude the two AI-attributed field values; and
- make no parameter-selection or production-default change.

Bill's chart and table values may be used to expose direction and discrepancy,
but not as pass/fail equivalence targets under substituted forcing. A
site-specific Santee experiment or a 100-year/result-table reproduction
requires a new superseding admission record.

CAL-02's scaffolded five-arm language currently requires Hubbard and Santee
constant-cover arms. Before execution CAL-02 must consume this admission and
amend those requirements to `BLOCKED_SOURCE_BUNDLE`, unless a superseding
CAL-01 record admits a real hash-bound constant-cover management. The
deciduous fixture baseline must never be relabeled constant cover.

The machine-readable authority is `cal02-admission.json`.
