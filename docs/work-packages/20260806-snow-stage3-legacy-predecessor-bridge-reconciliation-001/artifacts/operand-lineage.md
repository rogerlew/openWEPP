# Operand Lineage

Status: `prospectively frozen / exact field adapter pending implementation`.

Evidence class: `Static`.

| Operand | Canonical meaning | Units/basis | Reduction | Rejected alias |
| --- | --- | --- | --- | --- |
| external complete carrier | absorbed shortwave + net longwave + sensible + latent + precipitation-advected heat | J m^-2 per evaluated substep | ordered substep sum -> day -> WY | CoE melt energy; daily aggregate without support proof |
| active internal conduction | energy transferred into/out of the active layer | J m^-2 per evaluated substep | included once in active-layer energy; paired lower-layer transfer must cancel internally | external carrier; lower-interface flux counted twice |
| complete active energy | external carrier + active internal conduction | J m^-2 per evaluated substep | ordered substep sum -> day -> WY | surface carrier alone |
| cold-content response | positive energy consumed warming resolved ice to the fusion boundary | J m^-2 | state-transition reconstruction | latent fusion energy |
| fusion response | positive energy consumed by ice melt after cold content | J m^-2 | state-transition reconstruction | CoE melt or snowfall loss |
| terminal unallocated energy | residual only after resolved ice is exhausted | J m^-2 | explicit diagnostic sum | unused-positive-energy legacy aggregate |
| support | evaluated sequential substep with resolved pre-state | boolean/count | exact ordered membership and duration | day-level `stage3_evaluated` alone |

Every primitive is diagnostic/evaluation-only; CoE state remains authoritative.
The consumer adapts schema v4 daily aggregates and schema v6 ordered tuples to
the same explicit units, duration, area basis, support rule, daily dates,
WY1990--2024 windows, and Python `statistics.median` order. Exact adapter field
names, signs, tolerances, and fixture hashes are frozen in
`protocol-freeze.json` before execution and tested with adversarial aliases.
