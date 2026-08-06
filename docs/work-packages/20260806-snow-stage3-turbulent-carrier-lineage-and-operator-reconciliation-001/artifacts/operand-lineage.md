# Operand Lineage

Status: `result-blind field freeze / amendment review required`.

Evidence class: `Static`.

Schema v6 is an enabled-only diagnostic projection. It contains ordered
duration-tagged substep objects. Existing public schema-v5 fields and their
`flux * seconds / 3600` accumulation semantics remain unchanged. Schema-v6
hourly energies and evaluated-support means are separately named consumer
derivations and never reinterpret v5.

Positive energy and vapor mass point toward snow. Temperatures are Celsius,
pressure is pascals, lengths are metres, velocities are `m s^-1`, energy flux
is `W m^-2`, energy is `J m^-2`, mass is `kg m^-2`, mass flux is
`kg m^-2 s^-1`, density is `kg m^-3`, specific humidity is `kg kg^-1`, and
heat capacity is `J kg^-1 K^-1`. Every row is diagnostic and consumer-forbidden
unless explicitly identified as raw authoritative source lineage.

## Applicability And Support

An evaluated tuple has `applicable=true`, reason `evaluated`, and finite values
for every field applicable to its selected equations. A non-evaluated hour has
no tuple and one typed hourly reason: `no_resolved_snow_at_day_start`,
`thin_pack_boundary_reached`, or `operator_not_selected`. JSON `null` represents
N/A; numeric zero never represents missing state, endpoints, stability, or
transfer. Neutral/zero-wind stability is applicable with an explicit stability
class and nullable Obukhov length, not a missing tuple.

Tuple duration is the exact solver interval. Same-state emits one `3600 s`
tuple per evaluated hour. Sequential emits every existing dynamic substep.
Consumer-derived hourly energy is `sum(flux * duration)`; the evaluated-support
mean is `energy / sum(duration)` only when support is positive.

## Identity And Control-Volume Fields

| Schema-v6 field | Unit/type | Source expression | Support/aggregation | Role and rejected aliases |
| --- | --- | --- | --- | --- |
| `operator_id` | enum | selected admitted evaluation operator | tuple identity; no aggregation | Diagnostic; never production owner. |
| `hour_index`, `substep_index` | integer | ordered loop indices | exact positional join | Diagnostic; calendar-only joins rejected. |
| `elapsed_start_seconds`, `duration_seconds` | s | existing operator loop and selected substep | intervals must be contiguous and within `[0,3600]` | Exact support; requested time is not evaluated support. |
| `applicable`, `applicability_reason` | bool/enum | evaluation branch outcome | no numeric aggregation | Typed N/A authority. |
| `source_fingerprint_fnv1a64` | hex u64 | unchanged ordered day-start layer/cold snapshot hash | exact daily join | Raw authoritative source identity, not effective input identity. |
| `forcing_fingerprint_fnv1a64` | hex u64 | unchanged 24-hour forcing hash | exact daily join | Raw forcing identity. |
| `geometry_fingerprint_fnv1a64` | hex u64 | unchanged pressure/virtual-instrument/selector hash | exact daily join | Geometry identity. |
| `effective_input_fingerprint_fnv1a64` | hex u64 | ordered `to_bits` of effective active and total state immediately before carrier call | compare first tuple and track every tuple | Diagnostic carrier-input identity; raw source hash is a rejected alias. |
| `projection_id` | enum | `whole_column_immutable_v1` or `aligned_active_volume_v1` | first tuple classification | Separates projection from later evolution. |
| `active_layer_prefix_count_before`, `total_layer_count_before` | count | actual slices passed/retained immediately before carrier call | exact endpoint | Membership is ordered prefix `[0, active_count)` of the emitted layer-state fingerprint. |
| `active_layer_state_fingerprint_before_fnv1a64` | hex u64 | ordered active slice layer fields plus cold vector | exact endpoint | First effective active membership/state. |
| `total_layer_state_fingerprint_before_fnv1a64` | hex u64 | ordered complete clone layer fields plus cold vector | exact endpoint | Clone state, not production state. |
| `active_layer_prefix_count_after`, `total_layer_count_after` | count | clone after substep energy/mass operations | exact endpoint | Null for zero support. |
| `active_layer_state_fingerprint_after_fnv1a64`, `total_layer_state_fingerprint_after_fnv1a64` | hex u64 | same ordered hash after substep | exact endpoint | Same-state must equal before; no production alias. |

