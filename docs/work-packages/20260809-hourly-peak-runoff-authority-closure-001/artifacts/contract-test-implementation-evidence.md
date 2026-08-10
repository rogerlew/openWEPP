# Contract-Test Implementation Evidence

Status: `complete`

Evidence mode: `Static + Ran`

Added `tests/integration/peak_hourly_authority_contract.rs` and the explicit
Cargo test target. It binds the canonical invariant IDs, hourly source and unit
markers, retirement of the old production branch, exactly-once area conversion,
and the real erosion consumer. The terminal guard additionally binds
`SC-SED-001` rev63 to the internal maximum-hour `m/s` peak, public-only area
conversion, no-fallback posture, and seconds-dimensional `TOL-SED-009` duration
custody while rejecting reuse of sediment tolerance `TOL-SED-001`.
