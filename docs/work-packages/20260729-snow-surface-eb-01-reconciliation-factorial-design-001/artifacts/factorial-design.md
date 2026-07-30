# B/L/S/LS Factorial Design

Status: `pre-registered`; result-bearing execution is not authorized here.

## Scientific Question

Does adding authoritative sub-canopy longwave (`L`), energy-consistent
sublimation (`S`), or both (`LS`) improve snow mass, persistence, and
downstream timing relative to a common baseline (`B`)? The interaction is
material because both mechanisms act through the same snow temperature, cold
content, energy, and mass.

## Comparable Cells

The four cells are fixed in [factorial-cells.csv](factorial-cells.csv). Every
cell must use identical forcing, starting state, time step, canopy projection,
phase model, density model, liquid holding/routing, surface-energy carrier,
observation operator, reporting window, and numeric policy. Longwave and
sublimation must be independent typed selectors. The current single
`SnowMeltModel` enum cannot create comparable cells because it bundles
sublimation with liquid holding.

The carrier may not be chosen merely to activate a previously nonpromoted
candidate. If a successor uses the multilayer surface-energy path, all four
cells must use that exact path and must preserve its opt-in status. A successor
may instead admit a common surface-energy carrier, but it must do so
prospectively in the canonical contract.

## Order Of Operations

At each hourly step:

1. Partition precipitation and freeze all common forcing.
2. Diagnose or solve one shared snow-surface temperature under an admitted
   rule.
3. Compose atmospheric and canopy longwave by complementary sky and canopy
   view fractions; canopy emission replaces, rather than simply supplements,
   the displaced sky contribution.
4. Compute sensible heat and one coupled latent/vapor exchange from the same
   aerodynamic state.
5. Sum signed surface-energy operands on one area/time basis.
6. Update cold content, melt, and refreeze.
7. Debit the vapor mass corresponding to the latent flux exactly once.
8. Update retained liquid and route only liquid water.

## Estimands

For every response `Y`:

- longwave main effect: `Y(L) - Y(B)`;
- sublimation main effect: `Y(S) - Y(B)`;
- combined effect: `Y(LS) - Y(B)`; and
- interaction: `Y(LS) - Y(L) - Y(S) + Y(B)`.

Signs must be interpreted per response; an improvement is not automatically a
positive numerical difference. The response definitions and units are frozen
in [response-operator-ledger.csv](response-operator-ledger.csv).

## Independent Closure

The producer records the raw operands named in
[mass-energy-operand-lineage.csv](mass-energy-operand-lineage.csv). Mass uses a
whole-snowpack water control volume over one square meter of ground. Total
storage is ice plus retained liquid; melt and refreeze are internal transfers
and do not enter the mass balance. Signed vapor exchange is positive for
deposition into the pack and negative for sublimation out of it. A separate
consumer reconstructs:

`mass_residual = total_water_storage_before + solid_input_to_pack
+ liquid_input_to_pack + vapor_mass_exchange - liquid_outflow
- total_water_storage_after`.

Energy uses the surface/phase solve before liquid routing, on the same ground
area. `thermal_energy` is signed internal sensible energy relative to the
contract-admitted `0 degC` ice reference. `net_phase_change_mass` is positive
for melt and negative for refreeze. After multiplying all fluxes by the exact
`step_duration`, the consumer reconstructs:

`energy_residual = step_duration * (net_shortwave + net_longwave
+ sensible_heat + latent_heat + ground_conduction
+ advected_precipitation_heat) - (thermal_energy_after
- thermal_energy_before) - latent_heat_fusion * net_phase_change_mass`.

The reconstruction must read public or diagnostic raw operands, not the
producer's reported residual. It independently reconstructs the signed
latent/mass pair with the phase-appropriate value returned by
`latent_heat_for_surface_temperature`:

`latent_heat * step_duration
= latent_heat_exchange * vapor_mass_exchange`.

Thus sublimation is negative in both the surface-energy and mass conventions;
deposition is positive in both. A loss-positive public counter may be derived
as `max(0, -vapor_mass_exchange)`, but it cannot become a second mass debit.
Daily totals may be formed only after per-step duration conversion.

The numeric tolerances for mass closure, energy closure, and latent/mass
equivalence are owned by EB-03 canonical-authority work and must be frozen
before result-bearing EB-04 execution. Until then, the decision ledger carries
an explicit execution hold rather than deferring an acceptance gate.

## Interpretation Order

First verify trace identity, both closures, and latent/mass equivalence. Then
examine the direct energy and vapor operands. Only after those pass should the
campaign interpret SWE,
depth, disappearance, runoff timing, or frost response. A better aggregate
score cannot compensate for a failed physical ledger.
