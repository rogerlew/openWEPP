# R4C Process Span Contract

Status: complete.
Evidence mode: Static.

Selected span:

```text
DIRECT_R4C_STORAGE_INPUT_SPAN =
  [Normalization, StorageReconciliation]
```

Required producer:

```text
storage_initial_m = direct water.soil_water_m
precip_input_m = R3A direct downstream precipitation_m
```

Mutation target:

```text
storage_reconciliation_inputs.storage_initial_m = storage_initial_m
storage_reconciliation_inputs.precip_input_m = precip_input_m
```

Span requirements:

- R3A input accounting must execute first.
- `storage_initial_m` must be finite and nonnegative.
- `precip_input_m` must be finite and nonnegative.
- `storage_initial_m` must come from direct storage state, not publication or
  diagnostic ledger state.
- `precip_input_m` must come from R3A direct precipitation, not
  total-accounted input, transfer input, R4A runoff input, publication fields,
  or R3B diagnostic ledger values.
- The span mutates direct storage-input state only and remains shadow-only.

Boundary:

R4C does not authorize publication, output schema changes, scheduler changes,
default activation, compatibility storage/request/writeback access, or producer
migration for `S`, ET, `D`, `Qd`, snow, irrigation, or liquid assembly.

Contract status:

The selected span maps to existing `SC-WATBAL-001` WB12 storage reconciliation
authority and does not require a canonical contract amendment before production
Rust edits.
