# Disposition

Status: EXECUTED-HOLD-CASE4-HYBRID-LADDER

The package executed the parent ratification gates and the rev-31 implicit
solve-cost lever. It closes held because the Case-4 hybrid oracle ladder failed
the current ratified peak tolerance at every rung.

Completed:
- Scaffolded package and catalog entries.
- Amended `SC-OFEROUTE-001` to rev 31 for deterministic branch-local warm
  seeding and implicit solve-cost counters.
- Implemented the cost lever and diagnostics-only counters.
- Added and ran focused warm-seed/counter tests.
- Added a retained Case-4 hybrid ladder vector.
- Ran H2637 active hybrid release timing/profile evidence through a delegated
  comparator runner.
- Scaffolded Tier-1 and Tier-2 follow-on packages.

Hold:
- `case4_hybrid_manning_ladder_meets_iwagaki_oracle` failed with peak errors
  `22.8% / 15.5% / 10.2%` vs the ratified `5%` tolerance.
- Fidelity-tolerance ratification is blocked by that failure.
- No selector promotion/default activation claim is made.
