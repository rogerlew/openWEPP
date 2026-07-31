# Contract-Test Evidence

Status: `PASS`

Evidence mode: `Ran`

`tests/integration/snow_surface_eb03_contract.rs` now binds both contract
versions and invariants and independently checks boundary-crossing active
mass, harmonic conduction sign/cancellation, and the exact mass-to-substep
transitions.

Ran:

```text
cargo nextest run --test snow_surface_eb03_contract
8 passed, 0 skipped
```
