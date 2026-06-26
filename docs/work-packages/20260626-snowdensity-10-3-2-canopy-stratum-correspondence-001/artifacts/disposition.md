# Disposition

Status: complete.

Closure: `COMPLETE-10-3-2-CANOPY-STRATUM-BINDING-DISPOSITIONED`.

The package mapped Harvard and Marcell observed canopy strata to the current
modeled surfaces and closed with a binding constraint.

Current Harvard and Marcell fixtures are single static mixed-forest hillslopes
with runtime `cancov = 0.55`. They do not bind to the advertised observed
strata: Harvard `hemlock` / `hardwood` / `open`, or Marcell
`conifer` / `deciduous` / `open`.

The fixtures remain useful as mixed-hillslope diagnostics and planning anchors,
but they cannot carry canopy-stratum verdicts until paired variants or an
explicit aggregate observation binding exists.

No fixture inputs, production Rust code, science contracts, output schemas,
defaults, selectors, coefficients, radiation, albedo, density, melt, or frost
behavior changed.
