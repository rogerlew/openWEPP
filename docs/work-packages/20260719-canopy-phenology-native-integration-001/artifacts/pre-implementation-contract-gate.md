# Pre-Implementation Contract Gate

Evidence mode: `Ran`

Status: `expected red`

Corrected-intake command, run after intent plan `f307138c...` and before
production implementation:

```text
cargo test -p openwepp-plant-phenology --test native_canopy_contract
```

Result: `FAIL` as intended. The compiler reported that
`ForestCanopyDailyResult`, `ForestCanopyParameters`, `ForestCanopyState`, and
`realize_forest_canopy` do not exist in `openwepp_plant_phenology`. The new
contract-derived endpoint, first-realization, ledger, strict-`bb`, wrapped
NH-to-SH, and bit-identical cycle tests therefore cannot pass through an old or
prototype implementation.
