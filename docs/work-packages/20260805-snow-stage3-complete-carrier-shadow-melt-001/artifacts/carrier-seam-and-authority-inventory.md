# Carrier Seam And Authority Inventory

Evidence class: Static.

## Existing real path

- Carrier orchestration:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`.
- Typed runtime inputs and diagnostics:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`.
- Admitted primitives: `crates/openwepp-meteorology/src/surface_energy.rs`.

The current carrier integrates net shortwave, selector-controlled net
longwave, selector-controlled latent sublimation, and active/lower conduction.
It passes zero for sensible heat and precipitation-advected heat to
`surface_energy_balance`, exactly matching `GAP-SNOWENERGY-011`.

## Available inputs

The real substep has hourly rain, snowfall, air temperature, radiation, pack
surface temperature, wind speed, dewpoint, atmospheric pressure, canopy
fraction, and substep duration. These are sufficient for the existing
precipitation-advection primitive and longwave primitive.

## Missing authoritative turbulent boundary

`TurbulentFluxInputs` additionally requires:

- air-temperature measurement height `z_T`;
- vapor-pressure measurement height `z_q`;
- wind measurement height `z_u`; and
- aerodynamic roughness length `z_0`.

No Stage 3 runtime input currently supplies these quantities. Pinned libsnobal
commit `bf8b41c...` likewise treats `z_T`, `z_u`, and `z_0` as measurement or
parameter inputs; it does not establish one universal set applicable to the
openWEPP forcing/canopy seam. The pysnobal wrapper's default wind height alone
does not close the other boundaries.

## Disposition

Hold production edits. Guessing fixed heights or roughness would be provisional
process physics and would undermine the prospective carrier test. Amend
`SC-SNOWENERGY-001` first to select typed sources and relative/absolute-height
chronology, then implement all complete-carrier operands in shadow together.
CoE remains the sole melt owner.
