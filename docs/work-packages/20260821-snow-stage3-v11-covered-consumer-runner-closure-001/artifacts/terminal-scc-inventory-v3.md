# Terminal executable SCC inventory v3

Ran: package-local inventory validator and Tarjan SCC calculation.

- pass: `True`
- exact ordered components: `12`
- SCCs: `[["carrier.air_temperature", "carrier.specific_humidity", "hydrology.water", "lse.surface_temperature", "snow.enthalpy", "snow.ice", "snow.liquid", "soil.enthalpy", "surface_liquid.storage", "vegetation.liquid", "vegetation.temperature"], ["bgc.transition"]]`
- follower: `bgc.transition` (no feedback edge)
