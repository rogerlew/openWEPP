# FROST STEP 3 Residue Parameterization Diagnostic

Evidence mode: Ran.

- Decision branch: `C` Dec_* does not drive seasonal residue_depth_m
- Justification: Dec_* residue_depth_m is flat or does not show the required autumn-to-spring decline.
- GAP-SNOWFREEZE-002: GAP-SNOWFREEZE-002 remains open; cropland Dec_* management did not prove a physically seasonal residue_depth_m path to the frost solver, so first-class forest litter cover should be promoted before fixture repointing.
- Step 2 analyzer: `docs/work-packages/20260629-frost-step2-sleepers-attribution-001/artifacts/attribute_sleepers.py`

## Entry Gate

- Fixture: `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh`
- Trace summary: `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/entry_gate_residue_trace_summary.json`
- Monthly trajectory: `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/entry_gate_residue_monthly_trajectory.csv`
- Seasonal: `False`
- Physically reasonable: `False`
- Residue depth min/max m: `0.0230259` / `0.0230259`
- Autumn mean m: `0.0230259`
- Spring mean m: `0.0230259`
- Max monthly mean month: `1`

## Core Test

Not run because the entry gate did not pass.

## Step 4 Note

The Step 1 >0.25 systematic-timing-fraction cutoff is diagnostic-script-local; only TOLERANCE_DAYS=14 is inherited by this package.
