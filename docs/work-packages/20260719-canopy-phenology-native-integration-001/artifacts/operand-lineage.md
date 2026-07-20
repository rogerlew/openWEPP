# Operand Lineage

Evidence mode: `Static`

Status: `complete for CP-GSI02`

| Operand | Units / basis | Source authority | Runtime alias / consumer | Disposition |
|---|---|---|---|---|
| `GSI21` | fraction; trailing 1-21 real daily samples | Jolly et al. equations plus SC-PLANT cold-start/chronology rule | `GsiDailyResult::growing_season_index` | authoritative state |
| `Bf,max` | `kg m^-2`; full-leaf area basis | native YAML | `summer_foliar_biomass_kg_m2` | authoritative parameter |
| `fe` | fraction | native YAML | `evergreen_fraction` | authoritative parameter |
| `Bfe`, `Bfd` | `kg m^-2` | CP-GSI02 equations | realization evergreen/deciduous fields | authoritative state |
| `Bf` | `kg m^-2` | exact `Bfe+Bfd` | realization live foliar field; native growth/interception biomass | authoritative state |
| `Bs` | `kg m^-2`; nonseasonal | native YAML | realization structural biomass | authoritative state; excluded from transfer |
| `LAI` | `m^2 m^-2` | `xmxlai*f` | realization/growth/ET/WB15 LAI | authoritative state |
| `Cc` | fraction | baseline `1-exp(-bb*Bf)` plus structural floor and 0.999 cap | realization/growth/snow/ET/WB15/erosion | authoritative state |
| `A_leaf` | `kg m^-2 d^-1` | positive daily `Bf` change | realization allocation field | authoritative flux; internal evidence only |
| `L_leaf` | `kg m^-2 d^-1` | negative daily `Bf` change | realization litter -> decomposition surface litter | authoritative flux |
| surface residue | `kg m^-2` | decomposition of prior residue plus same-day `L_leaf` | decomposition state | authoritative state |
| residue depth | `m` | residue mass times declared conversion | decomposition state -> frost thermal input | authoritative coupling state |

All mass operands use OFE horizontal-area basis. No conversion occurs between
the canopy realization, growth handoff, and decomposition litter input. The
only unit conversion in the CP-GSI02 chain is residue mass-to-depth through the
existing declared coefficient. Test-only traces are diagnostic evidence of the
authoritative consumers; they do not replace or publish state.
