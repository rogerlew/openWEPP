# Rust Correctness Review — `10b914da1`

Evidence: `Static` plus focused `Ran` evidence.

Verdict: `HOLD`.

The reviewer confirmed one material cross-input precedence defect in each of
two exported public seams:

- `apply_surface_liquid_resource_phase()` validated configuration and beginning
  state domains before inspecting finalized-use and condensation-credit E002
  identities; and
- `execute_surface_liquid_ingress()` likewise allowed configuration or
  beginning/working-state E003 failures to mask wrong ingress identities.

The finding is accepted. The correction performs whole-public-envelope E001/E002
preflight before E003 validation, preserves the existing later category order,
and adds mixed-poison matrices for configuration, beginning state, working
state, finalized use, condensation and ingress inputs.

The interrupted reviewer did not run broad or heavy gates. Fresh exact-byte
review is required after the correction is committed.
