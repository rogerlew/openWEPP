# Pre Implementation Contract Gate

Status: completed
Evidence mode: ran

Static: red gate was run after contract/test edits and before production implementation.

Ran:
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract -- --nocapture`
- Result: failed as expected before production edits with `error[E0599]: no variant or associated item named direction_degrees found for enum BoundaryValue`.
