# HPHYS0239 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static

## Contract-Derived Tests

1. `tests/integration/wb11_hydrology_kernel_contract.rs`
   - Added
     `hphys0239_contract_wb11_hydrology_tail_order_requires_wb19_then_wb12_reconciliation`.
   - Asserts canonical hydrology-tail order:
     `Percolation -> ET -> Lateral -> Drainage -> RunoffReconciliation -> StorageReconciliation`.
   - Asserts canonical dependency edges across that tail chain.

2. `crates/openwepp-runner/src/hillslope/mod.rs`
   - Added
     `hphys0239_wb13_hydrology_publication_prefers_flux_surface_over_stale_state_surface`.
   - Encodes stale-state vs flux-conflict probe for WB13 `Q`/`Ep`/`Es`/`Er`
     publication family.

## Measure Mapping

- `MEASURE-HP239-002`: satisfied (ordering vectors).
- `MEASURE-HP239-003`: satisfied (WB13 `Q`/`Ep`/`Es`/`Er` conflict probe
  covers flux-authoritative publication after production flux-authority edit).
