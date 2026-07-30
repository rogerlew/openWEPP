# Canopy-Air Temperature Proxy Sensitivity

Caption: Analytical error in canopy-emitted longwave when air temperature is
used in place of effective canopy radiometric temperature. Each line is a
hemispherical sky-view fraction; lower sky view gives the canopy more weight.

- Question: How strongly does an air-temperature proxy respond to canopy-air
  temperature mismatch?
- Population: Hypothetical homogeneous stands at an air temperature of `0 °C`;
  this is not a fitted site dataset.
- Units: Temperature difference in `°C`; longwave error in `W m^-2`.
- Processing: Exact Stefan-Boltzmann difference
  `(1-f_sky) sigma [(T_air+delta T)^4-T_air^4]` using
  `sigma = 5.670374419e-8 W m^-2 K^-4`.
- Interpretation: A dense-view canopy (`f_sky=0.1`) that is `5 °C` warmer than
  air contributes roughly `20 W m^-2` more than the air proxy. The same
  mismatch matters far less when most of the hemisphere is visible sky.
- Uncertainty: The plot isolates one operand. Atmospheric longwave, surface
  temperature, emissivity, spatial temperature variation, and measurement
  error are held out.
- Limitation: This is a sensitivity illustration, not a correction curve or
  calibration range.