Each layer-state fingerprint covers, in order, current vector index,
`mass_swe_m`, `thickness_m`, `density_kg_m3`, `settle_day_count`,
`temperature_c`, `liquid_water_m`, effective cold content, and
`refrozen_liquid_m`. Prefix count plus ordered fingerprint is the frozen layer
membership representation; no persistent physical layer identifier is
invented.

## State And Forcing Fields

| Schema-v6 field | Unit | Source expression | Support/aggregation | Role and rejected aliases |
| --- | --- | --- | --- | --- |
| `active_ice_mass_before_kg_m2`, `active_ice_mass_after_kg_m2` | kg m^-2 | active `sum(mass_swe_m) * 1000` | exact tuple endpoints | Evaluation active volume. |
| `total_ice_mass_before_kg_m2`, `total_ice_mass_after_kg_m2` | kg m^-2 | clone `sum(mass_swe_m) * 1000` | exact tuple endpoints | Clone total; authoritative pack mass rejected. |
| `active_depth_before_m`, `active_depth_after_m` | m | active `sum(thickness_m)` | exact tuple endpoints | Thermal active depth; never aerodynamic `z_0,aero`. |
| `active_density_before_kg_m3`, `active_density_after_kg_m3` | kg m^-3 | control-volume mass/depth conversion already used by carrier | exact tuple endpoints | Effective active density. |
| `active_cold_before_j_m2`, `active_cold_after_j_m2` | J m^-2 | active cold-vector sum before/after | exact tuple endpoints | Positive energy required to warm to melt point. |
| `total_cold_before_j_m2`, `total_cold_after_j_m2` | J m^-2 | full clone cold-vector sum before/after | exact tuple endpoints | Clone total. |
| `surface_temperature_before_c`, `surface_temperature_after_c` | deg C | existing cold-content-to-temperature mapping for effective active state | exact tuple endpoints | Evaluation surface; authoritative Stage 3 temperature rejected. |
| `air_temperature_c` | deg C | hourly forcing | repeated per tuple; duration-weight only for summaries | Carrier input. |
| `dewpoint_c` | deg C | daily CLIGEN input used by existing carrier | repeated per tuple | Lineage for actual vapor pressure. |
| `wind_speed_m_s` | m s^-1 | daily CLIGEN wind used by existing carrier | repeated per tuple | Carrier input. |
| `air_pressure_pa` | Pa | selected Stage 3 surface-energy options | repeated per tuple | Carrier input. |
| `hourly_radiation_mj_m2` | MJ m^-2 hour^-1 | hourly forcing total | repeated per tuple; do not sum repeated value | Incoming shortwave lineage. |
| `daily_solar_radiation_mj_m2`, `daily_extraterrestrial_radiation_mj_m2` | MJ m^-2 day^-1 | selected daily forcing/options | repeated per tuple | Longwave cloud-forcing lineage. |
| `daylight` | bool | selected daily forcing/options | repeated per tuple | Longwave applicability. |
| `canopy_cover_fraction` | 1 | direct input | repeated per tuple | Longwave input. |
| `rain_m`, `snowfall_geometric_m` | m h^-1 total | hourly forcing totals | repeated; not integrated per substep | Snowfall requires frozen `0.1` SWE conversion. |
| `rain_mass_flux_kg_m2_s`, `snow_mass_flux_kg_m2_s` | kg m^-2 s^-1 | `rain_m*1000/3600`; `snowfall_geometric_m*0.1*1000/3600` | flux held through substeps; energy uses tuple duration | Substep duration divisor is a rejected alias. |
| `rain_temperature_c`, `snow_temperature_c` | deg C | hourly hydrometeor temperature when precipitating; otherwise surface temperature | repeated per tuple | Typed applicability retained. |
| `rain_specific_heat_j_kg_k`, `snow_specific_heat_j_kg_k` | J kg^-1 K^-1 | existing temperature-dependent `specific_heat_water/ice` result | repeated per tuple | Required advected operand; constant heat capacity rejected. |

## Radiation Fields

