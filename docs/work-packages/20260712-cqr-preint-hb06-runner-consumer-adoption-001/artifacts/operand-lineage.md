# WB13 Operand And Consumer Lineage

Evidence class: **Static + Ran**

The canonical direct projection assigns `soil_water_total_m = total_soil_m`.
The direct-publication boundary converts both to millimetres and rejects an
alias residual above `1e-6 mm`. The common runner WAT adapter then:

1. constructs the pre-existing `HillslopeWatRow` projection;
2. requires all four canonical profile operands;
3. builds `Wb13DailyWaterBalanceInput` with emitted simulation-year keys;
4. applies finite, range, `Q == QOFE`, storage-alias, and profile-order guards;
5. bit-compares the three keys and all 22 canonical scalars; and
6. reconstructs canonical WAT fields from the validated row, retaining only
   additive runner fields from the original projection.

No text render/parse, string-keyed map, schema change, buffering change, or
numeric regrouping is used. Deliberately malformed `QOFE`, storage aliases,
missing profiles, and partial profiles fail closed.

The real consumer proof is the SIMIMPL04 executable run: production execution
passes through the common adapter and streaming Parquet sink, after which the
integration test reads `H5.wat.parquet` and verifies exact schema ordering,
nullability, row count/order, simulation-year keys, and representative
bit-identical canonical values.

