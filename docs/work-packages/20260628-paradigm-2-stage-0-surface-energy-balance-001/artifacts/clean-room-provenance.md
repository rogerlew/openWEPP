# Clean-Room Provenance

Evidence mode: `Static`

## License And Source Hygiene

Local source reference:

- Repository: `/workdir/pysnobal` (`/workdir` is a symlink to `/home/workdir`).
- Clone commit: `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`.
- License record: `/workdir/pysnobal/setup.py` line 75 declares
  `license="CC0 1.0"`.
- `deny.toml` allow-lists `CC0-1.0`; GPL/AGPL/LGPL family licenses are excluded
  by omission from the allow list.

The local PySnobal repository has no top-level LICENSE file. The setup.py CC0
declaration is therefore the recorded portability evidence for this Stage 0
port. Constants and equations were ported as reusable numerics; no value was
fitted to openWEPP fixtures.

## Equation And Constant Map

| Stage 0 primitive | openWEPP symbol | Source authority | Constants |
|---|---|---|---|
| Net shortwave | `net_shortwave_radiation` | Standard balance `incoming * (1 - albedo)` paired with libsnobal `_net_rad.c` all-wave structure. | Caller-supplied albedo. |
| Net longwave | `net_longwave_radiation` | libsnobal `_net_rad.c`; `radiation.h`. | Stefan-Boltzmann `5.67032e-8 W m^-2 K^-4`; caller-supplied emissivity. |
| Net all-wave | `net_all_wave_radiation` | libsnobal `_net_rad.c`. | Sum of net shortwave plus emissivity-scaled longwave. |
| Turbulent sensible heat | `turbulent_fluxes_monin_obukhov` | libsnobal `_h_le.c` and `hle1.c`. | `CP_AIR=1005 J kg^-1 K^-1`, `VON_KARMAN=0.41`, `GRAVITY=9.80665 m s^-2`, Paeschke constant `7.35`, stable beta `5.2`, unstable beta `16`, max iterations `50`, tolerance `1e-5`. |
| Turbulent latent heat and mass flux | `turbulent_fluxes_monin_obukhov`, `latent_heat_flux_from_mass_flux`, `mass_flux_from_latent_heat_flux` | libsnobal `hle1.c`, `evap.c`, `envphys.h`. | `MOL_AIR=28.9644 kg kmol^-1`, `MOL_H2O=18.0153 kg kmol^-1`, `RGAS=8.31432e3 J kmol^-1 K^-1`, `LH_VAP`, `LH_FUS`, `LH_SUB`. |
| Saturation vapor pressure helper | `saturation_vapor_pressure_snobal_pa` | libsnobal `sati.c`, `satw.c`, `envphys.h`. | `FREEZE=273.16 K`, `BOIL=373.15 K`, `SEA_LEVEL=101324.6 Pa`. |
| Ground/substrate conduction | `conductive_heat_flux` | libsnobal `g_snow.c`, `g_soil.c`, and generic transfer function `ssxfr.c`. | Series-resistance transfer `2*k1*k2*(T2-T1)/(k2*d1+k1*d2)`. |
| Advected heat from precipitation | `precipitation_advected_heat_flux` | libsnobal `_advec.c`, `heat_stor.c`, `envphys.h`. | `CP_WATER(T)=4217.7 - 2.55*(T-273.16)`, `CP_ICE(T)=4.186798188*(0.024928+0.00176*T)/0.001`. |
| Surface balance sum | `surface_energy_balance` | libsnobal `_e_bal.c`; Marks et al. 1999 energy/mass flux set. | Sum `Rn + H + L_vE + G + M`. |

## Published Context

Marks et al. 1999 describes ISNOBAL/SNOBAL as an energy-balance snowmelt model,
shows the point energy and mass fluxes in its conceptual diagram, and lists the
energy flux outputs `Rn`, `H`, `LvE`, `G`, and `M`. This package uses the paper
for published process context and the CC0 libsnobal source for equation-level
port details.

## No-Fitting Statement

No Stage 0 equation, constant, threshold, or tolerance was fitted to
SNOTEL/cancov, snow-depth, frost, or water-temperature fixtures. Stage 0 is a
pure numerics foundation; later production candidates must still pass their own
contract and observed-data gates before activation.

Short form: no fixture fitting.
