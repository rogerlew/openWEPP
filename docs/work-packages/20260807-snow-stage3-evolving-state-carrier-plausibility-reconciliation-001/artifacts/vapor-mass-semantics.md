# Vapor Mass Semantics Freeze

Static: frozen before new attribution execution.

`vapor_mass_exchange_kg_m2` is signed opportunity: deposition is positive and
sublimation negative. For immutable same-state `S` and frozen-active `F`, actual
transfer and debit are `NOT_APPLICABLE`; their state is intentionally not
mutated. For sequential `Q`, independently reconstruct actual vapor transfer
from raw vapor and pre-transfer active ice without reading the producer transfer
columns: `deposition=max(raw,0)` and
`sublimation=min(max(-raw,0),active_ice_before)`. Compare those values to the
producer `deposition_kg_m2` and `sublimation_kg_m2`, then reconcile them to
total-ice endpoints with the separately energy-checked melt operand:

`total_after = total_before - melt - sublimation + deposition`.

The analyzer must reject raw-as-bounded, vapor-as-liquid, sublimation-as-melt,
and separately reduced median aliases. The package reconstructs bounded
within-day Q transfer and endpoint change over independently reinitialized
daily operators, then aggregates those diagnostic opportunities by window. It
does not call that aggregate a physical seasonal trajectory or persistent loss.
