# Contract Implementation Evidence

Status: complete

Evidence mode: Static

`SC-SNOWFREEZE-001` was amended before production implementation from v124 to
v125. The amendment adds:

- `REF-SNOWFREEZE-WET-COMPACTION-OPERAND-CLOSURE`, anchored to PySnobal 0.2.3,
  Anderson 1976, the pinned WEPP baseline, and this package's authority intake;
- the `snow_wet_compaction_liquid_input` variable and exact `m water
  equivalent` to `kg m^-2` density-boundary conversion;
- `INV-SNOWFREEZE-092`, `OBL-SNOWFREEZE-P-065`, and
  `TOL-SNOWFREEZE-017`;
- guard, alias, boundary, consumer, and binding-exposure entries; and
- a dedicated operand-authority addendum and revision-history entry.

The bound algorithm is
`sum(max(hourly melt_raw_m, 0)) + rain_retained + rain_released`, evaluated
after active-pack precipitation/melt generation and before runoff disposition.
The private scalar is finite and nonnegative. It reaches the selected bulk or
multilayer density consumer once, where water depth becomes liquid mass using
`1000 kg m^-3`.

The contract explicitly rejects the retired
`snowpack_state_loss + routed_melt` duplicate, routed-only, state-loss-plus-rain,
raw-daily-rain, and retained-store level/change aliases. It preserves melt,
SWE, routing, Stage-3, phase, energy, radiation, canopy, frost, density
coefficients/cap, selectors/defaults, and public schemas.
