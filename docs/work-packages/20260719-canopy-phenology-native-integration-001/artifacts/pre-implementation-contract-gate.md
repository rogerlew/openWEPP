# Pre-Implementation Contract Gate

Evidence mode: `Ran`

Command:

```text
cargo test -p openwepp-plant-phenology --test native_canopy_contract
```

Expected pre-implementation result: `FAIL` (exit 101).

The contract-derived test compiled against the pre-CP-GSI02 production crate
and failed only because `ForestCanopyParameters` and
`realize_forest_canopy` did not yet exist. This is the intended red gate that
freezes the public realization surface before implementation.

Evidence mode: `not-run`

Status: `queued`
