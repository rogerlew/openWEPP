# Worker Handoff

Evidence class: Static.

SNOWDENSITY-07 is closed COMPLETE-07-RUNTIME-OPT-IN.

Next recommended package: SNOWDENSITY-08 Snow/Frost Gate Rerun.

Recommended SNOWDENSITY-08 scope:

- Keep `legacy_wepp` default and do not add parser/runfile/CLI activation.
- Use typed callers or a diagnostic harness to compare the new
  `physics_bulk_density_compaction_v1` runtime opt-in against the existing
  SNOTEL rubric and non-SNOTEL snow-control/frost gate.
- Report whether the opt-in density path improves snow-depth/density control
  enough to unblock frost attribution.
- Preserve the CoE boundary anti-alias proof: CoE melt/liquid/SWE boundaries
  remain authoritative unless a later contract explicitly changes melt.
- Do not tune coefficients, radiation, albedo, canopy, or frost physics inside
  the rerun package.

Open follow-up outside SNOWDENSITY-08:

- The strategy still lists the physically correct opt-in density cap as an open
  contract question.
- Low-canopy/mixed/deciduous melt adjudication remains SNOWDENSITY-05H and is
  not required before rerunning density gates.

