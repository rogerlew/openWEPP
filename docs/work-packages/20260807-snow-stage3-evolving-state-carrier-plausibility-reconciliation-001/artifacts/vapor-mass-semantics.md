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

## Executed Result

Ran: every eligible Q tuple independently matched producer bounded transfer,
melt, and total-mass endpoints. Capacity-truncated tuple count is `0` at all
four sites, so `VAPOR_OPPORTUNITY_TRANSFER_MISMATCH` is not emitted. Median raw
latent-minus-bounded latent energy is numerical zero at each site (absolute
site median no larger than `2.22e-10 J m^-2`).

Site medians of independently aggregated Q raw opportunity / bounded
sublimation / bounded deposition, in `kg m^-2`, are:

| Site | Raw opportunity | Bounded sublimation | Bounded deposition |
| --- | ---: | ---: | ---: |
| Mica Creek | `-90.5711` | `92.9131` | `3.09376` |
| Niwot | `-208.822` | `210.373` | `1.40382` |
| Paradise | `-125.047` | `142.426` | `13.5063` |
| Snowbird | `-114.310` | `117.162` | `2.50301` |

These columns are separately reduced distributions and are not an additive
median ledger. They characterize independently reinitialized daily Q windows,
not persistent seasonal mass loss.
