# R4B Process Span Contract

Status: complete.
Evidence mode: Static.

Selected span:

```text
DIRECT_R4B_STORAGE_RECONCILIATION_SPAN =
  [StorageReconciliation, ClosureDiagnostics]
```

Required equation:

```text
storage_reconciled_m =
  storage_initial_m
  + precip_input_m
  + snow_coupling_m
  - q_runoff_m
  - evapotranspiration_m
  - deep_seepage_m
  - subsurface_loss_m

closure_residual_m =
  storage_initial_m
  + precip_input_m
  + snow_coupling_m
  - q_runoff_m
  - evapotranspiration_m
  - deep_seepage_m
  - subsurface_loss_m
  - storage_reconciled_m
```

Span requirements:

- `q_runoff_m` is consumed from R4A direct downstream operands.
- `snow_coupling_m` is signed finite `S`.
- Storage and loss magnitudes are finite and nonnegative.
- `storage_reconciled_m` must be finite and nonnegative.
- Closure residual must be finite and within the declared direct tolerance.
- The span mutates direct storage state only and remains shadow-only.

Boundary:

R4B does not authorize publication, output schema changes, scheduler changes,
default activation, compatibility storage/request/writeback access, or upstream
producer migration for ET, deep seepage, subsurface loss, snow, irrigation, or
liquid assembly.

Contract status:

The selected span maps to existing `SC-WATBAL-001` WB12 storage reconciliation
authority and does not require a canonical contract amendment before production
Rust edits.
