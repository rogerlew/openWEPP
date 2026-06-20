# Producer Selection

Status: pre-implementation.

Static: R4N promotes the aggregate ET producer from R4F handoff to direct WB17
compute. The package keeps the legacy R4F handoff API as scaffold/backward
compatibility for focused tests, but the aggregate direct executor and R4B
closure gate must consume final R4N output.

Selected producers:

- `R4N surface ET`: consumes post-R4M layer state and typed WB17 inputs,
  computes residue evaporation, soil evaporation, transpiration demand, stage
  updates, PMET branch handling, and soil-evaporation layer mutation.
- `R4N root uptake`: consumes post-R4O layer state and the R4N surface ET
  shadow, computes SWU potential/actual uptake, mutates layer state, finalizes
  `Ep`, `Ws`, aggregate `ET`, and R4B `evapotranspiration_m`.

Static: WB17 cannot be represented as a single late R4B handoff. `SC-WATBAL-001`
orders surface ET before WB19 and root uptake after WB19, so R4N is one package
with two direct spans.
