# Selector Exclusion

Status: `PASS for retained bounded checkpoint`

Evidence class: `Static + Ran`

The checkpoint adds one explicit library export in
`openwepp-hillslope-orchestrator` and an integration-test caller. Recursive
source inspection found no reference to `openwepp-land-surface-energy` or
`land_surface_energy_shadow` under `crates/openwepp-runner/**` and no new
production scheduler, direct-runtime dispatch, input-selector, default or
output-publication call site.

The bridge accepts an explicitly supplied cloned production frame and returns
an uncommitted shadow result. Tests prove the supplied production frame is
unchanged after success and failure. The retained code therefore remains
default-off and unreachable from production selection.

This PASS applies only to selector exclusion. It does not close the missing
forest-litter water owner or authorize activation.
