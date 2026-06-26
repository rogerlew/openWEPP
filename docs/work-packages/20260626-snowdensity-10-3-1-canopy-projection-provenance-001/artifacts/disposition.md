# Disposition

Status: complete.

Closure: `COMPLETE-10-3-1-CANOPY-PROJECTION-PROVENANCE-ARCHIVED`.

The package archived the requested provenance evidence for all eight
`cancov_forest` fixtures and dispositioned every mismatch.

The major result is a downstream constraint, not a production code change:
current openWEPP snowbench melt diagnostics use static initial-condition canopy
(`generated_openwepp_runtime_surface.cancov`), while upstream wepppy seasonal
projection evidence is per-day WEPP output by management class. Mixed,
deciduous, and pasture/open canopy claims require a later package to route
per-day canopy into the diagnostic path or explicitly scope the adjudication to
static initial canopy.

No fixture inputs, production Rust code, science contracts, output schemas,
defaults, or physics constants changed.

