# Disposition

Evidence class: Static + Ran.

Disposition: COMPLETE-07-RUNTIME-OPT-IN.

SNOWDENSITY-07 completed the typed runtime opt-in for
`physics_bulk_density_compaction_v1`. The implementation keeps `legacy_wepp`
as default/rollback and keeps CoE SWE/liquid/routed-melt boundaries
authoritative. The opt-in mutates only publication/runtime physical snow depth
and bulk density, and carries the legacy CoE boundary depth/density/settle
separately for future CoE melt boundary calculations.

Closure gates passed, including full workspace tests and dependency policy.

No default activation, parser/runfile/CLI selector, output schema, tuning,
mixed/deciduous melt adjudication, or frost attribution changed.
