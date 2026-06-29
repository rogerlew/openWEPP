# Execution Evidence

Evidence mode: Ran.

## Inputs

Consumed existing Step 1 artifacts:

- `docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001/artifacts/current_snow_control_routing.json`
- `docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001/artifacts/site_reports/site1_sleepers_south_field_vt.comparison_report.json`
- `docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001/artifacts/site_reports/site2_sleepers_w9_hardwood_vt.comparison_report.json`

The script reconstructed full frost-depth residual distributions from the WAT
paths recorded in those reports:

- `target/frost_step1_current_snow_control/site1/site1_sleepers_south_field_vt.wat.parquet`
- `target/frost_step1_current_snow_control/site2/site2_sleepers_w9_hardwood_vt.wat.parquet`

No harness rerun was needed.

## Command

```bash
.venv/bin/python \
  docs/work-packages/20260629-frost-step2-sleepers-attribution-001/artifacts/attribute_sleepers.py \
  --output-json docs/work-packages/20260629-frost-step2-sleepers-attribution-001/artifacts/sleepers_attribution.json \
  --output-md docs/work-packages/20260629-frost-step2-sleepers-attribution-001/artifacts/sleepers_attribution.md
```

Result: pass.

## Output

- `artifacts/sleepers_attribution.json`
- `artifacts/sleepers_attribution.md`

Scope guard: the analyzer hard-fails unless the input sites are exactly the two
Step 1 `FORCING-LIMITED` Sleepers sites.
