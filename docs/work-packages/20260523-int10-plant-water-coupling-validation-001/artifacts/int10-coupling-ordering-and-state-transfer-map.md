# INT10 Coupling Ordering and State-Transfer Map

Status: `complete`
Evidence mode: `Static + Ran`

## Canonical Lane Ordering

INT10 coupled daily lane ordering authority:

1. `decomposition_transition`
2. `residue_partition_transition`
3. `annual_growth_transition` / `perennial_growth_transition`
4. watbal/hydrology lane:
   - `evapotranspiration`
   - `percolation_deep_seepage`
   - `lateral_transfer`
   - `drainage`
   - `runoff_reconciliation`
   - `storage_reconciliation`

## Coupled State-Transfer Map

| Producer phase | Surface/symbol | Consumer phase | Guard/failure posture |
|---|---|---|---|
| `decomposition_transition` | typed decomposition context (`order_decomp_before_soil`, `order_growth_after_decomp`, transition payload) | `residue_partition_transition` and downstream growth lane | typed decomposition boundary guards (`HS-DECOMP-E-001..010`) |
| `annual_growth_transition` | typed growth context (`order_growth_after_decomp`, `order_watbal_after_growth`, transition payload) | watbal/hydrology lane entry precondition | typed growth boundary guards (`HS-GROWTH-E-001..007`) |
| transition writeback | state marker written in decomposition phase (`int10_decomp_marker`) | growth and watbal phases | hard fail by test contract if marker is absent |
| transition writeback | state marker written in growth phase (`int10_growth_marker`) | watbal phases | hard fail by test contract if marker is absent |

## INT10 Guard Surface (Contract-Test Covered)

- Missing coupled ordering symbol (`pl_order_watbal_after_growth`) produces
  typed failure at annual growth transition: `HS-GROWTH-E-001`.
- Non-finite coupled ordering symbol (`pl_order_growth_after_decomp = NaN`)
  produces typed failure at decomposition transition: `HS-DECOMP-E-002`.
- No silent fallback/clamp/default path is allowed for ordering violations or
  non-finite coupled symbols.
