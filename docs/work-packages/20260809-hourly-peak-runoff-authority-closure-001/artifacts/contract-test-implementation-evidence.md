# Contract-Test Implementation Evidence

Status: `complete`

Evidence mode: `Static + Ran`

Added `tests/integration/peak_hourly_authority_contract.rs` and the explicit
Cargo test target. It binds the canonical invariant IDs, hourly source and unit
markers, retirement of the old production branch, exactly-once area conversion,
and the real erosion consumer.
