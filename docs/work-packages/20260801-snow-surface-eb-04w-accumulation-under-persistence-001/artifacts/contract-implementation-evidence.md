# Contract Implementation Evidence

Evidence mode: **Static + Ran**.

`SC-SNOWFREEZE-001` version 121 binds `INV-SNOWFREEZE-088`,
`OBL-SNOWFREEZE-P-062`, and `TOL-SNOWFREEZE-013`. The runtime implementation:

1. preserves the existing SIMIMPL28 phase result when projecting
   `DirectWinterHourlyForcing` into `DirectSnowHourlyForcing`;
2. carries the authoritative pre-partition active precipitation total plus typed
   rain, physical snowfall depth, snowfall SWE, phase fractions, phase-model
   identity, and optional hydrometeor temperature;
3. returns the exact four legacy CoE formula operands before the unchanged sum
   is capped against available pack water;
4. publishes uncapped melt, the separate cap adjustment, and applied melt;
5. publishes an explicit zero implemented wind-redistribution operand without
   interpreting physical redistribution as zero; and
6. sends the ledger through the real direct-production JSONL formatter under
   schema `openwepp-r7h-direct-production-snow-trace-v3`.

Dry hours require zero phase fractions. Wet hours require complementary phase
fractions, proportional rain/snow amounts, and independently reconstruct the
authoritative active total from rain plus snowfall SWE within `1e-12 m`.
The diagnostic-applicability field follows `active_snow_coupling`; inactive
rows remain explicit rather than masquerading as active zero-process results.

No public WAT/PASS/HBP schema, physics selector, coefficient, process equation,
cap, state mutation, observation, or default changed.
