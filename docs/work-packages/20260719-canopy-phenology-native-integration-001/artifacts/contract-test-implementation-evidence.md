# Contract-Test Implementation Evidence

Evidence mode: `Ran`

Status: `focused contract gates pass`

- `cargo test -p openwepp-plant-phenology`: 12 unit, 5 canopy-contract, and 1
  restart test passed.
- Native endpoints, invalid authority, daily ledger, phase-flipped SH symmetry,
  and two-year no-drift are executable tests.
- YAML missing-authority and exact PL projection tests passed.
- Typed executor override, same-day litter/no-pending-bucket, source ordering,
  and real native direct-run tests passed.
