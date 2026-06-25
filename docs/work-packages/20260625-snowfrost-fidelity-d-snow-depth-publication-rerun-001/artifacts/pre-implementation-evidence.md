# Pre-Implementation Evidence

Evidence mode: Static.

SNOWFROST-FIDELITY-A classified all five pilot sites without defect
attribution because modeled snow depth was absent. The observation manifest and
harness explicitly rejected WAT `Snow-Water` as a snow-depth proxy.

`SC-SNOWFREEZE-001` already defines `snow_runtime_depth` and
`snow_runtime_density` as runtime snow-state diagnostic surfaces and binds
`TOL-SNOWFREEZE-009` to paired modeled-vs-observed snow depth, not SWE.

Existing runner trace/publication code reads `snow.runtime_depth_m`, so D can
publish that existing state without changing snow/frost physics.

Implementation constraint: add a diagnostic output surface only. No edits may
change constants, equations, snow/frost runtime branch conditions, Qwet,
SFCC/frozen-K models, or observation thresholds.
