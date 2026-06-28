# Contract Gate

Evidence mode: Static.

## Pre-Code Contract Amendment

PASS. `SC-SNOWFREEZE-001` was amended before production selector edits:

- `contract_version: 101`
- `REF-SNOWFREEZE-SNOWDENSITY1015`
- `INV-SNOWFREEZE-072`
- `OBL-SNOWFREEZE-P-047`
- `SNOWDENSITY-10.3.15 Default Activation Under Active Cap Addendum`
- boundary disposition row for default activation under active cap
- revision history entry for v101

## Authority Summary

The v101 amendment activates only the already-adjudicated active-cap bundle:

- no-env `snow_melt_model = coe_liquid_holding_capacity_v1`
- no-env `snow_density_model = physics_bulk_density_compaction_v1`
- explicit rollback/test selectors remain `legacy_coe` and `legacy_wepp`
- active density cap remains `522 kg m^-3`
- unsupported candidates fail closed in the active default selector path

Historical opt-in invariants remain authority for conservation, rollback,
boundary split, no-site-tuning, and fail-closed behavior.