| Schema-v6 field | Unit | Source expression | Support/aggregation | Role and rejected aliases |
| --- | --- | --- | --- | --- |
| `incoming_shortwave_w_m2` | W m^-2 | `hourly_radiation_mj_m2 * 1e6 / 3600` | tuple flux; energy `* duration` | Incoming, not absorbed. |
| `snow_albedo_fraction` | 1 | selected snow albedo state or existing `0.45` fallback | repeated per tuple | Exact existing value; no new default. |
| `net_shortwave_w_m2` | W m^-2 | `incoming_shortwave * (1-albedo)` | tuple flux | State-independent external term. |
| `actual_vapor_pressure_pa` | Pa | existing saturation-vapor-pressure function at dewpoint | repeated per tuple | Longwave and turbulent input. |
| `longwave_cloud_fraction` | 1 | existing Dilley-Unsworth clearness mapping | tuple value | Diagnostic intermediate. |
| `sky_view_fraction` | 1 | `(1-canopy_cover)^1.6` | tuple value | Diagnostic intermediate. |
| `atmospheric_longwave_w_m2` | W m^-2 | existing Dilley-Unsworth atmospheric result | tuple flux | `L_atm`; state-independent for matched forcing. |
| `canopy_longwave_w_m2` | W m^-2 | existing air-temperature blackbody result | tuple flux | `L_can`; canopy temperature equals air is explicit. |
| `subcanopy_longwave_w_m2` | W m^-2 | `f_sky*L_atm + (1-f_sky)*L_can` | tuple flux | `L_sub`. |
| `outgoing_longwave_w_m2` | W m^-2 | existing snow-surface blackbody result | tuple flux | `L_out`; state-dependent. |
| `net_longwave_w_m2` | W m^-2 | `L_sub-L_out` | tuple flux; energy `* duration` | External signed term. |

## Turbulent Geometry And Primitive Fields

| Schema-v6 field | Unit | Source expression | Support/aggregation | Role and rejected aliases |
| --- | --- | --- | --- | --- |
| `air_temperature_height_m` | m | selected `z_T`, currently CLIGEN virtual instrument `5.0` | repeated per tuple | Virtual instrument height. |
| `vapor_pressure_height_m` | m | selected `z_q`, currently `5.0` | repeated per tuple | Virtual instrument height. |
| `wind_speed_height_m` | m | selected `z_u`, currently `5.0` | repeated per tuple | Virtual instrument height. |
| `aerodynamic_roughness_length_m` | m | selected `z_0,aero`, currently `0.005` | repeated per tuple | Bare `z_0` is rejected: it is thermal active depth in SC-SNOWENERGY-001. |
| `turbulent_max_iterations` | count | selected existing solver option, currently `50` | repeated per tuple | Solver control. |
| `turbulent_convergence_tolerance` | 1 | selected existing option, currently `1e-5` | repeated per tuple | Absolute/relative Obukhov test. |
| `surface_vapor_pressure_pa` | Pa | existing saturation-vapor-pressure function at evaluation surface temperature | tuple value | State-dependent. |
| `air_potential_temperature_k` | K | `T_air,K + (g/cp_air)*z_T` | tuple value | Exact solver primitive. |
| `surface_temperature_k` | K | existing Celsius-to-libsnobal-K mapping | tuple value | Exact solver primitive. |
| `specific_humidity_air_kg_kg`, `specific_humidity_surface_kg_kg` | kg kg^-1 | existing vapor/pressure molecular-mass equation | tuple value | Exact humidity pair. |
| `air_density_kg_m3` | kg m^-3 | existing gas-density/virtual-temperature equation at geometric mean air/surface states | tuple value | Exact solver primitive. |
| `displacement_height_m` | m | `2*7.35*z_0,aero/3` | tuple value | Exact geometry intermediate. |
| `log_momentum`, `log_sensible`, `log_latent` | 1 | `ln((z-d)/z_0,aero)` for `z_u/z_T/z_q` | tuple value | Exact neutral denominators. |
| `stability_class` | enum | `zero_wind`, `neutral`, `stable`, or `unstable` from existing solve | tuple value | N/A is not encoded as zero. |
| `obukhov_length_m` | m/null | final existing solver result; null only for admitted zero-wind/neutral completion | tuple value with class | Final solver diagnostic. |
| `psi_momentum`, `psi_sensible`, `psi_latent` | 1 | existing final stability-correction equation; zero for explicit neutral/zero-wind | tuple value | Exact final corrections. |
| `turbulent_iterations` | count | existing solver returned count | tuple value | Zero is valid only for zero wind. |
| `friction_velocity_m_s` | m s^-1 | final `k*u/(log_momentum-psi_momentum)` | tuple value | Exact final solver state. |
| `sensible_exchange_velocity_m_s` | m s^-1 | `k*u_star/(log_sensible-psi_sensible)` | tuple value | Diagnostic factor, not flux total. |
| `latent_exchange_velocity_m_s` | m s^-1 | `k*u_star/(log_latent-psi_latent)` | tuple value | Diagnostic factor, not flux total. |
| `surface_latent_heat_j_kg` | J kg^-1 | existing temperature-dependent vaporization/sublimation latent heat | tuple value | Constant latent heat rejected. |
| `vapor_mass_flux_kg_m2_s` | kg m^-2 s^-1 | `(q_air-q_surface)*rho_air*latent_exchange_velocity` | tuple flux | Positive deposition, negative sublimation. |
| `sensible_flux_w_m2` | W m^-2 | `(theta_air-T_surface,K)*rho_air*cp_air*sensible_exchange_velocity` | tuple flux | Positive toward snow. |
| `latent_flux_w_m2` | W m^-2 | `surface_latent_heat*vapor_mass_flux` | tuple flux | Positive toward snow. |

