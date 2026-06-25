# Pre-Implementation Evidence

Status: executed-hold

Evidence mode: Static + Ran.

Static: `/workdir/pysnobal/pysnobal/defaults.py` defines the required custom
forcing columns used by the exporter: `net_solar_Wm-2`,
`downwelling_thermal_Wm-2`, `temp_air_degC`, `temp_ground_degC`,
`vapor_pressure_Pa`, `wind_speed_ms-1`, `precip_mass_mm`,
`precip_temp_degC`, `snow_precip_fraction`, and
`snow_precip_density_kgm-3`.

Static: `/workdir/pysnobal/pysnobal/c_snobal/libsnobal/vars.c` and
`/workdir/pysnobal/pysnobal/c_snobal/libsnobal/g_soil.c` treat `T_g` as the
soil-temperature boundary at `z_g`, not as snow-surface temperature. G0 uses
constant lanes at `z_g = 0.10 m`: `-2.5 degC`, `-0.5 degC`, and `0.0 degC`.

Static: openWEPP already exposes SIMIMPL28 typed hourly winter forcing through
`DirectWinterHourlyForcing`. The package added only a diagnostic complete-row
wrapper so warm/no-snow days remain serially present for external PySnobal
input export.

Static: snowfall is exported as water-equivalent precipitation mass by
`snowfall_depth_m * snow.options.newsnw`. WAT `Snow-Water`, WAT
`Snow-Depth`, `frost.hourly.surface_temp_c_####`, `surtmp(hour)`, and raw
daily langleys/day radiation are rejected aliases in `lineage.json` and the
focused bridge contract.

Ran: local PySnobal imported through `/tmp/pysnobal-g0-venv/bin/python` with
`PYTHONPATH=/workdir/pysnobal`, `numpy 1.26.4`, and `pandas 2.2.0`.
PySnobal packaging itself was not installed as a normal dependency because its
declared NumPy pin is incompatible with Python 3.12; the harness uses the
checked-out source path explicitly.

Ran: `rg -n "qwet|Qwet|frzftp" crates || true` returned no matches after the
G0 implementation.
