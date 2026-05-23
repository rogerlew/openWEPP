# INT10 Coupled Replay Ordering and State-Trace Evidence

Status: `complete`
Evidence mode: `Ran`

## Command

```bash
cargo test --test int10_plant_water_coupling_validation_contract -- --nocapture
```

## Coupled Replay Ordering Evidence

Observed pass of:
- `int10_contract_conformance_validates_coupled_replay_ordering_and_state_transfer`

Validated scheduler execution order includes canonical coupled path:
- decomposition lane (`decomposition_transition`, `residue_partition_transition`)
- growth lane (`annual_growth_transition` / `perennial_growth_transition`)
- watbal lane (`evapotranspiration`, `percolation_deep_seepage`,
  `lateral_transfer`, `drainage`, `runoff_reconciliation`,
  `storage_reconciliation`)

## State-Trace Evidence

- Decomposition transition writes state marker: `int10_decomp_marker = 10.0`.
- Annual growth transition observes decomposition marker and writes:
  `int10_growth_marker = 20.0`.
- Watbal phases assert visibility of both markers, proving coupled
  writeback-surface transfer continuity from plant lanes into hydrology lanes.

## Typed Failure Trace Evidence

Observed pass of typed-failure vectors:
- Missing symbol `pl_order_watbal_after_growth` -> halts at
  `AnnualGrowthTransition` with `HS-GROWTH-E-001`.
- Non-finite value `pl_order_growth_after_decomp = NaN` -> halts at
  `DecompositionTransition` with `HS-DECOMP-E-002`.