The independent analyzer reimplements these contract equations. It may compare
producer fluxes only after reconstructing them and may not call a Rust producer
helper or use a producer total as an input operand.

## Advected, Complete, And Endpoint Fields

| Schema-v6 field | Unit | Source expression | Support/aggregation | Role and rejected aliases |
| --- | --- | --- | --- | --- |
| `precipitation_advected_flux_w_m2` | W m^-2 | `cp_r*m_dot_r*(T_r-T_s) + cp_s*m_dot_s*(T_snow-T_s)` | tuple flux | External signed term. |
| `complete_external_flux_w_m2` | W m^-2 | shortwave + longwave + sensible + latent + advected | tuple flux | Producer total is check-only. |
| `vapor_mass_exchange_kg_m2` | kg m^-2 | `vapor_mass_flux*duration` | sum across tuples | Signed toward snow. |
| `sublimation_kg_m2` | kg m^-2 | bounded `max(-vapor exchange,0)` existing sequential debit | sum | Sequential only; same-state N/A, not zero. |
| `deposition_kg_m2` | kg m^-2 | `max(vapor exchange,0)` existing sequential addition | sum | Sequential only. |
| `melt_kg_m2` | kg m^-2 | existing bounded fusion debit | sum | Sequential only. |
| `cold_energy_change_j_m2` | J m^-2 | active cold before minus post-energy/pre-export cold | sum | Sequential only. |
| `cold_content_export_j_m2` | J m^-2 | proportional cold removed with melt/sublimation | sum | Positive removed from clone. |
| `internal_active_lower_conduction_j_m2` | J m^-2 | existing internal exchange | sum; separate from external subset | Nonexternal; snow-ground heat alias rejected. |
| `energy_closure_residual_j_m2` | J m^-2 | existing sequential thermodynamic allocation residual | maximum absolute and sum | Check-only. |

Sequential endpoint identities are independently checked for every tuple and
hour:

```text
total_ice_after = total_ice_before - melt - sublimation + deposition
total_cold_after = total_cold_before - cold_energy_change - cold_content_export
```

Mass tolerance is `max(1e-12 kg m^-2, 1e-12*sum_abs_operands)` and cold/energy
tolerance is `max(1e-6 J m^-2, 1e-12*sum_abs_operands)`. Same-state requires
`to_bits` equality for before/after state and fingerprints. The first effective
input comparison uses the same unit-specific tolerances plus exact layer counts
and fingerprints.

## Rejected Aliases And Anti-Tautology

Fixtures must make every accepted value distinct from production Stage 3
surface temperature, CoE energy/melt, internal conduction, snow-ground heat,
whole-column state where active state is required, raw-source fingerprint where
effective-input fingerprint is required, substep precipitation divisors,
constant heat capacity/latent heat, zero-filled N/A, `duration/3600` v5 flux
arrays, and calendar support. Historical v5 is parsed only by an explicit v5
adapter; unknown schema versions fail closed.
