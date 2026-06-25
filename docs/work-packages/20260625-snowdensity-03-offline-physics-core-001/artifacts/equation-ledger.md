# Equation Ledger

Static:

- Scope is `physics_bulk_candidate_v1` in Rust snowbench only.
- Runtime coupling is explicitly `none; offline snowbench candidate only`.
- Contract authority is `SC-SNOWFREEZE-001#INV-SNOWFREEZE-051` and
  `OBL-SNOWFREEZE-P-026`; ADR-0027 authorizes opt-in candidate work without
  default activation.

## Candidate State

- `ice_mass_kg_m2`: frozen/solid SWE store.
- `liquid_water_kg_m2`: retained liquid store.
- `density_kg_m3`: bulk snowpack density.
- `cold_content_j_m2`: bulk snowpack energy deficit relative to 0 degC.
- `snow_cover_age_h`: snow cover age used by densification.

Depth is derived as total mass divided by density. SWE is total mass converted
to water depth. The implementation maintains one snow mass ledger and derives
depth/density outputs from that state.

## Constants

| Constant | Value | Provenance / disposition |
|---|---:|---|
| `ice_density_kg_m3` | `917.0` | Physical ice density; candidate-only. |
| `water_density_kg_m3` | `1000.0` | Physical water density; candidate-only. |
| `latent_heat_fusion_j_kg` | `333500.0` | Latent heat of fusion; candidate-only. |
| `snow_heat_capacity_j_kg_k` | `2100.0` | Bulk snow heat capacity; candidate-only. |
| `new_snow_density_min_kg_m3` | `50.0` | Anderson/SNOW-17-style lower bound; candidate-only. |
| `new_snow_density_max_kg_m3` | `200.0` | Anderson/SNOW-17-style upper bound; candidate-only. |
| `new_snow_density_base_kg_m3` | `50.0` | Anderson/SNOW-17-style cold fresh-snow density; candidate-only. |
| `new_snow_density_temperature_threshold_c` | `-15.0` | Fresh-density temperature threshold; candidate-only. |
| `new_snow_density_temperature_coefficient` | `1.7` | Fresh-density temperature coefficient; candidate-only. |
| `dry_compaction_max_density_kg_m3` | `550.0` | SNOBAL/PySnobal lineage cap from `_time_compact.c`; candidate-only. |
| `dry_compaction_swe_max_kg_m2` | `2000.0` | SNOBAL/PySnobal lineage cap from `_time_compact.c`; candidate-only. |
| `wet_compaction_max_density_kg_m3` | `550.0` | SNOBAL/PySnobal lineage cap from `_h2o_compact.c`; candidate-only. |
| `wet_compaction_half_saturation_fraction` | `0.4` | SNOBAL/PySnobal lineage wet-compaction shape constant; candidate-only. |
| `max_liquid_water_volume_fraction` | `0.01` | Stable form of SNOBAL retained-water volume limit; candidate-only. |
| `positive_degree_melt_kg_m2_per_c_hour` | `0.18` | Provisional bulk positive-degree melt proxy; diagnostic only. |
| `solar_melt_efficiency_kg_m2_per_mj` | `0.02` | Provisional bulk shortwave melt proxy; diagnostic only. |
| `subfreezing_cold_content_relaxation_per_hour` | `0.015` | Provisional cold-content accumulation proxy; diagnostic only. |

## Candidate Equations

Fresh snow density:

```text
rho_new = clamp(
    rho_base                                  if T <= T_threshold
    rho_base + c * (T - T_threshold)^1.5      otherwise,
    rho_min,
    rho_max
)
```

New snow mixing:

```text
depth_total = depth_existing + snow_mass / rho_new
rho_mixed = (ice_mass + liquid_mass + snow_mass) / depth_total
```

Liquid retention:

```text
capacity_factor =
    max_liquid_water_volume_fraction
    * rho_water
    * (rho_ice - rho_bulk)
    / (rho_ice * rho_bulk)

retained_liquid_capacity =
    capacity_factor * ice_mass / (1 - capacity_factor)
```

The stable form above avoids release-capacity feedback when retained liquid
changes total snow mass.

Dry compaction:

```text
PTM = 0.5 * density / (density - 150)
POC = min(SWE, SWE_MAX) / SWE_MAX
rate = exp(-23.5 * PTM) * exp(-24.5 * POC)
density_after = density + (rho_max - density) * rate
```

Wet compaction:

```text
saturation = liquid / retained_liquid_capacity
wet_factor = saturation / (saturation + B)
density_after = density + (rho_max - density) * wet_factor
```

Cold-content refreeze:

```text
refreeze_mass = min(liquid, cold_content / latent_heat_fusion)
ice += refreeze_mass
liquid -= refreeze_mass
cold_content -= refreeze_mass * latent_heat_fusion
```

Mass closure per step:

```text
residual =
    (state_after_total + liquid_release)
    - (state_before_total + liquid_input + solid_input)
```

## Promotion Boundary

These equations are not production physics authority. They are the offline
candidate envelope needed for SNOWDENSITY-04 adjudication. Promotion requires a
later package to beat legacy on forcing-robust rubric cells without per-site
constants and then add runtime opt-in coupling under a separate gate.
