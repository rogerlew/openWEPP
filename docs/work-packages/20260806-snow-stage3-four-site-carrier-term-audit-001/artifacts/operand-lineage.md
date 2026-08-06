# Operand Lineage

Status: `frozen before result execution`

| Quantity | Unit | Exact consumer operand | Rejected aliases |
| --- | --- | --- | --- |
| shortwave | hourly/daily `J m^-2` | schema-v5 evaluation hourly/daily shortwave | CoE `A`, incident climate radiation, surface total |
| longwave | hourly/daily `J m^-2` | schema-v5 evaluation hourly/daily longwave | CoE `B`, incoming-only longwave |
| sensible | hourly/daily `J m^-2` | exported complete-arm sensible flux times exact `3600 s` | CoE `C`, air temperature, net turbulent |
| latent | hourly/daily `J m^-2` | exported complete-arm latent flux times exact `3600 s` | vapor mass without latent conversion, CoE `C` |
| precipitation advection | hourly/daily `J m^-2` | exported complete-arm advected flux times exact `3600 s` | rainfall depth, CoE `D` |
| external complete total | `J m^-2` | independent sum of the five terms above | producer total alone, residual, surface arm |
| surface arm | `J m^-2` | independent shortwave + longwave + latent | complete total |
| internal active/lower conduction | `J m^-2` | applicability flag plus exact zero for the same-state pair | snow-ground heat, external energy source/sink |
| support | `s` | hourly requested/evaluated arrays and evaluated boolean | row count, calendar duration |
| pair identity | IDs/hash | exact tag fields and equal non-formulation fingerprints | matching labels without hashes |

The absent snow-ground boundary is reported as `NOT_IMPLEMENTED`, not as a
zero-valued operand. Producer totals and residuals are checked against these
operands but cannot substitute for reconstruction.
