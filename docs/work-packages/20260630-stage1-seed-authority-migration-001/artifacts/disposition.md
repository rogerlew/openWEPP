# Disposition

Result: `EXECUTED-HOLD-STAGE1C-TYPED-LANE-SEED-AUTHORITY-MISSING`.

Stage 1B executed and verified. The direct production runoff publication
geometry now seeds `efflen_m` from typed topology geometry instead of reading it
from a `HillslopeWritebackSurface` seed authority. H2637 output identity held,
the focused multi-OFE/Wave-2 fixture passed, and the seed-read inventory moved
from `208` to `207`.

The package stops before Stage 1C. The next surface is the day-zero constructor
seeding path, and it remains genuinely surface-authoritative. Direct execution
does not yet receive a typed per-lane seed-authority carrier for soil/layers,
evapotranspiration stage state, plant growth state, plant water stress, and
snow/frost initial state. That carrier must also include a typed equivalent of
`seed_wb11_runtime_surface_inputs`, because the day-zero seed surface merges the
first climate day and derives mutable WB11/WB18/WB19 storage state before the
constructor reads it. Proceeding without that carrier would either duplicate
runtime-input projection formulas in runner code or hide remaining symbol-map
authority behind a wrapper.

First actionable follow-on:

1. Build a typed per-lane direct seed-authority object from parsed input-contract
   data and the WB11 day-zero projection during static setup.
2. Include the single-OFE case and the per-OFE lane case currently represented by
   `OfeLanePersistentStateSequence`.
3. Thread that typed authority through `HillslopeClimateExecutionState` into
   `build_direct_production_run_frame` and `DirectProductionDayInputBuilder`.
4. Re-run the Stage 1C identity gate, then resume the `207 -> 0` seed-read
   burn-down.

Stage 2 symbol-map runtime deletion remains blocked because the seed-read count
is not `0`.
