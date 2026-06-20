# R3B Span Contract

Status: complete.
Evidence mode: Static.

Selected direct span:

```text
DIRECT_R3B_WATER_LEDGER_SPAN =
  [RunoffReconciliation, StorageReconciliation, ClosureDiagnostics]
```

The span consumes:

- R3A `DirectInputAccountingState`;
- `DirectWaterState`;
- `DirectPublicationFrame`.

The span computes:

- `available_water_m = input_accounting.total_accounted_input_m + water.soil_water_m`;
- `known_outflow_m = publication.runoff_m + publication.evapotranspiration_m
  + publication.drainage_m + publication.lateral_flow_m`;
- `retained_water_m = water.soil_water_m + water.infiltration_m`;
- `diagnostic_residual_m = available_water_m - known_outflow_m - retained_water_m`.

The residual is diagnostic only. R3B does not assert closure, correct process
magnitudes, or publication meaning. Negative residuals are valid if finite.

The span mutates direct ledger state, produces downstream ledger operands, and
shadow-projects the signed ledger result. It must not call compatibility
storage/request/writeback APIs.

