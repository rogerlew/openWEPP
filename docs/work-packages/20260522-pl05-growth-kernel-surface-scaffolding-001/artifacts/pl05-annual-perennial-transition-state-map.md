# PL05 Annual/Perennial Transition State Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Annual/perennial placeholder transitions are encoded with explicit required symbol guards.
- Ordering flags are treated as required runtime contracts and must equal `1.0`.

Ran:
- Growth boundary failure tests executed and passed.

## Transition Inputs

### Common Growth Ordering Inputs

- `pl_order_growth_after_decomp = 1.0`
- `pl_order_watbal_after_growth = 1.0`

### AnnualGrowthTransition Required Inputs

- `pl_growth_slot_0001_crop_0001_imngmt` in `{1, 3}`
- `pl_growth_slot_0001_crop_0001_jdharv`
- `pl_growth_slot_0001_crop_0001_jdplt`
- `pl_growth_slot_0001_crop_0001_rw`
- `pl_decomp_slot_0001_crop_0001_resmgt`

Output request metadata:
- `phase_class = GrowthAnnualTransition`
- `growth_context.management_class = AnnualOrFallow`

### PerennialGrowthTransition Required Inputs

- `pl_growth_slot_0001_crop_0001_imngmt = 2`
- `pl_growth_slot_0001_crop_0001_jdharv`
- `pl_growth_slot_0001_crop_0001_jdplt`
- `pl_growth_slot_0001_crop_0001_rw`
- `pl_growth_slot_0001_crop_0001_jdstop`
- `pl_growth_slot_0001_crop_0001_mgtopt`

Output request metadata:
- `phase_class = GrowthPerennialTransition`
- `growth_context.management_class = Perennial`

## Transition Failure Surface

- `HS-GROWTH-E-001` missing required symbol
- `HS-GROWTH-E-002` non-finite required value
- `HS-GROWTH-E-003` ordering flag mismatch
- `HS-GROWTH-E-004` unsupported management class domain

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:30`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:386`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:461`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:507`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:1714`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:1764`
