# Schema And Lineage

Status: executed-hold

Evidence mode: Static + Ran.

| PySnobal column | Units | openWEPP/WEPP source | Source class | Conversion or formula | Rejected aliases | Enforced by |
| --- | --- | --- | --- | --- | --- | --- |
| `net_solar_Wm-2` | W m^-2 | SIMIMPL28 hourly `rad_mj_m2` | `diagnostic-proxy` | `MJ m^-2 h^-1 * 1e6 / 3600 * 0.80` | raw daily `rad` in langleys/day | `snowfrost_fidelity_g0_pysnobal_bridge_contract` |
| `downwelling_thermal_Wm-2` | W m^-2 | Hourly air temperature plus cloud fraction | `diagnostic-proxy` | Stefan-Boltzmann proxy with cloud-adjusted emissivity | net radiation, shortwave radiation | lineage JSON and harness lineage validation |
| `temp_air_degC` | degC | SIMIMPL28 hourly air temperature | `mechanical` | none | daily `tmax`, daily `tmin` | focused contract and finite-row validation |
| `temp_ground_degC` | degC | Constant G0 lane at `z.soil_temp_m` | `diagnostic-proxy` | none | `frost.hourly.surface_temp_c_####`, `surtmp(hour)`, snow-surface temperature | focused contract and config assertion |
| `vapor_pressure_Pa` | Pa | Climate dew point through `saturation_vapor_pressure_kpa` | `deterministic-derived` | `kPa * 1000` | relative humidity, air-temperature saturation vapor pressure | lineage JSON and finite-row validation |
| `wind_speed_ms-1` | m s^-1 | Daily climate `vwind` repeated hourly | `mechanical` | none | wind direction degrees | lineage JSON and finite-row validation |
| `precip_mass_mm` | mm water equivalent | SIMIMPL28 rain depth plus snowfall depth | `deterministic-derived` | `rain_m * 1000 + snowfall_depth_m * snow.options.newsnw` | `snow.hourly.snowfall_m_####` as water equivalent | focused contract, audit, nonnegative precip validation |
| `precip_temp_degC` | degC | Hourly air temperature | `diagnostic-proxy` | none | ground temperature, snow-surface temperature | lineage JSON |
| `snow_precip_fraction` | unit interval | Snow mass divided by total precipitation mass | `deterministic-derived` | `snow_mass_mm / precip_mass_mm`, else `0` | rain/snow branch flag | unit-interval validation |
| `snow_precip_density_kgm-3` | kg m^-3 | `snow.options.newsnw` from `snow.txt` or parser default | `mechanical` | none | WAT `Snow-Water`, WAT `Snow-Depth` | focused contract and audit |

Ran: site1 exported `lineage.json` contains every required PySnobal column with
`source_class` and `rejected_aliases`. Ran:
`cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract` passed.
