# R4D Process Span Contract

Status: complete.
Evidence mode: Static.

Selected span:

```text
DIRECT_R4D_DEEP_SEEPAGE_SPAN =
  [PercolationDeepSeepage, StorageReconciliation]
```

Required producer:

```text
deep_seepage_m = direct percolation deep-seepage handoff D
```

Mutation target:

```text
storage_reconciliation_inputs.deep_seepage_m = deep_seepage_m
```

Span requirements:

- `deep_seepage_m` must be finite and nonnegative.
- `deep_seepage_m` must come from the direct deep-seepage handoff operand, not
  public `Dp`, WB19 `Qd`, ET, snow, precipitation, R3B diagnostic ledger values,
  or storage residual compensation.
- The span mutates direct deep-seepage state and the R4B deep-seepage input only
  and remains shadow-only.

Boundary:

R4D does not authorize publication, output schema changes, scheduler changes,
default activation, compatibility storage/request/writeback access, full WB18
percolation migration, WB19 lateral/drainage migration, ET migration, or
snow-coupling migration.

Contract status:

The selected span maps to existing `SC-PERC-001` below-root-zone `D` authority
and `SC-WATBAL-001` WB12 storage-reconciliation authority. It does not require a
canonical contract amendment before production Rust edits.
