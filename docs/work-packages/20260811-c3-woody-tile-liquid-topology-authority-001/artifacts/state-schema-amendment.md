# State Schema Amendment

Status: `selected`

Evidence mode: `Static`

Shared stratum state retains the six tissue C/N identities, storage/transfer
pools, NSC, maintenance reserve, phenology, turnover, mortality, LAI/WAI, root
profile, and material transfers.

Every occupancy owns canopy liquid and the exact numerical state frozen in the
V2 model definition: sun/shade leaf temperature and `ci`, dry-stem and
wet-surface temperature, canopy-air temperature and specific humidity,
`beta_hyd`, stem/sun-leaf/shade-leaf potentials, the layer-ID-sorted root
potential vector, and an optional last-accepted transaction ID. Each scalar has
an explicit finite/domain rule; root cardinality equals configured root-layer
cardinality; initial identity is null and accepted identity must be the
immediately preceding transaction.

Warm starts affect initialization only; alternative valid starts must converge
to the same accepted physical state. Missing, duplicate, extra, stale,
wrong-unit, wrong-cardinality, or non-occupancy lanes fail before calculation.
Recursively lexicographically sorted keys, layer-ID order, and shortest
round-trip numbers define deterministic serialization, and every field enters
the state digest. All hydraulic potentials use `mm H2O`; an MPa-tagged lane is
rejected as a wrong-unit poison without conversion.
