# Turbulent Input Authority

Static: user authority dated 2026-08-05 and pinned libsnobal commit
`bf8b41c71e3e54ae654ae04005ddf72566c47ee6`.

The CLIGEN/openWEPP forcing projection represents virtual instruments relative
to the instantaneous modeled snow surface:

| Typed input | Definition | Value |
| --- | --- | ---: |
| `air_temperature_height` (`z_T`) | effective temperature height | `5 m` |
| `vapor_pressure_height` (`z_q`) | effective humidity height | `5 m` |
| `wind_speed_height` (`z_u`) | effective wind height | `5 m` |
| `roughness_length` (`z_0,aero`) | exposed-snow aerodynamic roughness | `0.005 m` |

Pinned provenance:

- `pysnobal/ipysnobal.py` defines `z_u=5.0` and `z_t=5.0`.
- `test_data_point/inheight.input` records `0 5 5 0.005 0.1` for time,
  wind height, thermodynamic height, roughness, and ground-measurement depth.
- libsnobal uses the thermodynamic height for temperature and vapor exchange;
  openWEPP exposes separate typed `z_T` and `z_q` fields with equal admitted
  values rather than collapsing their meanings.

`z_0,aero` is not `SC-SNOWENERGY-001` active thermal-layer depth `z_0`.
None of these values may be fitted to Snowbird or other evaluation results.
