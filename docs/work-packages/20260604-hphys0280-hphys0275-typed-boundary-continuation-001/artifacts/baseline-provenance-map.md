# Baseline Provenance Map

Status: completed
Evidence mode: static

Static: HPHYS0280 is a boundary typing/governance package, not a physics migration. No baseline physics equations, constants, or snowmelt behavior were ported or changed. The package preserves existing snow negative raw-melt authority by leaving `snow.hourly.melt_raw_m_{idx4}` scalar/follow-up rather than forcing a non-negative water-depth wrapper.

Ran: not-run; provenance map is static.
