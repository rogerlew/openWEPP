# Line-Count Governance

Status: `PASSED`

Evidence class: Ran.

```bash
wc -l \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs \
  tests/integration/paradigm2_stage3_decouple_water_temperature.rs \
  tools/snowfreeze_observed/paradigm2_stage3_decouple_water_temperature.py \
  docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md \
  docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001/package.md \
  docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001/artifacts/verification.md \
  docs/work-packages/README.md \
  docs/planning/snow-frost-fidelity-strategy.md
```

Result:

```text
1764 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
 208 tests/integration/paradigm2_stage3_decouple_water_temperature.rs
 349 tools/snowfreeze_observed/paradigm2_stage3_decouple_water_temperature.py
3143 docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
 123 docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001/package.md
  76 docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001/artifacts/verification.md
2504 docs/work-packages/README.md
1387 docs/planning/snow-frost-fidelity-strategy.md
9554 total
```

The touched hydrology implementation remains under the local `2000`-line
governance threshold.
