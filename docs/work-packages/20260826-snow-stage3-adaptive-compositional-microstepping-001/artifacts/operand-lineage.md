# Operand lineage

Static: pre-production conservation map.

| Operand | Units/basis | Authority | Status |
|---|---|---|---|
| `I0`, `I1`, `D`, `S`, `melt`, `refreeze` | kg m^-2 OFE-ground | owner directive / SnowEnergy successor | authoritative |
| `L0`, `Lin`, `L1` | kg m^-2 OFE-ground | owner directive / SurfaceLiquid custody | authoritative |
| `C0`, `C1`, `Q`, `U` | J m^-2 OFE-ground | complete bounded carrier / owner directive | authoritative |
| raw vapor mass/latent energy | kg m^-2; J m^-2 | shared carrier opportunity | diagnostic input |
| actual vapor mass/latent energy | kg m^-2; J m^-2 | bounded vapor custody | authoritative |
| trial comparison values | owning field units | complete prognostic owner states | computational diagnostic |

Rejected formulas: melt-before-deposition ordering; raw latent retained after
sublimation truncation; vapor-as-liquid; duplicate latent credit; deletion of
positive residual ice; interpolation/proration of evaluated carrier results.
