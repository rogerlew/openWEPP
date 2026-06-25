# Worker Handoff

Evidence mode: Static + Ran.

SNOWFROST-FIDELITY-E is complete. It redirects the next work from frost physics
to snow-depth fidelity.

Current adjudicated state:

- Sites 1, 2, and 4 have like-for-like paired physical snow-depth evidence and
  fail `TOL-SNOWFREEZE-009`.
- The dominant signed direction is modeled snow deeper than observed snow.
- Adjacent-day timing/stage checks do not explain the failures.
- WAT `Snow-Water` is SWE and remains an invalid snow-depth proxy, even though
  it is numerically closer on many rows.
- Sites 3 and 5 cannot control snow because paired observed snow-depth rows are
  absent.

Next package:

1. Scaffold a Defect-Closure package for snow-depth
   producer/carry/input/settlement adjudication.
2. Start from the modeled-over-observed direction.
3. Trace snowpack initial state/carry, snowfall depth input, density/settling,
   rain-on-snow storage, melt depletion, and publication lineage before any
   production edit.
4. Rerun the observed snow-control gates after any in-envelope correction.
5. Do not resume frost heat-flow, frozen-K/SFCC, impedance, or migration/fringe
   work until snow-depth control passes or is contract-bounded.
