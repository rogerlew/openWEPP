# R4A Process Span Contract

Status: complete.
Evidence mode: Static.

Selected direct process span:

```text
DIRECT_R4A_RUNOFF_PARTITION_SPAN =
  [RunoffReconciliation, StorageReconciliation, ClosureDiagnostics]
```

Canonical authority:

- `SC-RUNOFFPART-001#INV-RUNOFFPART-001`
- `SC-RUNOFFPART-001#INV-RUNOFFPART-002`
- `SC-RUNOFFPART-001#INV-RUNOFFPART-009`
- `SC-RUNOFFPART-001#INV-RUNOFFPART-014`
- `SC-RUNOFFPART-001#INV-RUNOFFPART-016`
- `SC-RUNOFFPART-001#INV-RUNOFFPART-027`

The direct span consumes:

- `liquid_input_m`;
- `runon_input_m`;
- `cumulative_infiltration_m`;
- `depression_storage_delta_m`;
- `surface_saturation_runoff_m`.

The direct span computes:

```text
partition_runoff_m =
  liquid_input_m + runon_input_m
  - cumulative_infiltration_m
  - depression_storage_delta_m

q_runoff_m = partition_runoff_m + surface_saturation_runoff_m

closure_residual_m =
  liquid_input_m + runon_input_m + surface_saturation_runoff_m
  - cumulative_infiltration_m
  - depression_storage_delta_m
  - q_runoff_m
```

The span mutates only direct runtime state:

- `DirectRunoffPartitionState`;
- `DirectWaterState::infiltration_m`;
- `DirectWaterState::runoff_m`;
- direct downstream operands;
- direct shadow projection.

The span does not publish outputs, mutate compatibility storage, call scheduler
paths, or claim full WB12/WB14 migration.
