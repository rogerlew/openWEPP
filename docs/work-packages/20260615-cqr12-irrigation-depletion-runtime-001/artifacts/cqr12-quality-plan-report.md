# CQR12 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.

Static: protected boundaries are public API, `irrigation.depletion.*` symbols,
typed errors, field names, allowed strings, parser compatibility, units,
numeric expression order, period iteration/order, and kernel-facing projection
behavior.

Static: baseline target identity was
`seed_hillslope_runtime_surface_from_irrigation_depletion`, line `52`, CC
`33.0`, coverage `0.0`, CRAP `1122.0`.

Static: closure target is the same public seed function after private helper
extraction plus every newly extracted depletion helper. The closure threshold is
CRAP `<= 30`.

Static: review and verification must disposition warnings for target-file
coverage below the science-tier threshold and the pre-existing out-of-scope
frost `too_many_lines` suppression.

Status: complete.
