# SNOWDENSITY-10.3.19 Harder-Pomeroy Default Activation

Evidence mode: Static/Ran direct-production activation gate.

- Disposition: `ACTIVATED`
- Cross-SNOTEL gate pass: `True`
- Selector trace gate pass: `True`
- Partition conservation gate pass: `True`
- Prior robust fails / score: `17` / `172`
- New default robust fails / score: `15` / `179`
- Better / worse robust cells vs prior: `9` / `2`
- Humid-New-England depth blocker: `False`
- Density bias note: `23.6234 kg m^-3`

## Model Summary

| Model | Robust fail | Robust score | SWE median bias | Depth median bias | Density median bias |
|---|---:|---:|---:|---:|---:|
| `activated_bundle` | 17 | 172 | -0.268769 | -0.327801 | 0.288915 |
| `harder_pomeroy_default` | 15 | 179 | -0.219035 | -0.294542 | 23.6234 |

## Trace And Conservation

| Model | Expected phase | Rows | Precip rows | Max partition residual m | Selector ok | Conservation ok |
|---|---|---:|---:|---:|---|---|
| `activated_bundle` | `legacy_rst` | 159986 | 53711 | 5.55112e-17 | `True` | `True` |
| `harder_pomeroy_default` | `harder_pomeroy_hourly` | 159986 | 53711 | 5.55112e-17 | `True` | `True` |

## Release Notes

- Humid-New-England depth regression remains a non-representative roadmap item.
- Cross-SNOTEL density bias rises to about `+23.6 kg m^-3`; recovery is tracked separately.
- No `.run` disable option, fixture, public schema, density-cap, or frost change is authorized.
