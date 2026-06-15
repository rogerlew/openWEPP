# CQR16 Behavior Equivalence

Status: complete.

Static: production behavior change is limited to routing
`BoundaryUnitRegistryError::fmt` through private helper functions. The helpers
emit the same strings as the previous single match body.

Ran:

```text
cargo test --test sim_contract_boundary_unit_registry cqr16 -- --nocapture
```

Result: pass, `6 passed; 0 failed; 15 filtered out`.

Static: characterization tests cover every `BoundaryUnitRegistryError` variant
and every `OutputUnitRegistryError` variant with exact expected display
strings.

Static: registry row construction, alias lookup, template matching, unit labels,
publication aliases, and scalar exception semantics were not edited.
