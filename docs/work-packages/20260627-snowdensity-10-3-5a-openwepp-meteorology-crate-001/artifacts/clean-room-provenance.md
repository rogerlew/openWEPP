# Clean-Room Provenance

Status: complete
Evidence mode: Static + Ran

Map every implemented equation, constant, test vector, and reference value to its
source.

## Source-Use Statement

- CHM/GPL code read or used: **No**. No CHM source tree or GPL implementation
  was opened, read, ported, paraphrased, or translated.
- MetPy used: **Limited static reference only**. Consulted
  `/home/workdir/MetPy/src/metpy/calc/thermo.py` for the standard liquid/solid
  saturation-helper shape and phase naming. No MetPy code was translated.
- Harder & Pomeroy equation extraction complete: **Yes**. Static extraction from
  `references/copyrighted/source_pdfs/harder2013.pdf`.
- Ran: independent Python scratch calculations were used to generate fixed
  numeric reference values for tests from the recorded equations; the script did
  not write files.

## Equation Ledger

| Implementation item | Source | Evidence class | Notes |
|---|---|---|---|
| `saturation_vapor_pressure_water_kpa` | Harder & Pomeroy 2013 Appendix A vapor-pressure expression | Static | Uses `0.611 * exp(17.3*T/(237.3+T))` in `kPa`; `T` is Celsius. |
| `saturation_vapor_pressure_ice_kpa` | Standard Magnus-type ice helper; MetPy consulted only to confirm liquid/solid helper separation | Static + Inference | Helper only; not production `RST` authority and not used to alter WEPP partition. |
| `actual_vapor_pressure_from_relative_humidity_kpa` | Psychrometric definition: RH times saturation vapor pressure | Static + Inference | Uses auto-phase saturation for candidate solver identity. |
| `dew_point_from_relative_humidity` / `relative_humidity_from_dew_point` | Algebraic inverse of the Harder-Pomeroy liquid-water saturation helper | Static + Inference | Used for checked primitive round trips. |
| `vapor_density_from_pressure_and_temperature` | Harder & Pomeroy 2013 Appendix A ideal-gas vapor-density expression | Static | Uses water molar mass and universal gas constant. |
| `molecular_diffusivity_water_vapor_in_air` | Harder & Pomeroy 2013 Appendix A diffusivity expression | Static | `2.06e-5 * (TaK / 273.15)^1.75`. |
| `thermal_conductivity_air` | Harder & Pomeroy 2013 Appendix A thermal-conductivity expression | Static | Uses Kelvin air temperature. |
| `latent_heat_sublimation` | Harder & Pomeroy 2013 Appendix A latent-heat expression | Static | Selected for candidate hydrometeor temperature below 0 degC. |
| `latent_heat_vaporization` | Harder & Pomeroy 2013 Appendix A latent-heat expression | Static | Selected for candidate hydrometeor temperature at/above 0 degC. |
| `hydrometeor_temperature_from_relative_humidity` | Harder & Pomeroy 2013 hydrometeor energy-balance equation | Static + Ran | Implemented as fixed-point solver with typed non-convergence error and iteration metadata. |
| `rainfall_fraction_for_hydrometeor_temperature` | Harder & Pomeroy 2013 logistic rainfall-fraction coefficients | Static | Includes 15-minute, hourly, and daily coefficient sets. |

## Reference Vectors

| Vector | Source | Expected value | Tolerance | Notes |
|---|---|---:|---:|---|
| Water saturation at 0 degC | Harder-Pomeroy expression | `0.611 kPa` | `1e-12` | Crate test. |
| Water saturation at 20 degC | Harder-Pomeroy expression, independent Python reconstruction | `2.344507723843366 kPa` | `1e-12` | Crate test. |
| Ice saturation at -10 degC | Magnus-type ice helper, independent Python reconstruction | `0.2598724746280262 kPa` | `1e-12` | Crate test; helper-only. |
| Dewpoint for 10 degC and RH 0.5 | Inverse Harder-Pomeroy liquid-water saturation | `0.08792980043198889 degC` | `1e-12` | Crate test with RH round trip. |
| Latent heat vaporization at 0 degC | Harder-Pomeroy Appendix A | `2501000 J kg^-1` | `1e-9` | Crate test. |
| Latent heat sublimation at -10 degC | Harder-Pomeroy Appendix A | `2836600 J kg^-1` | `1e-9` | Crate test. |
| Diffusivity at 0 degC | Harder-Pomeroy Appendix A | `2.06e-5 m^2 s^-1` | `1e-15` | Crate test. |
| Air conductivity at 0 degC | Harder-Pomeroy Appendix A | `0.02393845 W m^-1 K^-1` | `1e-12` | Crate test. |
| Hydrometeor `Ti`, Ta 0 degC RH 0.5 | Independent Python reconstruction | `-3.229846367476 degC` | `1e-6` | Crate test. |
| Hydrometeor `Ti`, Ta 5 degC RH 0.7 | Independent Python reconstruction | `2.6791461600920456 degC` | `1e-6` | Crate test. |
| Hydrometeor `Ti`, Ta -5 degC RH 0.8 | Independent Python reconstruction | `-5.963503925215093 degC` | `1e-6` | Crate test. |
| Hourly rain fraction, Ti -2 degC | Harder-Pomeroy hourly coefficients | `0.006204718602135303` | `1e-12` | Crate test. |
| Hourly rain fraction, Ti 0 degC | Harder-Pomeroy hourly coefficients | `0.28548100694860773` | `1e-12` | Crate test. |
| Hourly rain fraction, Ti 2 degC | Harder-Pomeroy hourly coefficients | `0.96236114903252` | `1e-12` | Crate test. |
